use std::env;

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn get_port() -> u16 {
    env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number")
}