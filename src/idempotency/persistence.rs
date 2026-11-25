use actix_web::{HttpResponse, http::StatusCode, web::Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::common::SuccessResponse, idempotency::key::IdempotencyKey};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "header_pair")]
struct HeaderPairRecord {
    name: String,
    value: Vec<u8>,
}

pub async fn get_saved_response(
    db: &PgPool,
    idempotency_key: &IdempotencyKey,
    user_id: Uuid,
) -> Result<Option<HttpResponse>, anyhow::Error> {
    let saved_response = sqlx::query!(
        r#"
            SELECT
                response_status_code,
                response_headers as "response_headers: Vec<HeaderPairRecord>",
                response_body
            FROM idempotency
            WHERE
                user_id = $1 AND
                idempotency_key = $2
        "#,
        user_id,
        idempotency_key.as_ref()
    )
    .fetch_optional(db)
    .await?;

    if let Some(record) = saved_response {
        let status_code = StatusCode::from_u16(record.response_status_code.try_into()?)?;
        let json: Json<SuccessResponse> = Json(serde_json::from_slice(&record.response_body)?);
        let mut response = HttpResponse::build(status_code);
        for HeaderPairRecord { name, value } in record.response_headers {
            response.append_header((name, value));
        }

        Ok(Some(response.json(json)))
    } else {
        Ok(None)
    }
}
