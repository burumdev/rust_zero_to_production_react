use actix_web::{
    HttpRequest, HttpResponse, error::ErrorInternalServerError, http::StatusCode, web,
};
use secrecy::ExposeSecret;

use crate::{middleware::csrf_middleware::get_csrf_cookie, startup::ApplicationHmacSecret};

pub async fn csrf_cookie(
    req: HttpRequest,
    hmac_secret: web::Data<ApplicationHmacSecret>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut response = HttpResponse::new(StatusCode::OK);

    if req.cookie("csrf_token").is_none() {
        let mut cookie = get_csrf_cookie(hmac_secret.get_ref().0.expose_secret());
        // The path shouldn't be '/api' so this cookie can be read by javascript on main page
        cookie.set_path("/");

        response
            .add_cookie(&cookie)
            .map_err(|e| ErrorInternalServerError(e))?;
    }

    Ok(response)
}
