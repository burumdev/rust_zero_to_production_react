use std::future::{Ready, ready};
use std::rc::Rc;

use actix_web::body::{BoxBody, EitherBody};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::http::Method;
use actix_web::{Error, HttpResponse, HttpResponseBuilder, web};
use chrono::Utc;
use futures_util::future::LocalBoxFuture;
use hmac::{Hmac, KeyInit, Mac};
use secrecy::ExposeSecret;
use sha2::Sha256;

use crate::startup::ApplicationHmacSecret;

type HmacSha256 = Hmac<Sha256>;

pub struct Csrf;

impl<S, B> Transform<S, ServiceRequest> for Csrf
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CsrfMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CsrfMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct CsrfMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CsrfMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let s = self.service.clone();

        return Box::pin(async move {
            let Some(hmac_secret) = req.app_data::<web::Data<ApplicationHmacSecret>>() else {
                return Ok(error_response(
                    req,
                    &mut HttpResponse::InternalServerError(),
                ));
            };

            let hmac_secret = hmac_secret.get_ref().0.expose_secret().to_owned();
            let method = req.method();

            if method == Method::GET || method == Method::HEAD || method == Method::OPTIONS {
                let mut res = s.call(req).await?;
                // If the response doesn't have the csrf_token cookie
                // We add it, so it will be sent with every mutation request.
                if res.request().cookie("csrf_token").is_none() {
                    let ts = Utc::now().timestamp().to_string();
                    let signature = calculate_signature_hex(&hmac_secret, &ts);
                    let token = format!("{}.{}", ts, signature);

                    let cookie = Cookie::build("csrf_token", token)
                        .http_only(false)
                        .same_site(SameSite::Strict)
                        .finish();

                    res.response_mut().add_cookie(&cookie)?;
                }

                Ok(res.map_body(|_head, body| EitherBody::left(body)))
            } else {
                let header_token = req
                    .headers()
                    .get("X-CSRF-Token")
                    .and_then(|val| val.to_str().ok());

                let cookie_token = req.cookie("csrf_token").map(|c| c.value().to_string());

                let is_valid = match (header_token, cookie_token) {
                    (Some(ht), Some(ct)) if ht == ct => verify_csrf_token(ht, &hmac_secret),
                    _ => false,
                };

                if !is_valid {
                    return Ok(error_response(req, &mut HttpResponse::Forbidden()));
                }

                let res = s.call(req).await?;

                Ok(res.map_body(|_head, body| EitherBody::left(body)))
            }
        });
    }
}

fn verify_csrf_token(token: &str, hmac_secret: &str) -> bool {
    let parts: Vec<&str> = token.split(".").collect();

    if parts.len() != 2 {
        return false;
    }

    let timestamp_str = parts[0];
    let candidate_signature = parts[1];

    let Ok(timestamp) = timestamp_str.parse::<i64>() else {
        return false;
    };

    if Utc::now().timestamp() - timestamp > 3600 {
        return false;
    }

    let computed_signature = calculate_signature_hex(hmac_secret, timestamp_str);

    subtle::ConstantTimeEq::ct_eq(
        candidate_signature.as_bytes(),
        computed_signature.as_bytes(),
    )
    .into()
}

fn error_response<B>(
    req: ServiceRequest,
    response_builder: &mut HttpResponseBuilder,
) -> ServiceResponse<EitherBody<B>> {
    req.into_response(response_builder.body("").map_into_boxed_body())
        .map_into_right_body()
}

pub fn calculate_signature_hex(hmac_secret: &str, ts: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(hmac_secret.as_bytes()).unwrap();
    mac.update(ts.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}
