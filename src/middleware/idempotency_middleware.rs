use actix_session::SessionExt;
use actix_web::{
    Error,
    body::{BoxBody, EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::Data,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    idempotency::{
        key::IdempotencyKey,
        persistence::{NextAction, save_response, try_processing},
    },
    utils::{e400, e500},
};

const TXT_IDEM_INVALID: &str = "Idempotency key invalid";

pub async fn idempotency_middleware<B: MessageBody>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<EitherBody<BoxBody>>, Error> {
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

    match try_processing(db, &idem_key, user_id).await.map_err(e500)? {
        NextAction::StartProcessing(transaction) => {
            let service_response = next.call(req).await?;
            let (http_request, http_response) = service_response.into_parts();
            let http_response = save_response(transaction, &idem_key, user_id, http_response);

            Ok(ServiceResponse::new(
                http_request,
                http_response.await.map_err(e500)?,
            ))
        }
        NextAction::ReturnSavedResponse(sr) => Ok(req.into_response(sr).map_into_right_body()),
    }
}
