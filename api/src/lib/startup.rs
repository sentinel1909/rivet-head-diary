// rivet-head-diary api
// src/lib/startup.rs

// dependencies
use crate::config::ApiConfig;
use crate::data_access_layer::Postgres;
use crate::handlers::{authorize, health_check, protected};
use crate::telemetry::MakeRequestUuid;
use anyhow::{Error, Result};
use axum::{
    routing::{get, post},
    Router,
};
use secrecy::Secret;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use tracing::Level;

// struct type to represent the application state
#[derive(Clone)]
pub struct AppState {
    pub dal: Arc<Postgres>,
    pub client_id: Secret<String>,
    pub client_secret: Secret<String>,
}

// struct type to represent the application
pub struct Application(pub Router);

// implemente methods for the Application type
impl Application {
    pub fn build(
        pool: PgPool,
        ApiConfig {
            client_id,
            client_secret,
        }: ApiConfig,
    ) -> Result<Application> {
        // create the application state, bind the database pool received from Shuttle to the db_pool field
        let dal = Arc::new(Postgres::new(pool));
        let app_state = AppState {
            dal,
            client_id,
            client_secret,
        };

        // create the app router with associated handlers and state
        let app = Router::new()
            .route("/api/health_check", get(health_check))
            .route("/api/login", post(authorize))
            .route("/api/protected", get(protected))
            .layer(
                ServiceBuilder::new()
                    .set_x_request_id(MakeRequestUuid)
                    .layer(
                        TraceLayer::new_for_http()
                            .make_span_with(
                                DefaultMakeSpan::new()
                                    .include_headers(true)
                                    .level(Level::INFO),
                            )
                            .on_response(DefaultOnResponse::new().include_headers(true)),
                    )
                    .propagate_x_request_id(),
            )
            .nest_service("/", ServeDir::new("dist"))
            .with_state(app_state);

        Ok(Self(app))
    }

    // Utility function to run the application in tests
    pub async fn run_until_stopped(self, addr: SocketAddr) -> Result<(), Error> {
        let listener = TcpListener::bind(addr).await?;
        axum::serve(listener, self.0).await?;

        Ok(())
    }
}
