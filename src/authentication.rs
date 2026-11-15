use actix_web::http::header::HeaderMap;
use anyhow::Context;
use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
    password_hash::phc::SaltString,
};
use base64::{
    Engine, alphabet,
    engine::{GeneralPurpose, GeneralPurposeConfig},
};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{constants::MOCK_PASSWORD_HASH, telemetry::spawn_blocking_with_tracing};

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub struct Credentials {
    pub username: String,
    pub password: SecretString,
}

pub fn basic_authentication(headers: &HeaderMap) -> Result<Credentials, anyhow::Error> {
    let header_value = headers
        .get("Authorization")
        .context("The 'Authorization' header was missing")?
        .to_str()
        .context("The 'Authorization' header was not a valid UTF8 string.")?;
    let base64encoded_segment = header_value
        .strip_prefix("Basic ")
        .context("The authorization scheme was not 'Basic'.")?;

    let base64engine = GeneralPurpose::new(&alphabet::STANDARD, GeneralPurposeConfig::default());
    let decoded_bytes = base64engine
        .decode(base64encoded_segment)
        .context("Failed to base64-decode 'Basic' credentials.")?;
    let decoded_credentials = String::from_utf8(decoded_bytes)
        .context("The decoded credential string is not valid UTF8.")?;

    let mut credentials = decoded_credentials.splitn(2, ':');
    let username = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A username must be provided in 'Basic' auth."))?
        .to_string();
    let password = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A password must be provided in 'Basic' auth."))?
        .to_string();

    Ok(Credentials {
        username,
        password: SecretString::from(password),
    })
}

#[tracing::instrument(name = "Validate credentials", skip_all)]
pub async fn validate_credentials(
    credentials: Credentials,
    db: &PgPool,
) -> Result<uuid::Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash = SecretString::from(MOCK_PASSWORD_HASH);

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&credentials.username, db)
            .await
            .map_err(AuthError::UnexpectedError)?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(|| {
        verify_password_hash(expected_password_hash, credentials.password)
    })
    .await
    .context("Failed to spawn blocking task.")??;

    user_id.ok_or_else(|| AuthError::InvalidCredentials(anyhow::anyhow!("Unknown username.")))
}

#[tracing::instrument(name = "Verify password hash", skip_all)]
pub fn verify_password_hash(
    expected_password_hash: SecretString,
    password_candidate: SecretString,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}

pub async fn get_stored_password(
    user_id: Uuid,
    db: &PgPool,
) -> Result<Option<SecretString>, anyhow::Error> {
    let row = sqlx::query!(
        r#"
            SELECT password_hash
            FROM users
            WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_optional(db)
    .await
    .context("Failed to perform query to fetch password hash.")?
    .map(|row| SecretString::from(row.password_hash));

    Ok(row)
}

async fn get_stored_credentials(
    username: &str,
    db: &PgPool,
) -> Result<Option<(uuid::Uuid, SecretString)>, anyhow::Error> {
    let row = sqlx::query!(
        r#"
            SELECT user_id, password_hash
            FROM users
            WHERE username = $1
        "#,
        username,
    )
    .fetch_optional(db)
    .await
    .context("Failed to perform query to validate credentials.")?
    .map(|row| (row.user_id, SecretString::from(row.password_hash)));

    Ok(row)
}

pub fn compute_password_hash(password: SecretString) -> Result<SecretString, anyhow::Error> {
    let salt = SaltString::generate();
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password_with_salt(password.expose_secret().as_bytes(), &salt.to_salt())
    .map_err(|e| AuthError::UnexpectedError(anyhow::anyhow!(e)))?
    .to_string();

    Ok(SecretString::from(password_hash))
}
