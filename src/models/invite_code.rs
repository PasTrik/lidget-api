use crate::errors::AppError;

pub struct InviteCode {
    pub id: String,
    pub code: String,
    pub user_id: String,
    pub expires_at: String,
    pub created_at: String,
    pub used_at: Option<String>,
}

pub struct NewInviteCode {
    pub id: String,
    pub code: String,
    pub user_id: String,
    pub created_at: String,
}

pub async fn find_invite_code_by_code(pool: &sqlx::SqlitePool, code: &str) -> Result<Option<InviteCode>, AppError> {
    sqlx::query_as!(InviteCode,
        "SELECT id as \"id!\", code, user_id, expires_at, created_at, used_at
         FROM invite_codes WHERE code = ?",
        code)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::DatabaseError)
}

pub async fn create_invite_code(pool: &sqlx::SqlitePool, code: NewInviteCode) -> Result<InviteCode, AppError> {
    let date = chrono::Utc::now().to_rfc3339();
    let expires_at = (chrono::Utc::now() + chrono::Duration::days(1)).to_rfc3339();
    sqlx::query_as!(InviteCode,
        "INSERT INTO invite_codes (id, code, user_id, expires_at, created_at)
         VALUES (?, ?, ?, ?, ?)
         RETURNING id as \"id!\", code, user_id, expires_at, created_at, used_at",
        code.id, code.code, code.user_id, expires_at, date
    )
        .fetch_one(pool).await.map_err(|_| AppError::DatabaseError)
}