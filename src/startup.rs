use std::{net::TcpListener, str::FromStr};

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer, dev::Server, http::header::HeaderName, web};
use secrecy::SecretString;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing_actix_web::TracingLogger;

use crate::{
    configuration::{AppEnvironment, Configuration, DatabaseConfiguration},
    email_client::EmailClient,
    middleware::csrf::Csrf,
    routes::{
        health_check::health_check, login::login, newsletters::publish_newsletter,
        subscriptions::subscribe, subscriptions_confirm::confirm,
    },
};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Configuration) -> Result<Self, std::io::Error> {
        let db = get_connection_pool(&configuration.database);

        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email.");

        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            "Tyrell Corporation".to_string(),
            configuration.email_client.authorization_token,
            timeout,
            configuration.email_client.sandbox_id,
            configuration.email_client.sandbox_base_url,
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            db,
            email_client,
            configuration.application.base_url,
            port,
            configuration.application.frontend_dev_origin,
            configuration.application.environment,
            configuration.application.hmac_secret,
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseConfiguration) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}

pub struct ApplicationBaseUrl(pub String);
pub struct ApplicationPort(pub u16);
pub struct ApplicationHmacSecret(pub SecretString);

pub fn run(
    listener: TcpListener,
    db: PgPool,
    email_client: EmailClient,
    base_url: String,
    port: u16,
    frontend_dev_origin: String,
    app_environment: AppEnvironment,
    hmac_secret: SecretString,
) -> Result<Server, std::io::Error> {
    let db = web::Data::new(db);
    let email_client = web::Data::new(email_client);
    let app_base_url = web::Data::new(ApplicationBaseUrl(base_url.clone()));
    let app_port = web::Data::new(ApplicationPort(port));
    let environment = web::Data::new(app_environment);
    let hmac_secret = web::Data::new(ApplicationHmacSecret(hmac_secret));

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_dev_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
                HeaderName::from_str("X-CSRF-Token").unwrap(),
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Csrf)
            .wrap(cors)
            .wrap(TracingLogger::default())
            .service(
                web::scope("/api")
                    .route("/subscriptions", web::post().to(subscribe))
                    .route("/subscriptions/confirm", web::get().to(confirm))
                    .route("/newsletters", web::post().to(publish_newsletter))
                    .route("/login", web::post().to(login))
                    .route("/health_check", web::get().to(health_check)),
            )
            .app_data(db.clone())
            .app_data(email_client.clone())
            .app_data(app_base_url.clone())
            .app_data(app_port.clone())
            .app_data(environment.clone())
            .app_data(hmac_secret.clone())
            .service(
                Files::new("/", "./frontend/dist")
                    .index_file("index.html")
                    .prefer_utf8(true)
                    // This allows frontend routing like react-router to work
                    // properly and not request non-existing files on the server.
                    .default_handler(|req: actix_web::dev::ServiceRequest| {
                        let (http_req, _payload) = req.into_parts();
                        async move {
                            let response =
                                actix_files::NamedFile::open("./frontend/dist/index.html")?
                                    .into_response(&http_req);
                            Ok(actix_web::dev::ServiceResponse::new(http_req, response))
                        }
                    }),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
