// tests/api/helpers.rs

// dependencies
use rivet_head_diary_api_lib::config::ApiConfig;
use rivet_head_diary_api_lib::telemetry::{get_subscriber, init_subscriber};
use rivet_head_diary_api_lib::Application;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use once_cell::sync::Lazy;
use reqwest::header::HeaderValue;
use sqlx::{postgres::PgConnectOptions, Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
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

// a struct type to represent a Test Application
#[allow(dead_code)]
pub struct TestApp {
    pub address: String,
    pub port: u16, // field not used right yet
    pub api_client: reqwest::Client,
}

// methods for the TestApp type
impl TestApp {
    pub async fn login(&self, body: &serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/api/login", &self.address))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .json(&body)
            .send()
            .await
            .expect("Unable to send request.")
    }

    pub async fn protected(&self, token: &str) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/api/protected", &self.address))
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .expect("Unable to send request.")
    }
}

// struct type to represent the database settings for testing
#[derive(Clone, Debug)]
struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

// implement methods on the database type for testing
impl DatabaseSettings {
    pub fn new() -> Self {
        DatabaseSettings {
            username: "postgres".into(),
            password: "postgres".into(),
            port: 5432,
            host: "localhost".into(),
            // Create a new database name for each test
            database_name: Uuid::new_v4().to_string(),
        }
    }

    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

// function to configure the database for testing
async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection_pool
}

// Spawn a new application as a background task, to ensure test isolation
pub async fn spawn_app() -> TestApp {
    // initialize tracing only once
    Lazy::force(&TRACING);

    // test database configuration settings
    let db_config = DatabaseSettings::new();

    // create and migrate the test database
    let pool = configure_database(&db_config).await;

    // get the api configuration
    let config = ApiConfig {
        client_id: "test_id".to_string().into(),
        client_secret: "test_secret".to_string().into(),
    };

    let application = Application::build(pool, config).expect("Failed to build application");

    let listener = TcpListener::bind("localhost:0").expect("Failed to bind a port");
    let addr = listener.local_addr().unwrap();
    let port = addr.port();

    tokio::spawn(application.run_until_stopped(addr));

    let client = reqwest::Client::builder().build().unwrap();

    TestApp {
        address: format!("http://localhost:{port}"),
        port,
        api_client: client,
    }
}
