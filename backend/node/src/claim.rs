use warp::Filter;
use std::sync::Arc;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use warp::{Rejection, Reply};
use crate::db_actions;

// Stub claim_route for backend build
#[derive(Deserialize)]
pub struct ClaimRequest {
	pub session_id: String,
	pub wallet_address: String,
}

#[derive(Serialize)]
pub struct ClaimResponse {
	pub claimed_amount: i64,
	pub status: String,
}

pub fn claim_route(client: Arc<Client>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	let post_claim = warp::post()
		.and(warp::path("claim"))
		.and(warp::body::json())
		.and(with_client(client.clone()))
		.and_then(handle_claim);

	let get_unclaimed = warp::get()
		.and(warp::path("unclaimed"))
		.and(warp::query::<UnclaimedQuery>())
		.and(with_client(client))
		.and_then(handle_unclaimed);

	post_claim.or(get_unclaimed)
}

#[derive(Deserialize)]
pub struct UnclaimedQuery {
	pub session_id: String,
}

fn with_client(client: Arc<Client>) -> impl Filter<Extract = (Arc<Client>,), Error = std::convert::Infallible> + Clone {
	warp::any().map(move || client.clone())
}

async fn handle_unclaimed(query: UnclaimedQuery, client: Arc<Client>) -> Result<impl Reply, Rejection> {
	match db_actions::get_unclaimed_nun(&client, &query.session_id).await {
		Ok(amount) => Ok(warp::reply::json(&serde_json::json!({"unclaimed": amount}))),
		Err(_) => Ok(warp::reply::json(&serde_json::json!({"error": "DB error"}))),
	}
}

async fn handle_claim(req: ClaimRequest, client: Arc<Client>) -> Result<impl Reply, Rejection> {
	// Get unclaimed NUNs
	let amount = match db_actions::get_unclaimed_nun(&client, &req.session_id).await {
		Ok(a) => a,
		Err(_) => return Ok(warp::reply::json(&serde_json::json!({"error": "DB error"}))),
	};
	if amount == 0 {
		return Ok(warp::reply::json(&ClaimResponse { claimed_amount: 0, status: "Nothing to claim".to_string() }));
	}
	// Mark as claimed
	if let Err(_) = db_actions::mark_nun_claimed(&client, &req.session_id).await {
		return Ok(warp::reply::json(&serde_json::json!({"error": "DB error"}))); 
	}
	// Optionally: record claim (not implemented here)
	Ok(warp::reply::json(&ClaimResponse { claimed_amount: amount, status: "Claimed".to_string() }))
}
