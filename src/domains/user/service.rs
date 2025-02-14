use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;
use crate::domains::user::repository::{find_user_by_id, update_user};
use crate::domains::user::dto::{UserProfileResponse, create_user_profile_response};
use crate::utils::error::AppError;
use super::entity::{User, UpdateProfileRequest};

pub async fn get_user_profile(pool: &PgPool, user_id: &str) -> Result<UserProfileResponse, AppError> {
    let uuid = Uuid::parse_str(user_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid user ID: {}", e)))?;

    let user = match find_user_by_id(pool, &uuid).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::NotFoundError(format!("User with ID {} not found", uuid))),
        Err(e) => return Err(AppError::DatabaseError(sqlx::Error::Protocol(e))),
    };

    Ok(create_user_profile_response(user))
}

pub async fn update_user_profile(
    pool: &PgPool,
    user_id: &str,
    update_data: &UpdateProfileRequest
) -> Result<UserProfileResponse, AppError> {
    let uuid = Uuid::parse_str(user_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid user ID: {}", e)))?;

    let current_user = match find_user_by_id(pool, &uuid).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::NotFoundError(format!("User with ID {} not found", uuid))),
        Err(e) => return Err(AppError::DatabaseError(sqlx::Error::Protocol(e))),
    };

    let updated_user = User {
        id: current_user.id,
        email: update_data.email.clone().unwrap_or(current_user.email),
        name: update_data.name.clone(),
        phone: update_data.phone.clone(),
        address: update_data.address.clone(),
        password_hash: current_user.password_hash,
        created_at: current_user.created_at,
        updated_at: Some(Utc::now()),
        deleted_at: update_data.deleted_at,
    };

    let result = match update_user(pool, &updated_user).await {
        Ok(user) => user,
        Err(e) => return Err(AppError::DatabaseError(sqlx::Error::Protocol(e))),
    };

    Ok(create_user_profile_response(result))
}