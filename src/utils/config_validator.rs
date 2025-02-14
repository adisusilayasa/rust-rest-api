use std::env;

pub fn validate_required_env_vars() -> Result<(), String> {
    let required_vars = vec![
        "DATABASE_URL",
        "JWT_SECRET",
        "PORT",
        "RUST_LOG",
    ];

    for var in required_vars {
        if env::var(var).is_err() {
            return Err(format!("Missing required environment variable: {}", var));
        }
    }

    // Validate PORT is a valid number
    if let Ok(port_str) = env::var("PORT") {
        if port_str.parse::<u16>().is_err() {
            return Err("PORT must be a valid number between 0 and 65535".to_string());
        }
    }

    // Validate DATABASE_URL format
    if let Ok(db_url) = env::var("DATABASE_URL") {
        if !db_url.starts_with("postgresql://") {
            return Err("DATABASE_URL must be a valid PostgreSQL connection string".to_string());
        }
    }

    Ok(())
}