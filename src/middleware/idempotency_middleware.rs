use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};

use crate::{idempotency::key::IdempotencyKey, utils::e400};

const TXT_IDEM_INVALID: &str = "Idempotency key invalid";

pub async fn idempotency_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let idem_header = req
        .headers()
        .get("Idempotency-Key")
        .and_then(|head| head.to_str().ok());

    let Some(idem_key) = idem_header else {
        return Err(e400(TXT_IDEM_INVALID));
    };

    let idem_key: IdempotencyKey = idem_key.to_owned().try_into().map_err(|e| e400(e))?;

    req.extensions_mut().insert(idem_key);

    next.call(req).await
    // post-processing
}
