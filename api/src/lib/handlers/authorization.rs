// rivet-head-diary api
// src/lib/handlers/authorization.rs

// dependencies
use crate::authentication::{Claims, KEYS};
use crate::errors::AuthError;
use crate::startup::AppState;
use axum::Json;
use axum::extract::State;
use chrono::Utc;
use jsonwebtoken::{encode, Header};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

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
