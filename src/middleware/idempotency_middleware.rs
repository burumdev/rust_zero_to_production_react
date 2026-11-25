use actix_session::SessionExt;
use actix_web::{
    Error, HttpMessage,
    body::EitherBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::Data,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    idempotency::{key::IdempotencyKey, persistence::get_saved_response},
    utils::{e400, e500},
};

const TXT_IDEM_INVALID: &str = "Idempotency key invalid";

pub async fn idempotency_middleware<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<EitherBody<B>>, Error> {
    let idem_header = req
        .headers()
        .get("Idempotency-Key")
        .and_then(|head| head.to_str().ok());

    let Some(idem_key) = idem_header else {
        return Err(e400(TXT_IDEM_INVALID));
    };

    let idem_key: IdempotencyKey = idem_key.to_owned().try_into().map_err(e400)?;

    // Unwrapping everything here because prior auth middleware
    // should have added the user_id to the session
    // and database connection should be up and running
    let user_id = req.get_session().get::<Uuid>("user_id").unwrap().unwrap();
    let db = req.app_data::<Data<PgPool>>().unwrap();
    if let Some(saved_response) = get_saved_response(&db, &idem_key, user_id)
        .await
        .map_err(e500)?
    {
        return Ok(req.into_response(saved_response).map_into_right_body());
    }

    req.extensions_mut().insert(idem_key);

    let normal_response = next.call(req).await?;

    // post-processing
    Ok(normal_response.map_into_left_body())
}
