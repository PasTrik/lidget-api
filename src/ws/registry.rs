use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::RwLock;
use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use crate::errors::AppError;
use crate::ws::types::OutgoingMessage;


pub struct WsSession {
  pub session_id: String,
  pub sender: SplitSink<WebSocket, Message>,
}

pub type Registry = Arc<RwLock<HashMap<String, Vec<WsSession>>>>;


pub async fn register(
  registry: &Registry,
  user_id: String,
  session_id: String,
  sender: SplitSink<WebSocket, Message>
) -> Result<(), AppError> {
  let mut map = registry.write().await;
  map.entry(user_id)
    .or_insert_with(Vec::new)
    .push(WsSession { session_id, sender });
  Ok(())
}

pub async fn unregister(registry: &Registry, user_id: String) -> Result<(), AppError> {
  let mut map = registry.write().await;
  map.remove(&user_id);
  Ok(())
}

pub async fn unregister_session(registry: &Registry, user_id: &str, session_id: &str) -> Result<(), AppError> {
  let mut map = registry.write().await;
  if let Some(sessions) = map.get_mut(user_id) {
    sessions.retain(|s| s.session_id != session_id);
    if sessions.is_empty() {
      map.remove(user_id);
    }
  }
  Ok(())
}


pub async fn send_to(registry: &Registry, user_id: &str, message: OutgoingMessage) -> Result<(), AppError> {
  let mut map = registry.write().await;
  if let Some(sessions) = map.get_mut(user_id) {
    for session in sessions.iter_mut() {
      let json = serde_json::to_string(&message)
        .map_err(|_| AppError::InternalServerError)?;
      session.sender.send(Message::Text(json.into()))
        .await
        .map_err(|_| AppError::InternalServerError)?;
    }
  }
  Ok(())
}