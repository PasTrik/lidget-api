use std::sync::Arc;
use axum::extract::{Query, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures_util::StreamExt;
use serde::Deserialize;
use crate::errors::AppError;
use crate::jwt::verify_token;
use crate::models::{notification, session, user};
use crate::state::AppState;
use crate::ws::registry;
use sha2::{Sha256, Digest};
use crate::models::session::Session;
use crate::ws::types::{IncomingMessage, OutgoingMessage};

#[derive(Deserialize)]
pub struct WsParams {
    pub token: String
}

pub async fn handle(
    State(state): State<Arc<AppState>>,
    Query(params): Query<WsParams>,
    ws: WebSocketUpgrade
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, params.token))
}

pub async fn handle_socket(
    socket: WebSocket,
    state: Arc<AppState>,
    token: String
) -> () {
    if let Err(e) = process_socket(socket, state, token).await {
        eprintln!("WebSocket error: {:?}", e);
    }

    ()
}

async fn process_socket(
    socket: WebSocket,
    state: Arc<AppState>,
    token: String
) -> Result<(), AppError> {
    let claims = verify_token(&token, &state.config.jwt_secret)?;
    let user_id = claims.sub;

    let token_hash = format!("{:x}", sha2::Sha256::digest(token.as_bytes()));
    let session = session::find_session_by_token_hash(
        &state.db,
        &token_hash,
    ).await?.ok_or(AppError::Unauthorized)?;

    let (mut sender, mut receiver) = socket.split();

    registry::register(
        &state.ws_registry,
        user_id.clone(),
        session.id.clone(),
        sender,
    ).await?;

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<IncomingMessage>(&text) {
                    Ok(incoming) => {
                        if let Err(e) = handle_message(incoming, &user_id, &session, &state).await {
                            eprintln!("Error handling message: {:?}", e);
                        }
                    }
                    Err(_) => {
                        eprintln!("Invalid message format: {}", text);
                    }
                }
            }

            Ok(Message::Close(_)) => {
                break;
            }

            Err(e) => {
                eprintln!("WebSocket error: {:?}", e);
                break;
            }

            _ => {}
        }
    }

    registry::unregister_session(
        &state.ws_registry,
        &user_id,
        &session.id,
    ).await?;

    session::update_session_last_seen_by_id(&state.db, &session.id).await.map_err(|_| AppError::DatabaseError)?;

    Ok(())
}

async fn handle_message(
    message: IncomingMessage,
    user_id: &str,
    session: &Session,
    state: &Arc<AppState>,
) -> Result<(), AppError> {
    match message {
        IncomingMessage::Sync => {
            let pending_notifications = notification::find_pending_notifications(
                &state.db,
                user_id,
            ).await?;

            for notif in pending_notifications {
                let message = OutgoingMessage::PendingNotifications {
                    notifications: vec![notif.payload.clone()]
                };
                registry::send_to(
                    &state.ws_registry,
                    user_id,
                    message,
                ).await?;
                notification::mark_notification_as_delivered(
                    &state.db,
                    &notif.id,
                ).await?;
            }
        }
        IncomingMessage::QuizStarted { quiz_id } => {
            // TODO: notifier le partenaire
        }
        IncomingMessage::QuizProgress { quiz_id, progress, total } => {
            // TODO: notifier le partenaire
        }
    }
    Ok(())
}