//! UnifiedNUN DB Actions: Sessions, Mining, Claims
use sqlx::{PgPool, Row};

pub async fn ensure_session(pool: &PgPool, session_id: &str) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO sessions (session_id) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn insert_mined_nun(pool: &PgPool, session_id: &str, action: &str, amount: i64) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO mined_nun (session_id, action, amount) VALUES ($1, $2, $3)")
        .bind(session_id)
        .bind(action)
        .bind(amount)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_unclaimed_nun(pool: &PgPool, session_id: &str) -> sqlx::Result<i64> {
    let row = sqlx::query("SELECT COALESCE(SUM(amount),0) as total FROM mined_nun WHERE session_id = $1 AND claimed = FALSE")
        .bind(session_id)
        .fetch_one(pool)
        .await?;
    Ok(row.get::<i64,_>("total"))
}

pub async fn mark_nun_claimed(pool: &PgPool, session_id: &str) -> sqlx::Result<()> {
    sqlx::query("UPDATE mined_nun SET claimed = TRUE WHERE session_id = $1 AND claimed = FALSE")
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn insert_claim(pool: &PgPool, session_id: &str, wallet_address: &str, amount: i64) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO claims (session_id, wallet_address, amount) VALUES ($1, $2, $3)")
        .bind(session_id)
        .bind(wallet_address)
        .bind(amount)
        .execute(pool)
        .await?;
    Ok(())
}
