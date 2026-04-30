use crate::errors::AppError;
use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Serialize)]
pub struct Relationship {
    pub id: String,
    pub user1_id: String,
    pub user2_id: String,
    pub user1_nickname: Option<String>, /* Ici le user1_nickname est le pseudo choisi par le 2ème partenaire */
    pub user2_nickname: Option<String>, /* De même */
    pub created_at: String,
}

pub struct NewRelationship {
    pub id: String,
    pub user1_id: String,
    pub user2_id: String,
}

pub async fn create_relationship(pool: &SqlitePool, relationship: NewRelationship) -> Result<Relationship, AppError> {
    sqlx::query_as!(
        Relationship,
        "INSERT INTO relationships (id, user1_id, user2_id)
         VALUES (?, ?, ?)
         RETURNING id as \"id!\", user1_id, user2_id, user1_nickname, user2_nickname, created_at",
        relationship.id, relationship.user1_id, relationship.user2_id
    )
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::DatabaseError)
}

pub async fn get_relationship_by_user_id(pool: &SqlitePool, user_id: &str) -> Result<Option<Relationship>, AppError> {
    sqlx::query_as!(
        Relationship,
        "SELECT id as \"id!\", user1_id, user2_id, user1_nickname, user2_nickname, created_at
         FROM relationships
         WHERE user1_id = ? OR user2_id = ?",
        user_id, user_id
    )
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::DatabaseError)
}