use crate::errors::AppError;
use sha2::Digest;
use sqlx::SqlitePool;

pub struct Session {
    pub id: String,
    pub user_id: String,
    pub token_hash: String,
    pub device_name: String,
    pub last_seen: String,
    pub created_at: String,
}

pub struct NewSession {
    pub id: String,
    pub user_id: String,
    pub device_name: String,
    pub token_hash: String,
}

pub async fn create_session(pool: &SqlitePool, session: NewSession) -> Result<Session, AppError> {
    sqlx::query_as!(
        Session,
        "INSERT INTO sessions (user_id, device_name, token_hash)
         VALUES (?, ?, ?)
         RETURNING id as \"id!\", user_id, token_hash, device_name, last_seen, created_at",
        session.user_id, session.device_name, session.token_hash
    )
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::DatabaseError)
}

pub async fn find_session_by_token_hash(pool: &SqlitePool, token_hash: &str) -> Result<Option<Session>, AppError> {
    sqlx::query_as!(
        Session,
        "SELECT id as \"id!\", user_id, token_hash, device_name, last_seen, created_at
         FROM sessions WHERE token_hash = ?",
        token_hash
    )
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::DatabaseError)
}

pub async fn delete_session_by_token_hash(pool: &SqlitePool, token_hash: &str) -> Result<(), AppError> {
    sqlx::query!(
        "DELETE FROM sessions WHERE token_hash = ?",
        token_hash
    )
        .execute(pool).await.map_err(|_| AppError::DatabaseError)?;
    Ok(())
}

pub async fn update_session_last_seen_by_token_hash(
    pool: &SqlitePool,
    token_hash: &str,
) -> Result<(), AppError> {
    println!("token: {:}", token_hash);
    sqlx::query!(
        "UPDATE sessions SET last_seen = CURRENT_TIMESTAMP WHERE token_hash = ? AND last_seen < datetime('now', '-3 minutes')",
        token_hash
    ).execute(pool).await.map_err(|_| AppError::DatabaseError)?;
    Ok(())
}