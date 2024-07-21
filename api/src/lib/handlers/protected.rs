// src/lib/handlers/protected.rs

// dependencies
use axum::Json;
use axum::response::IntoResponse;
use crate::authentication::Claims;
use serde::Serialize;

// struct type to represent the authorization handler response
#[derive(Debug, Serialize)]
struct ProtectedResponse {
    status: String,
    msg: String,
}

// handler to allow access into protected areas
pub async fn protected(claims: Claims) -> impl IntoResponse {
     let protected_response = ProtectedResponse {
        status: String::from("Authorized"),
        msg: format!("Welcome to the protected area {}", claims.username)
     };

     Json(protected_response)
}