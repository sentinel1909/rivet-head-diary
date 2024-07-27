// src/lib/startup.rs

// dependencies
use crate::handlers::health::health_check;
use rocket::{Build, build, Rocket, routes};
use rocket::fs::FileServer;

// function to create a rocket instance
pub fn create() -> Rocket<Build> {
    build()
        .mount("/api", routes!(health_check))
        .mount("/", FileServer::from("dist"))
}