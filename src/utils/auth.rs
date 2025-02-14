use bcrypt::verify;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};
use uuid::Uuid;
use std::env;
use crate::{domains::user::entity::User, utils::error::AppError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn generate_token(user_id: Uuid) -> Result<String, AppError> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let now = Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (now + chrono::Duration::hours(24)).timestamp(),
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalError(format!("Token generation error: {}", e)))
}

pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
    .map_err(|_| AppError::AuthenticationError("Invalid token".to_string()))
}

pub fn verify_user_password(user: &User, password: &str) -> Result<bool, String> {
    verify(password, &user.password_hash)
        .map_err(|e| format!("Password verification error: {}", e))
}