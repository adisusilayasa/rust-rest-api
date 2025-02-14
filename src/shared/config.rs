use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL must be set")?,
            jwt_secret: std::env::var("JWT_SECRET")
                .map_err(|_| "JWT_SECRET must be set")?,
            server_addr: std::env::var("SERVER_ADDR")
                .unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
        })
    }
}