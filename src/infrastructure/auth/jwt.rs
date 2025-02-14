use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use crate::application::dtos::auth_dto::AuthResponse;
use crate::utils::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_token(user_id: &str) -> Result<AuthResponse, AppError> {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            env::var("JWT_SECRET")
                .map_err(|_| AppError::AuthError("JWT_SECRET not set".to_string()))?
                .as_bytes(),
        ),
    )
    .map_err(|e| AppError::AuthError(format!("Token generation error: {}", e)))?;

    Ok(AuthResponse { token })
}