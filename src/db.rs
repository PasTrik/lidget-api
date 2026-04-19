use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub async fn connect(database_url: &str) -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await.expect("Failed to connect to database");

    sqlx::migrate!("./migrations/")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    pool
}