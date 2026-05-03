use crate::config::Config;
use sqlx::SqlitePool;
use crate::ws::registry::Registry;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
    pub ws_registry: Registry,
}

impl AppState {
    pub fn new(pool: SqlitePool, config: Config, ws_registry: Registry) -> Self {
        Self { db: pool, config, ws_registry }
    }
}