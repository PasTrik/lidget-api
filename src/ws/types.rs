use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IncomingMessage {
  Sync,
  QuizStarted { quiz_id: String },
  QuizProgress { quiz_id: String, progress: i64, total: i64 },
}

#[derive(Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutgoingMessage {
  PendingNotifications { notifications: Vec<String> },
  QuizStarted { quiz_id: String, nickname: String },
  QuizProgress { quiz_id: String, progress: i64, total: i64 },
  NewPhoto { photo_id: String, caption: Option<String> },
  NewDrawing { photo_id: String },
  EventCreated { event_id: String, title: String, date: String },
  PartnerJoined { user_id: String, display_name: String }
}