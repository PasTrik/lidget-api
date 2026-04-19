use sqlx::SqlitePool;
use crate::errors::AppError;

pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub last_latitude: Option<f64>,
    pub last_longitude: Option<f64>,
    pub last_location_at: Option<String>,
    pub created_at: String,
}

pub struct NewUser {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: String,
    pub created_at: String,
}

pub async fn find_user_by_email(pool: &SqlitePool, email: &str) -> Result<Option<User>, AppError> {
    sqlx::query_as!(
        User,
        "SELECT id as \"id!\", email, password_hash, display_name, avatar_url,
         last_latitude, last_longitude, last_location_at, created_at
         FROM users WHERE email = ?",
        email
    )
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::DatabaseError)
}

pub async fn find_user_by_id(pool: &SqlitePool, id: &str) -> Result<Option<User>, AppError> {
    sqlx::query_as!(
        User,
        "SELECT id as \"id!\", email, password_hash, display_name, avatar_url,
         last_latitude, last_longitude, last_location_at, created_at
         FROM users WHERE id = ?",
        id
    )
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::DatabaseError)
}

pub async fn create_user(pool: &SqlitePool, user: NewUser) -> Result<User, AppError> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (id, email, password_hash, display_name, created_at)
         VALUES (?, ?, ?, ?, ?)
         RETURNING id as \"id!\", email, password_hash, display_name, avatar_url,
         last_latitude, last_longitude, last_location_at, created_at",
        user.id, user.email, user.password_hash, user.display_name, user.created_at
    )
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::DatabaseError)
}