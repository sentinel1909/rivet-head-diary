// rivet-head-diary api
// src/lib/domain/api_service.rs

// dependencies
use axum::{extract::Request, Router, ServiceExt};
use tower::layer::Layer;
use tower_http::normalize_path::NormalizePathLayer;

// struct type to represent the api service
pub struct ApiService {
    pub api: Router,
}

// implement the Shuttle Service trait for the AppService type
#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for ApiService {
    async fn bind(self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = self.api;
        let router = NormalizePathLayer::trim_trailing_slash().layer(router);

        axum::serve(
            tokio::net::TcpListener::bind(addr).await?,
            ServiceExt::<Request>::into_make_service(router),
        )
        .await?;

        Ok(())
    }
}
