// // rivet-head-diary api
// src/bin/main.rs

// dependencies
use rivet_head_diary_api_lib::domain::api_service::ApiService;
use rivet_head_diary_api_lib::telemetry::{get_subscriber, init_subscriber};
use rivet_head_diary_api_lib::{config::ApiConfig, startup::Application};
use shuttle_runtime::{Error, SecretStore};
use sqlx::PgPool;

// main function
#[shuttle_runtime::main]
async fn shuttle_main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> Result<ApiService, Error> {
    // initialize tracing
    let subscriber = get_subscriber(
        "rivet-head-diary-api".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // get the API configuration
    let config = ApiConfig::try_from(secrets)?;

    // build the application, pass in the database pool as a configuration parameter
    let Application(api) = Application::build(pool, config)?;

    // pass the ApiService to the Shuttle runtime to start
    Ok(ApiService { api })
}
