//! UnifiedNUN DB module (PostgreSQL, sqlx)
use mongodb::{Client, options::{ClientOptions, ServerApi, ServerApiVersion}};
use std::env;

pub async fn get_client() -> mongodb::error::Result<Client> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client_options = ClientOptions::parse(&db_url).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    Ok(client)
}

// Add async MongoDB functions for sessions, mining, claims here
