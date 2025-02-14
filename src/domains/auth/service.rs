use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
use chrono::Utc;
use crate::domains::user::repository::{find_user_by_email, create_user};
use crate::domains::user::entity::User;
use crate::utils::auth;
use crate::utils::error::AppError;
use crate::utils::auth::verify_user_password;

pub async fn register_user(
    pool: &PgPool,
    email: String,
    password: String
) -> Result<User, AppError> {
    if let Ok(Some(_)) = find_user_by_email(pool, &email).await {
        return Err(AppError::ValidationError("Email already exists".to_string()));
    }

    let password_hash = hash(password.as_bytes(), DEFAULT_COST)
        .map_err(|e| AppError::InternalError(format!("Password hashing error: {}", e)))?;

    let user = User {
        id: uuid::Uuid::new_v4(),
        email,
        name: None,
        phone: None,
        address: None,
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
    email: String,
    password: String
) -> Result<String, AppError> {
    let user = match find_user_by_email(pool, &email).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::AuthenticationError("Invalid credentials".to_string())),
        Err(e) => return Err(AppError::DatabaseError(sqlx::Error::Protocol(e))),
    };

    if !verify_user_password(&user, &password)
        .map_err(|e| AppError::InternalError(e))? {
        return Err(AppError::AuthenticationError("Invalid credentials".to_string()));
    }

    auth::generate_token(user.id)
}