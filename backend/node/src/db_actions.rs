//! UnifiedNUN DB Actions: Sessions, Mining, Claims

use mongodb::{bson::{doc, oid::ObjectId}, Client, Collection};
use futures_util::stream::TryStreamExt;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinedNun {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub session_id: String,
    pub action: String,
    pub amount: i64,
    pub claimed: bool,
}



pub async fn ensure_session(client: &Client, session_id: &str) -> mongodb::error::Result<()> {
    let db = client.database("unifiednun");
    let col: Collection<Session> = db.collection("sessions");
    let filter = doc! { "session_id": session_id };
    if col.find_one(filter.clone(), None).await?.is_none() {
        let session = Session { id: None, session_id: session_id.to_string() };
        col.insert_one(session, None).await?;
    }
    Ok(())
}

pub async fn insert_mined_nun(client: &Client, session_id: &str, action: &str, amount: i64) -> mongodb::error::Result<()> {
    let db = client.database("unifiednun");
    let col: Collection<MinedNun> = db.collection("mined_nun");
    let mined = MinedNun {
        id: None,
        session_id: session_id.to_string(),
        action: action.to_string(),
        amount,
        claimed: false,
    };
    col.insert_one(mined, None).await?;
    Ok(())
}

pub async fn get_unclaimed_nun(client: &Client, session_id: &str) -> mongodb::error::Result<i64> {
    let db = client.database("unifiednun");
    let col: Collection<MinedNun> = db.collection("mined_nun");
    let filter = doc! { "session_id": session_id, "claimed": false };
    let mut cursor = col.find(filter, None).await?;
    let mut total = 0i64;
    while let Some(doc) = cursor.try_next().await? {
        total += doc.amount;
    }
    Ok(total)
}

pub async fn mark_nun_claimed(client: &Client, session_id: &str) -> mongodb::error::Result<()> {
    let db = client.database("unifiednun");
    let col: Collection<MinedNun> = db.collection("mined_nun");
    let filter = doc! { "session_id": session_id, "claimed": false };
    let update = doc! { "$set": { "claimed": true } };
    col.update_many(filter, update, None).await?;
    Ok(())
}






