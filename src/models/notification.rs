use crate::errors::AppError;

pub struct Notification {
  pub id: String,
  pub user_id: String,
  pub event_type: String,
  pub payload: String,
  pub created_at: String,
  pub delivered_at: Option<String>,
}

pub struct NewNotification {
  pub id: String,
  pub user_id: String,
  pub event_type: String,
  pub payload: String,
}

pub async fn create_notification(
  pool: &sqlx::SqlitePool,
  notification: NewNotification,
) -> Result<Notification, AppError> {
  sqlx::query_as!(
    Notification,
    "INSERT INTO notifications_queue (id, user_id, event_type, payload)
     VALUES (?, ?, ?, ?)
     RETURNING id as \"id!\", user_id, event_type, payload, created_at, delivered_at",
    notification.id, notification.user_id, notification.event_type, notification.payload
  ).fetch_one(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn find_pending_notifications(
  pool: &sqlx::SqlitePool,
  user_id: &str
) -> Result<Vec<Notification>, AppError> {
  sqlx::query_as!(
    Notification,
    "SELECT id as \"id!\", user_id, event_type, payload, created_at, delivered_at
     FROM notifications_queue
     WHERE user_id = ? AND delivered_at IS NULL",
    user_id
  ).fetch_all(pool).await.map_err(|_| AppError::DatabaseError)
}

pub async fn mark_notification_as_delivered(
  pool: &sqlx::SqlitePool,
  id: &str
) -> Result<(), AppError> {
  sqlx::query!(
    "UPDATE notifications_queue SET delivered_at = CURRENT_TIMESTAMP WHERE id = ?",
    id
  ).execute(pool).await.map_err(|_| AppError::DatabaseError)?;
  Ok(())
}