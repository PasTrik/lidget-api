use crate::errors::AppError;
use crate::jwt::generate_token;
use crate::middleware::auth::{AuthUser, GuestUser};
use crate::models::sessions;
use crate::models::sessions::Session;
use crate::models::user::{create_user, find_user_by_email, NewUser, User};
use crate::models::{invite_code, relationship};
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use bcrypt::DEFAULT_COST;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub display_name: String,
}
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct JoinRequest {
    pub invite_code: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub invite_code: String,
    pub token: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

async fn create_session(state: &Arc<AppState>, token: &str, user: &User) -> Result<Session, AppError> {
    let token_hash = format!("{:x}", Sha256::digest(token.as_bytes()));

    sessions::create_session(
        &state.db,
        sessions::NewSession {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user.id.clone(),
            device_name: "Unknown".to_string(),
            token_hash,
        },
    ).await.map_err(|_| AppError::DatabaseError)
}

pub async fn register(
    _: GuestUser,
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
        },
    ).await?;

    let invite_code_string = uuid::Uuid::new_v4()
        .to_string()
        .replace("-", "")
        .chars()
        .take(6)
        .collect::<String>()
        .to_uppercase();

    let jwt = generate_token(&new_user.id, &state.config.jwt_secret)?;

    create_session(&state, &jwt, &new_user).await.map_err(|_| AppError::DatabaseError)?;

    // todo: générer l'invite code dans la DB

    let invite_code_db = invite_code::create_invite_code(
        &state.db,
        invite_code::NewInviteCode {
            id: uuid::Uuid::new_v4().to_string(),
            code: invite_code_string,
            user_id: new_user.id.clone()
        },
    ).await.map_err(|_| AppError::DatabaseError)?;

    Ok((StatusCode::CREATED, Json(RegisterResponse { invite_code: invite_code_db.code, token: jwt })))
}

pub async fn login(
    _: GuestUser,
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let email = body.email.trim().to_lowercase();
    let password = body.password.trim();

    let user = find_user_by_email(&state.db, &email).await?.ok_or(AppError::Unauthorized)?;
    let password_match = bcrypt::verify(password, &user.password_hash).map_err(|_| AppError::InternalServerError)?;

    if !password_match {
        return Err(AppError::Unauthorized);
    }

    let jwt = generate_token(&user.id, &state.config.jwt_secret).map_err(|_| AppError::InternalServerError)?;

    create_session(&state, &jwt, &user).await.map_err(|_| AppError::DatabaseError)?;

    Ok(Json(LoginResponse { token: jwt }))
}

pub async fn join(
    _: GuestUser,
    State(state): State<Arc<AppState>>,
    Json(body): Json<JoinRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let code = body.invite_code.trim().to_uppercase();
    let email = body.email.trim().to_lowercase();
    let password = body.password.trim();
    let display_name = body.display_name.trim();

    let invitation = invite_code::find_invite_code_by_code(
        &state.db,
        &code,
    ).await?.ok_or(AppError::NotFound)?;

    if invitation.used_at.is_some() {
        return Err(AppError::BadRequest("invitation already used".to_string()));
    }

    if invitation.expires_at < chrono::Utc::now().to_rfc3339() {
        return Err(AppError::BadRequest("invitation expired".to_string()));
    }

    let email_already_used = find_user_by_email(&state.db, &email).await?.is_some();
    if email_already_used {
        return Err(AppError::BadRequest("email already used".to_string()));
    }

    /*
        TODO: suppression des codes lors d'un cron job
    */

    let user_2 = create_user(&state.db, NewUser {
        id: uuid::Uuid::new_v4().to_string(),
        email,
        password_hash: bcrypt::hash(password, DEFAULT_COST).map_err(|_| AppError::DatabaseError)?,
        display_name: display_name.to_string(),
    }).await?;

    let _ = relationship::create_relationship(&state.db, relationship::NewRelationship {
        id: uuid::Uuid::new_v4().to_string(),
        user1_id: invitation.user_id,
        user2_id: user_2.id.clone(),
    }).await.map_err(|_| AppError::DatabaseError)?;

    let jwt = generate_token(&user_2.id, &state.config.jwt_secret).map_err(|_| AppError::InternalServerError)?;
    create_session(&state, &jwt, &user_2).await.map_err(|_| AppError::DatabaseError)?;

    invite_code::mark_invite_code_as_used(&state.db, &invitation.id).await.map_err(|_| AppError::DatabaseError)?;

    Ok(Json(LoginResponse { token: jwt }))
}

#[axum::debug_handler]
pub async fn me(
    auth: AuthUser,
    State(_): State<Arc<AppState>>,
) -> Result<Json<AuthUser>, AppError> {
    Ok(Json(auth))
}