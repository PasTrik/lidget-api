mod config;
mod db;
mod routes;
mod handlers;
mod models;
mod middleware;
mod events;
mod state;
mod errors;
mod jwt;
mod pagination;
pub mod upload;
pub mod ws;

use std::collections::HashMap;
use state::AppState;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
use crate::ws::registry::Registry;

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();
    let pool = db::connect(&config.database_url).await;
    let ws = Arc::new(RwLock::new(HashMap::new()));

    let state = Arc::new(AppState::new(pool, config, ws));

    let listener = tokio::net::TcpListener::bind(
        format!("0.0.0.0:{}", state.config.server_port)
    ).await.unwrap();

    let router = routes::create(state.clone())
      .nest_service("/uploads", ServeDir::new("uploads"));

    let router = routes::create(state.clone());

    axum::serve(listener, router).await.expect("Failed to start server");
}
