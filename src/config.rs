#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().expect("Failed to load .env file");
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set").parse().expect("SERVER_PORT must be a number");
        Self { database_url, jwt_secret, server_port }
    }
}