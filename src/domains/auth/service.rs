use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
use chrono::Utc;
use log::{warn, info};
use crate::domains::user::repository::{find_user_by_email, create_user};
use crate::domains::user::entity::User;
use crate::utils::auth;
use crate::utils::error::AppError;
use crate::utils::auth::verify_user_password;
use crate::utils::rate_limiter::LOGIN_LIMITER;

pub async fn register_user(
    pool: &PgPool,
    email: String,
    name: Option<&str>,
    address: Option<&str>,
    phone: Option<&str>,
    password: &str
) -> Result<User, AppError> {
    // Check if email already exists
    if let Ok(Some(_)) = find_user_by_email(pool, &email).await {
        return Err(AppError::validation("Email already exists"));
    }

    let password_hash = hash(password.as_bytes(), DEFAULT_COST)
        .map_err(|e| AppError::internal(format!("Password hashing error: {}", e)))?;

    let user = User {
        id: uuid::Uuid::new_v4(),
        email,
        name: name.map(String::from),
        phone: phone.map(String::from),
        address: address.map(String::from),
        password_hash,
        created_at: Utc::now(),
        updated_at: None,
        deleted_at: None,
    };

    create_user(pool, &user).await
        .map_err(|e| AppError::DatabaseError(sqlx::Error::Protocol(e)))
}

pub async fn login_user(
    pool: &PgPool,
    email: &str,
    password: &str
) -> Result<String, AppError> {
    // Check rate limit before processing login
    LOGIN_LIMITER.check_rate_limit(email).await?;

    let user = match find_user_by_email(pool, email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            warn!("Login attempt with non-existent email: {}", email);
            return Err(AppError::authentication("Invalid credentials"));
        },
        Err(e) => return Err(AppError::internal(format!("Database error: {}", e))),
    };

    if !verify_user_password(&user, password)
        .map_err(|e| AppError::internal(e))? {
        warn!("Failed login attempt for user: {}", email);
        return Err(AppError::authentication("Invalid credentials"));
    }

    // Reset rate limit counter on successful login
    LOGIN_LIMITER.reset(email).await;
    info!("Successful login for user: {}", email);

    auth::generate_token(user.id)
        .map_err(|e| AppError::internal(e))
}