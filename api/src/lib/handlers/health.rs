// src/lib/handlers/health_check.rs

// dependencies
use rocket::get;
use rocket::http::Status;

// health_check handler
#[get("/health_check")]
pub fn health_check() -> Status {
    Status::Ok
}