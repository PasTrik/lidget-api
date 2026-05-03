use crate::handlers::{auth, quizzes};
use crate::handlers::quizzes::{find_quiz_by_id, find_quiz_history};
use crate::state::AppState;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use std::sync::Arc;
use crate::ws;

pub fn create(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/auth", auth_routes())
        .nest("/api/photos", photo_routes())
        .nest("/api/quizzes", quiz_routes())
        .nest("/api/events", event_routes())
        .route("/ws", get(ws::handler::handle))
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
    .route("/", get(quizzes::find_all_quizzes))
    .route("/", post(quizzes::create_quiz))
    .route("/available", get(quizzes::find_available_quizzes))
    .route("/history", get(quizzes::find_quiz_history))
    .route("/categories", get(quizzes::find_all_quizzes_categories))
    .route("/categories", post(quizzes::create_quiz_category))
    .route("/categories/:id", patch(quizzes::edit_quiz_category))
    .route("/categories/:id", delete(quizzes::delete_quiz_category))
    .route("/:id", get(quizzes::find_quiz_by_id))
    .route("/:id", patch(quizzes::edit_quiz))
    .route("/:id", delete(quizzes::delete_quiz))
    .route("/:id/questions", post(quizzes::create_quiz_question))
    .route("/:id/questions/:question_id", patch(quizzes::edit_quiz_question))
    .route("/:id/questions/:question_id", delete(quizzes::delete_quiz_question))
    .route("/:id/questions/:question_id/choices/text", post(quizzes::create_quiz_text_choice))
    .route("/:id/questions/:question_id/choices/image", post(quizzes::create_quiz_image_choice))
    .route("/:id/answer", post(quizzes::answer_quiz))
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