use actix_web::{
    HttpResponse,
    body::{BoxBody, EitherBody, MessageBody, to_bytes},
    http::StatusCode,
    web::Json,
};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{domain::common::SuccessResponse, idempotency::key::IdempotencyKey};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "header_pair")]
struct HeaderPairRecord {
    name: String,
    value: Vec<u8>,
}

pub enum NextAction {
    StartProcessing(Transaction<'static, Postgres>),
    ReturnSavedResponse(HttpResponse),
}

pub async fn try_processing(
    db: &PgPool,
    idempotency_key: &IdempotencyKey,
    user_id: Uuid,
) -> Result<NextAction, anyhow::Error> {
    let mut transaction = db.begin().await?;
    let query = sqlx::query!(
        r#"
            INSERT INTO idempotency (
                user_id,
                idempotency_key,
                created_at
            )
            VALUES ($1, $2, now())
            ON CONFLICT DO NOTHING
        "#,
        user_id,
        idempotency_key.as_ref()
    );

    let n_inserted_rows = transaction.execute(query).await?.rows_affected();

    if n_inserted_rows > 0 {
        Ok(NextAction::StartProcessing(transaction))
    } else {
        let saved_response = get_saved_response(db, idempotency_key, user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("We expected a saved response, we didn't find it"))?;

        Ok(NextAction::ReturnSavedResponse(saved_response))
    }
}

pub async fn save_response<B: MessageBody>(
    mut transaction: Transaction<'static, Postgres>,
    idempotency_key: &IdempotencyKey,
    user_id: Uuid,
    response: HttpResponse<B>,
) -> Result<HttpResponse<EitherBody<BoxBody>>, anyhow::Error> {
    let (response_head, body) = response.into_parts();
    let body = to_bytes(body).await.unwrap_or_default();

    let status_code = response_head.status().as_u16() as i16;
    let headers = {
        let mut hdrs = Vec::with_capacity(response_head.headers().len());
        for (name, value) in response_head.headers().iter() {
            let name = name.as_str().to_owned();
            let value = value.as_bytes().to_owned();

            hdrs.push(HeaderPairRecord { name, value });
        }

        hdrs
    };

    let query = sqlx::query_unchecked!(
        r#"
            UPDATE idempotency
            SET
                response_status_code = $3,
                response_headers = $4,
                response_body = $5
            WHERE
                user_id = $1 AND
                idempotency_key = $2
        "#,
        user_id,
        idempotency_key.as_ref(),
        status_code,
        headers,
        body.as_ref()
    );

    transaction.execute(query).await?;

    let http_response = response_head.set_body(body).map_into_boxed_body();

    Ok(http_response.map_into_left_body())
}

pub async fn get_saved_response(
    db: &PgPool,
    idempotency_key: &IdempotencyKey,
    user_id: Uuid,
) -> Result<Option<HttpResponse>, anyhow::Error> {
    let saved_response = sqlx::query!(
        r#"
            SELECT
                response_status_code as "response_status_code!",
                response_headers as "response_headers!: Vec<HeaderPairRecord>",
                response_body as "response_body!"
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
