use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use bcrypt::DEFAULT_COST;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::errors::AppError;
use crate::jwt::generate_token;
use crate::models::invite_code;
use crate::models::sessions;
use crate::models::user::{create_user, find_user_by_email, find_user_by_id, NewUser};
use crate::state::AppState;
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub display_name: String
}

#[derive(Serialize)]
pub struct RegisterResponse  {
    pub invite_code: String,
    pub token: String
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), AppError> {
    // 1. Validate input
    let email = body.email.trim().to_lowercase();
    let password = body.password.trim();
    let display_name = body.display_name.trim();

    let hash_password = bcrypt::hash(password, DEFAULT_COST).map_err(|_| AppError::DatabaseError)?;
    let uuid = uuid::Uuid::new_v4();
    let user = find_user_by_email(&state.db, &email).await?;
    if user.is_some() {
        return Err(AppError::Conflict("Adresse email déjà utilisée".to_string())); // je peux pas mettre un message personnalisé: "email déjà used"
    }

    let new_user = create_user(
        &state.db,
        NewUser {
            id: uuid.to_string(),
            email: email.clone(),
            password_hash: hash_password,
            display_name: display_name.to_string(),
        }
    ).await?;

    let invite_code_string = uuid::Uuid::new_v4()
        .to_string()
        .replace("-", "")
        .chars()
        .take(6)
        .collect::<String>()
        .to_uppercase();

    let jwt = generate_token(&new_user.id, &state.config.jwt_secret)?;
    let token_hash = format!("{:x}", Sha256::digest(jwt.as_bytes()));

    sessions::create_session(
        &state.db,
        sessions::NewSession {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: new_user.id.clone(),
            device_name: "Unknown".to_string(),
            token_hash,
        }
    ).await.map_err(|_| AppError::DatabaseError)?;

    // todo: générer l'invite code dans la DB

    let invite_code_db = invite_code::create_invite_code(
        &state.db,
        invite_code::NewInviteCode {
            id: uuid::Uuid::new_v4().to_string(),
            code: invite_code_string,
            user_id: new_user.id.clone(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    ).await.map_err(|_| AppError::DatabaseError)?;

    Ok((StatusCode::CREATED, Json(RegisterResponse { invite_code: invite_code_db.code, token: jwt })))
}