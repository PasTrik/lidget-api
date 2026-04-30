use crate::errors::AppError;
use crate::jwt;
use crate::jwt::Claims;
use crate::models::relationship::Relationship;
use crate::models::user::User;
use crate::models::{relationship, sessions, user};
use crate::state::AppState;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use serde::Serialize;
use sha2::Digest;
use std::sync::Arc;

#[derive(Serialize)]
pub struct AuthUser {
    pub user: User,
    pub relationship: Option<Relationship>,
    pub claims: Claims,
}

pub struct GuestUser;

#[async_trait]
impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        // 1. Extraire le header Authorization
        let token = parts.headers.get("Authorization").ok_or(AppError::Unauthorized)?.to_str().map_err(|_| AppError::Unauthorized)?;
        // 2. Vérifier le format "Bearer <token>"
        let token = token.strip_prefix("Bearer ").ok_or(AppError::Unauthorized)?.trim();
        // 3. verify_token()
        let claims = jwt::verify_token(token, &state.config.jwt_secret)?;
        // 4. find_user_by_id()
        let user = user::find_user_by_id(
            &state.db,
            &claims.sub,
        ).await?.ok_or(AppError::Unauthorized)?;
        // 5. Retourner AuthUser

        let relationship = relationship::get_relationship_by_user_id(
            &state.db,
            &claims.sub,
        ).await?;

        let token_hash = format!("{:x}", sha2::Sha256::digest(token.as_bytes()));
        sessions::update_session_last_seen_by_token_hash(&state.db, &token_hash).await.map_err(|_| AppError::DatabaseError)?;

        Ok(AuthUser { user, claims, relationship })
    }
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for GuestUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &Arc<AppState>) -> Result<Self, Self::Rejection> {
        let token = parts.headers.get("Authorization").and_then(
            |v| v.to_str().ok()
        ).and_then(
            |v| v.strip_prefix("Bearer ")
        );

        if let Some(token) = token {
            if jwt::verify_token(token, &state.config.jwt_secret).is_ok() {
                return Err(AppError::Unauthorized);
            }
        }

        Ok(GuestUser)
    }
}