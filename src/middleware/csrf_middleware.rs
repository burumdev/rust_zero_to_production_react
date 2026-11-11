use actix_web::{
    Error,
    body::MessageBody,
    cookie::{Cookie, SameSite},
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorForbidden, ErrorInternalServerError},
    http::Method,
    middleware::Next,
    web,
};
use chrono::Utc;
use hmac::{Hmac, KeyInit, Mac};
use secrecy::ExposeSecret;
use sha2::Sha256;

use crate::startup::ApplicationHmacSecret;

type HmacSha256 = Hmac<Sha256>;

pub async fn csrf_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let Some(hmac_secret) = req.app_data::<web::Data<ApplicationHmacSecret>>() else {
        return Err(ErrorInternalServerError("An internal error occured."));
    };

    let hmac_secret = hmac_secret.get_ref().0.expose_secret().to_owned();
    let method = req.method();

    if method == Method::GET {
        let mut res = next.call(req).await?;
        // If the response doesn't have the csrf_token cookie
        // We add it, so it will be sent with every mutation request.
        if res.request().cookie("csrf_token").is_none() {
            let cookie = get_csrf_cookie(&hmac_secret);
            res.response_mut().add_cookie(&cookie)?;
        }

        Ok(res)
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
            return Err(ErrorForbidden("Invalid CSRF token."));
        }

        next.call(req).await
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

pub fn calculate_signature_hex(hmac_secret: &str, ts: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(hmac_secret.as_bytes()).unwrap();
    mac.update(ts.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

pub fn get_csrf_cookie(hmac_secret: &str) -> Cookie<'_> {
    let ts = Utc::now().timestamp().to_string();
    let signature = calculate_signature_hex(hmac_secret, &ts);
    let token = format!("{}.{}", ts, signature);

    Cookie::build("csrf_token", token)
        .http_only(false)
        .same_site(SameSite::Strict)
        .finish()
}
