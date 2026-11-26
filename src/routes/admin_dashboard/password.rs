use actix_web::{
    HttpResponse, ResponseError,
    http::StatusCode,
    web::{self, Json},
};
use secrecy::{ExposeSecret, SecretString};
use sqlx::{Executor, PgPool};
use uuid::Uuid;

use crate::{
    authentication::{
        AuthError, Credentials, compute_password_hash, get_stored_username, validate_credentials,
    },
    constants::TXT_TECHNICAL_ERROR,
    domain::common::SuccessResponse,
    middleware::UserId,
    utils::{e500, error_chain_fmt},
};

#[derive(serde::Deserialize)]
pub struct FormDataChangePassword {
    current_password: SecretString,
    new_password: SecretString,
    new_password_check: SecretString,
}

#[derive(thiserror::Error)]
#[allow(clippy::enum_variant_names)]
enum AdminPasswordError {
    #[error("Password mismatch")]
    PasswordMismatchError,
    #[error("Unexpected error")]
    UnexpectedError,
    #[error("Authentication failed")]
    AuthError,
}

impl std::fmt::Debug for AdminPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for AdminPasswordError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AdminPasswordError::PasswordMismatchError => StatusCode::BAD_REQUEST,
            AdminPasswordError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
            AdminPasswordError::AuthError => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let res_err = SuccessResponse {
            success: false,
            message: TXT_TECHNICAL_ERROR.to_owned(),
            data: None,
        };

        let mut response = HttpResponse::build(self.status_code());

        match self {
            AdminPasswordError::PasswordMismatchError => response.json(SuccessResponse {
                message: "Password and its repeat should match".to_owned(),
                ..res_err
            }),
            AdminPasswordError::UnexpectedError => response.json(res_err),
            AdminPasswordError::AuthError => response.json(SuccessResponse {
                message: "Current password could not be verified".to_owned(),
                ..res_err
            }),
        }
    }
}

fn check_passwords_match(p1: &SecretString, p2: &SecretString) -> Result<(), AdminPasswordError> {
    if p1.expose_secret() != p2.expose_secret() {
        return Err(AdminPasswordError::PasswordMismatchError);
    }

    Ok(())
}

pub async fn change_password(
    form: web::Json<FormDataChangePassword>,
    db: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<Json<SuccessResponse>, actix_web::Error> {
    let user_id = user_id.into_inner();
    let username = get_stored_username(*user_id, &db).await.map_err(e500)?;

    let FormDataChangePassword {
        current_password,
        new_password,
        new_password_check,
    } = form.0;

    let credentials = Credentials {
        username,
        password: current_password,
    };

    validate_credentials(credentials, &db)
        .await
        .map_err(|e| match e {
            AuthError::UnexpectedError(_) => AdminPasswordError::UnexpectedError,
            AuthError::InvalidCredentials(_) => AdminPasswordError::AuthError,
        })?;

    check_passwords_match(&new_password, &new_password_check)?;

    // Validated from this point
    let password_hash =
        compute_password_hash(new_password).map_err(|_| AdminPasswordError::UnexpectedError)?;

    update_user_password(*user_id, password_hash, &db)
        .await
        .map_err(|_| AdminPasswordError::UnexpectedError)?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Password changed successfully".to_owned(),
        data: None,
    }))
}

async fn update_user_password(
    user_id: Uuid,
    password_hash: SecretString,
    db: &PgPool,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"
            UPDATE users
            SET password_hash = $1
            WHERE user_id = $2
        "#,
        password_hash.expose_secret(),
        user_id
    );

    db.execute(query).await?;

    Ok(())
}
