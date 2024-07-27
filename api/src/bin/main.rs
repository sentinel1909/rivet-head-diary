// // rivet-head-diary api
// src/bin/main.rs

// dependencies
use rivet_head_diary_api_lib::startup::create;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = create();

    Ok(rocket.into())
}
