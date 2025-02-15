use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;
use crate::domains::user::repository::{find_user_by_id, update_user};
use crate::domains::user::dto::{UserProfileResponse, create_user_profile_response};
use crate::utils::error::AppError;
use super::entity::{User, UpdateProfileRequest};
use regex::Regex;

// Add this function
pub async fn get_user_profile(pool: &PgPool, user_id: &str) -> Result<UserProfileResponse, AppError> {
    let uuid = Uuid::parse_str(user_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid user ID format: {}", e)))?;

    let user = match find_user_by_id(pool, &uuid).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(AppError::NotFoundError(
                format!("User profile not found for ID: {}", uuid)
            ))
        },
        Err(e) => {
            log::error!("Database error while fetching user: {}", e);
            return Err(AppError::DatabaseError(sqlx::Error::Protocol(e)))
        },
    };

    Ok(create_user_profile_response(user))
}

fn validate_email(email: &str) -> Result<(), AppError> {
    if !email.contains('@') || !email.contains('.') {
        return Err(AppError::ValidationError("Invalid email format".to_string()));
    }
    Ok(())
}

fn validate_phone(phone: &str) -> Result<(), AppError> {
    let phone_regex = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
    if !phone_regex.is_match(phone) {
        return Err(AppError::ValidationError("Invalid phone number format".to_string()));
    }
    Ok(())
}

pub async fn update_user_profile(
    pool: &PgPool,
    user_id: &str,
    update_data: &UpdateProfileRequest
) -> Result<UserProfileResponse, AppError> {
    // Validate user ID
    let uuid = Uuid::parse_str(user_id)
        .map_err(|e| AppError::ValidationError(format!("Invalid user ID format: {}", e)))?;

    // Validate email if provided
    if let Some(ref email) = update_data.email {
        validate_email(email)?;
    }

    // Validate phone if provided
    if let Some(ref phone) = update_data.phone {
        validate_phone(phone)?;
    }

    // Validate name if provided
    if let Some(ref name) = update_data.name {
        if name.trim().is_empty() {
            return Err(AppError::ValidationError("Name cannot be empty".to_string()));
        }
        if name.len() > 100 {
            return Err(AppError::ValidationError("Name is too long".to_string()));
        }
    }

    // Validate address if provided
    if let Some(ref address) = update_data.address {
        if address.trim().is_empty() {
            return Err(AppError::ValidationError("Address cannot be empty".to_string()));
        }
        if address.len() > 200 {
            return Err(AppError::ValidationError("Address is too long".to_string()));
        }
    }

    let current_user = match find_user_by_id(pool, &uuid).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(AppError::NotFoundError(
                format!("User profile not found for ID: {}", uuid)
            ))
        },
        Err(e) => {
            log::error!("Database error while fetching user: {}", e);
            return Err(AppError::DatabaseError(sqlx::Error::Protocol(e)))
        },
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