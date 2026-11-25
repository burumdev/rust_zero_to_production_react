use actix_web::{
    HttpResponse, ResponseError,
    http::StatusCode,
    web::{self, Json},
};
use anyhow::Context;
use sqlx::PgPool;

use crate::{
    authentication::get_stored_username,
    constants::TXT_TECHNICAL_ERROR,
    domain::{common::SuccessResponse, subscriber::ValidEmail},
    email_client::EmailClient,
    idempotency::{key::IdempotencyKey, persistence::get_saved_response},
    middleware::UserId,
    utils::error_chain_fmt,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NewsletterBody {
    title: String,
    content: String,
}

struct ConfirmedSubscriber {
    email: ValidEmail,
}

#[derive(thiserror::Error)]
pub enum PublishError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
impl std::fmt::Debug for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
impl ResponseError for PublishError {
    fn status_code(&self) -> StatusCode {
        match self {
            PublishError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PublishError::AuthError(_) => StatusCode::UNAUTHORIZED,
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
            PublishError::UnexpectedError(_) => response.json(res_err),
            PublishError::AuthError(e) => response.json(SuccessResponse {
                message: e.to_string(),
                ..res_err
            }),
        }
    }
}

#[tracing::instrument(
    name = "Publish a newsletter issue",
    skip_all,
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn publish_newsletter(
    body: web::Json<NewsletterBody>,
    db: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    user_id: web::ReqData<UserId>,
    idempotency_key: web::ReqData<IdempotencyKey>,
) -> Result<HttpResponse, PublishError> {
    let user_id = user_id.into_inner();
    if let Some(saved_response) = get_saved_response(&db, &idempotency_key, *user_id)
        .await
        .map_err(PublishError::UnexpectedError)?
    {
        return Ok(saved_response);
    }

    let username = get_stored_username(*user_id, &db)
        .await
        .map_err(PublishError::UnexpectedError)?;

    tracing::Span::current().record("username", tracing::field::display(&username));
    tracing::Span::current().record("user_id", tracing::field::display(*user_id));

    let confirmed_subscribers = get_confirmed_subscribers(&db).await?;
    for subscriber in confirmed_subscribers {
        match subscriber {
            Ok(subscriber) => {
                email_client
                    .send_email(&subscriber.email, &body.title, &body.content)
                    .await
                    .with_context(|| {
                        format!("Failed to send newsletter issue to {}", subscriber.email)
                    })?;
            }
            Err(error) => {
                tracing::warn!(
                    error.cause_chain = ?error,
                    "Skipping a confirmed subscriber. \
                    Their stored contact details are invalid"
                );
            }
        }
    }

    Ok(
        HttpResponse::build(StatusCode::OK).json(Json(SuccessResponse {
            success: true,
            message: "Newsletter published".to_owned(),
            data: None,
        })),
    )
}

#[tracing::instrument(name = "Get confirmed subscribers", skip_all)]
async fn get_confirmed_subscribers(
    db: &PgPool,
) -> anyhow::Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    let rows = sqlx::query!(
        r#"
            SELECT email
            FROM subscriptions
            WHERE status = 'confirmed'
        "#,
    )
    .fetch_all(db)
    .await?;

    let confirmed_subscribers = rows
        .into_iter()
        .map(|r| match ValidEmail::parse(r.email) {
            Ok(email) => Ok(ConfirmedSubscriber { email }),
            Err(error) => Err(anyhow::anyhow!(error)),
        })
        .collect();

    Ok(confirmed_subscribers)
}
