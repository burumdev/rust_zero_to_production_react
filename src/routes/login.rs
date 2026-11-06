use actix_web::HttpResponse;
use secrecy::SecretString;

#[derive(serde::Deserialize)]
struct LoginFormData {
    username: String,
    password: SecretString,
}

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().finish()
}
