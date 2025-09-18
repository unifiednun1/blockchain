//! UnifiedNUN Blockchain Node (Starter)



use warp::Filter;
use warp::http::Method;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
mod claim;
mod ai_integration;
mod wasm_contract;
mod db;
mod db_actions;

#[derive(Serialize, Deserialize)]
pub struct MineRequest {
    pub session_id: String,
    pub action: String,
}

#[derive(Serialize, Deserialize)]
pub struct MineResponse {
    pub mined_nun: u64,
    pub user_share: u64,
    pub platform_share: u64,
}

#[tokio::main]
async fn main() {
    // Initialize MongoDB client
    let client = Arc::new(db::get_client().await.expect("Failed to connect to MongoDB"));

    // Mining endpoint (POST /mine)
    let mine = {
        let client = client.clone();
        warp::post()
            .and(warp::path("mine"))
            .and(warp::body::json())
            .and_then(move |req: MineRequest| {
                let client = client.clone();
                async move {
                    // AI-powered reward calculation
                    let engagement_score = ai_integration::get_engagement_score(&req.session_id, &req.action);
                    let mined_nun = ai_integration::calculate_nun_reward(&req.action, engagement_score) as u64;

                    // Call WASM contract for minting/splitting
                    let wasm_path = "contracts/wasm/pkg/nun_contract_bg.wasm"; // Adjust path as needed
                    let contract_result = wasm_contract::call_mint_nun(wasm_path, mined_nun).unwrap_or_else(|_| wasm_contract::MintResult {
                        user_amount: mined_nun / 2,
                        platform_amount: mined_nun - (mined_nun / 2),
                        total: mined_nun,
                    });

                    // DB: ensure session, insert mined NUN
                    if let Err(e) = db_actions::ensure_session(&client, &req.session_id).await {
                        eprintln!("DB error: {e}");
                    }
                    if let Err(e) = db_actions::insert_mined_nun(&client, &req.session_id, &req.action, contract_result.total as i64).await {
                        eprintln!("DB error: {e}");
                    }

                    Ok::<_, warp::Rejection>(warp::reply::json(&MineResponse {
                        mined_nun: contract_result.total,
                        user_share: contract_result.user_amount,
                        platform_share: contract_result.platform_amount,
                    }))
                }
            })
    };

    // Claim endpoint (POST /claim)
    let claim = claim::claim_route(client.clone());

    // Combine routes and add CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(vec!["Content-Type"]);
    let routes = mine.or(claim).with(cors);

    // Start server
    use std::net::SocketAddr;
    let port = std::env::var("PORT").unwrap_or_else(|_| "3030".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");
    warp::serve(routes).run(addr).await;
}
