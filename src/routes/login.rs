use actix_session::Session;
use actix_web::{
    HttpResponse, ResponseError,
    http::StatusCode,
    web::{self, Json},
};
use secrecy::SecretString;
use sqlx::PgPool;

use crate::{
    authentication::{AuthError, Credentials, validate_credentials},
    domain::common::SuccessResponse,
    utils::error_chain_fmt,
};

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    username: String,
    password: SecretString,
}

#[tracing::instrument(
    skip_all,
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(
    form: web::Json<LoginFormData>,
    db: web::Data<PgPool>,
    session: Session,
) -> Result<Json<SuccessResponse>, LoginError> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };

    tracing::Span::current().record("username", tracing::field::display(&credentials.username));

    let user_id = validate_credentials(credentials, &db)
        .await
        .map_err(|e| match e {
            AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
            AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
        })?;

    tracing::Span::current().record("user_id", tracing::field::display(&user_id));

    session
        .insert("user_id", user_id)
        .map_err(|e| LoginError::UnexpectedError(anyhow::anyhow!(e)))?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Login successful".to_owned(),
        data: None,
    }))
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for LoginError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            LoginError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LoginError::AuthError(_) => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let res_err = SuccessResponse {
            success: false,
            message: "A technical error occured.".to_owned(),
            data: None,
        };

        let mut response = HttpResponse::build(self.status_code());

        match self {
            LoginError::AuthError(_) => response.json(SuccessResponse {
                message: "Wrong username or password".to_owned(),
                ..res_err
            }),
            LoginError::UnexpectedError(_) => response.json(res_err),
        }
    }
}
