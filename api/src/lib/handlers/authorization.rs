// rivet-head-diary api
// src/lib/handlers/authorization.rs

// dependencies
use crate::authentication::{Claims, KEYS};
use crate::renderer::compile_diary_template;
use crate::errors::AuthError;
use crate::startup::AppState;
use axum::Json;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use chrono::Utc;
use jsonwebtoken::{encode, Header};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use tera::Context;

// struct type to represent the body of our authorization
#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

// implement some methods for our AuthBody type
impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthPayLoad {
    pub client_id: String,
    pub client_secret: String,
}

// handler to authorize
pub async fn authorize(
    State(app_state): State<AppState>,
    Json(payload): Json<AuthPayLoad>,
) -> Result<Json<AuthBody>, AuthError> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    if payload.client_id != *app_state.client_id.expose_secret()
        || payload.client_secret != *app_state.client_secret.expose_secret()
    {
        return Err(AuthError::WrongCredentials);
    }

    let exp = (Utc::now().naive_utc() + chrono::naive::Days::new(1))
        .and_utc()
        .timestamp() as usize;
    let claims = Claims {
        username: payload.client_id,
        exp,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}

// function to set up the tera index template
fn diary_template(msg: String) -> String {
    let mut context = Context::new();
    context.insert("message", &msg);
    compile_diary_template().expect("Unable to compile template").render("index.html", &context).expect("Unable to render index template")
}

// handler to allow access into protected areas
pub async fn protected(claims: Claims) -> impl IntoResponse {
     let msg = format!("Welcome to the protected area, {}!", claims.username);
     Html(diary_template(msg))
}
