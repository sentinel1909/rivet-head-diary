// rivet-head-diary api
// src/lib/data_access_layer.rs

// dependencies
use sqlx::PgPool;

// a struct type for a Postgres database
#[derive(Clone)]
pub struct Postgres {
    pub pool: PgPool,
}

// implement a constructor for the Postgres type
impl Postgres {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
