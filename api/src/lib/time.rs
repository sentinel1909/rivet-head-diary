// rivet-head-diary api
// src/lib/time.rs

// dependencies
use chrono::prelude::{DateTime, Local};

// function to return the local date and time
pub fn date_time() -> DateTime<Local> {
    Local::now()
}

// function to return today's date as a string
pub fn today_date_string() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

// function to return the current year as a string
pub fn year_string() -> String {
    Local::now().format("%Y").to_string()
}
