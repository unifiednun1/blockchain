//! UnifiedNUN DB module (PostgreSQL, sqlx)
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn get_pool() -> PgPool {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB")
}

// Add async DB functions for sessions, mining, claims here
