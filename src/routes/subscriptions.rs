use actix_web::{HttpResponse, Responder, ResponseError, http::StatusCode, web};
use anyhow::Context;
use chrono::Utc;
use rand::{RngExt, distr::Alphanumeric};
use serde::Deserialize;
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{
    domain::{common::SuccessResponse, subscriber::NewSubscriber},
    email_client::EmailClient,
    startup::{ApplicationBaseUrl, ApplicationPort},
};

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[derive(thiserror::Error)]
pub enum SubscribeError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
impl From<String> for SubscribeError {
    fn from(value: String) -> Self {
        Self::ValidationError(value)
    }
}
impl ResponseError for SubscribeError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            SubscribeError::ValidationError(_) => StatusCode::BAD_REQUEST,
            SubscribeError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let message_string = match self {
            SubscribeError::ValidationError(_) => "Form has invalid values.".to_owned(),
            SubscribeError::UnexpectedError(_) => {
                "A technical error occured. Please try again later.".to_owned()
            }
        };

        HttpResponse::build(self.status_code()).json(SuccessResponse {
            success: false,
            message: message_string,
        })
    }
}
impl std::fmt::Debug for SubscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

fn generate_subscription_token() -> String {
    let mut rng = rand::rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(32)
        .collect()
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip_all,
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Json<FormData>,
    db: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>,
    port: web::Data<ApplicationPort>,
) -> Result<impl Responder, SubscribeError> {
    let new_subscriber = form.0.try_into()?;

    let mut transaction = db
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    let subscriber_id = insert_subscriber(&new_subscriber, &mut transaction)
        .await
        .context("Failed to insert new subscriber in the database.")?;

    let token = generate_subscription_token();

    insert_subscriber_confirmation(&subscriber_id, &token, &mut transaction)
        .await
        .context("Failed to store the confirmation token for a new subscriber.")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;

    let mut port_str = String::new();
    if port.0 != 80 && port.0 != 0 {
        port_str.push_str(&format!(":{}", port.0));
    };
    send_confirmation_email(
        &email_client,
        new_subscriber,
        &base_url.0,
        &port_str,
        &token,
    )
    .await
    .context("Failed to send confirmation email for subscription.")?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        success: true,
        message: "Thank you for joining our newsletter!".to_owned(),
    }))
}

#[tracing::instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(email_client, new_subscriber)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: NewSubscriber,
    base_url: &str,
    port: &str,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link =
        format!("{base_url}{port}/subscriptions/confirm?subscription_token={subscription_token}");

    let html_body = format!(
        "Welcome to our newsletter! Click {} to confirm your subscription.",
        confirmation_link
    );

    email_client
        .send_email(new_subscriber.email, "Welcome!", &html_body)
        .await
}

#[tracing::instrument(name = "Saving new subscriber details to the database", skip_all)]
pub async fn insert_subscriber(
    new_subscriber: &NewSubscriber,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<Uuid, sqlx::Error> {
    let subscriber_id = Uuid::new_v4();
    let query = sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at, status)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        subscriber_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now(),
        "pending_confirmation"
    );

    transaction.execute(query).await?;

    Ok(subscriber_id)
}

pub struct StoreTokenError(sqlx::Error);
impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
impl std::fmt::Debug for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
impl std::fmt::Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error has occured while \
            trying to store the subscription token."
        )
    }
}

#[tracing::instrument(name = "Creating new subscription confirmation record", skip_all)]
pub async fn insert_subscriber_confirmation(
    subscriber_id: &Uuid,
    token: &str,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<(), StoreTokenError> {
    let query = sqlx::query!(
        r#"
            INSERT INTO subscription_tokens (subscription_token, subscriber_id)
            VALUES ($1, $2)
        "#,
        token,
        subscriber_id,
    );

    transaction
        .execute(query)
        .await
        .map_err(|e| StoreTokenError(e))?;

    Ok(())
}
