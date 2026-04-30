use crate::handlers::auth;
use crate::state::AppState;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use std::sync::Arc;

pub fn create(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/auth", auth_routes())
        .nest("/api/photos", photo_routes())
        .nest("/api/quizzes", quiz_routes())
        .nest("/api/events", event_routes())
        .with_state(state)
}

fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/join", post(auth::join))
        .route("/@me", get(auth::me))
}

fn photo_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(placeholder))
        .route("/", post(placeholder))
}

fn quiz_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(placeholder))
        .route("/history", get(placeholder))
        .route("/:id/answer", post(placeholder))
}

fn event_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(placeholder))
        .route("/", post(placeholder))
        .route("/:id", patch(placeholder))
        .route("/:id", delete(placeholder))
}

async fn placeholder() -> &'static str {
    "TODO"
}