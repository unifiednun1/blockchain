use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use crate::db_actions;

#[derive(Deserialize)]
pub struct ClaimRequest {
    pub session_id: String,
    pub wallet_address: String,
}

#[derive(Serialize)]
pub struct ClaimResponse {
    pub claimed_nun: u64,
    pub status: String,
}

pub fn claim_route(pool: Arc<PgPool>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("claim"))
        .and(warp::body::json())
        .and_then(move |req: ClaimRequest| {
            let pool = pool.clone();
            async move {
                // Get unclaimed NUN
                let unclaimed = db_actions::get_unclaimed_nun(&pool, &req.session_id).await.unwrap_or(0);
                if unclaimed > 0 {
                    // Insert claim record and mark mined NUN as claimed
                    let _ = db_actions::insert_claim(&pool, &req.session_id, &req.wallet_address, unclaimed).await;
                    let _ = db_actions::mark_nun_claimed(&pool, &req.session_id).await;
                }
                Ok::<_, warp::Rejection>(warp::reply::json(&ClaimResponse {
                    claimed_nun: unclaimed as u64,
                    status: if unclaimed > 0 {
                        format!("Transferred {} NUN to {}", unclaimed, req.wallet_address)
                    } else {
                        "No unclaimed NUN available.".to_string()
                    },
                }))
            }
        })
}
