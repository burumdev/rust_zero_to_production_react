use std::sync::LazyLock;

use sha3::Digest;
use sqlx::{Connection, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::{
    configuration::{DatabaseConfiguration, get_configuration},
    email_client::EmailClient,
    startup::{Application, get_connection_pool},
    telemetry::{get_subscriber, init_subscriber},
};

pub struct TestApp {
    pub address: String,
    pub db: PgPool,
    pub email_client: EmailClient,
    test_user: TestUser,
}

impl TestApp {
    pub async fn post_subscriptions(&self, json_str: &str) -> reqwest::Response {
        let response = reqwest::Client::new()
            .post(&format!("{}/api/subscriptions", &self.address))
            .header("Content-Type", "application/json")
            .body(json_str.to_owned())
            .send()
            .await
            .expect("Failed to execute request.");

        response
    }

    pub async fn post_newsletters(&self, body: serde_json::Value) -> reqwest::Response {
        let response = reqwest::Client::new()
            .post(format!("{}/api/newsletters", &self.address))
            .basic_auth(&self.test_user.username, Some(&self.test_user.password))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request.");

        response
    }
}

pub struct TestUser {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
        }
    }

    async fn store(&self, db: &PgPool) {
        let password_hash = sha3::Sha3_256::digest(self.password.as_bytes());
        let password_hash = hex::encode(password_hash);
        sqlx::query!(
            "INSERT INTO users (user_id, username, password_hash)
            VALUES ($1, $2, $3)",
            self.user_id,
            self.username,
            password_hash,
        )
        .execute(db)
        .await
        .expect("Failed to store test user.");
    }
}

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    // Database name is random and a new database will be spawned with
    // every test run. This is done so individual tests don't conflict with eath other.
    // The way to sanitize this is to run the development
    // Postgres instance in a docker container (which we do). Once docker container is cleared,
    // all databases in it will be cleared too.
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;

        c
    };

    configure_database(&configuration.database).await;

    let app = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");

    let port = app.port();
    let _ = tokio::spawn(app.run_until_stopped());

    let email_client = EmailClient::new(
        configuration.email_client.base_url.clone(),
        configuration.email_client.sender().unwrap(),
        "Tyrell Corporation Testing Department".to_owned(),
        configuration.email_client.authorization_token.clone(),
        configuration.email_client.timeout(),
        configuration.email_client.sandbox_id,
        configuration.email_client.sandbox_base_url.clone(),
    );

    let test_app = TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db: get_connection_pool(&configuration.database),
        email_client,
        test_user: TestUser::generate(),
    };

    test_app.test_user.store(&test_app.db).await;

    test_app
}

async fn configure_database(config: &DatabaseConfiguration) {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    let query = format!(r#"CREATE DATABASE "{}";"#, config.database_name);

    sqlx::query(sqlx::AssertSqlSafe(query.as_str()))
        .execute(&mut connection)
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection)
        .await
        .expect("Failed to migrate the database");
}
