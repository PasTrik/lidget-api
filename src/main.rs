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

use std::sync::Arc;
use state::AppState;

#[tokio::main]
async fn main() {
    // 1. Config
    let config = config::Config::from_env();
    // 2. DB
    let pool = db::connect(&config.database_url).await;
    // 3. AppState
    let state = Arc::new(AppState::new(pool, config));
    // 4. Router
    let listener = tokio::net::TcpListener::bind(
        format!("0.0.0.0:{}", state.config.server_port)
    ).await.unwrap();

    let router = routes::create(state.clone());

    axum::serve(listener, router).await.expect("Failed to start server");

    // 5. Lancer le serveur
}
