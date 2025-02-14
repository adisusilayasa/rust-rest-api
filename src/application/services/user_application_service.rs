use uuid::Uuid;
use crate::domain::repositories::user_repository::UserRepository;
use crate::application::dtos::user_dto::UserProfileResponse;
use crate::utils::error::AppError;

pub struct UserApplicationService<T: UserRepository> {
    user_repository: T,
}

impl<T: UserRepository> UserApplicationService<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn get_profile(&self, user_id: &str) -> Result<UserProfileResponse, AppError> {
        let uuid = Uuid::parse_str(user_id)
            .map_err(|e| AppError::ValidationError(format!("Invalid user ID: {}", e)))?;

        let user = self.user_repository.find_by_id(&uuid).await
            .map_err(|e| AppError::DatabaseError(sqlx::Error::Protocol(e.to_string())))?
            .ok_or_else(|| AppError::NotFoundError(format!("User with ID {} not found", uuid)))?;

        Ok(UserProfileResponse::from(user))
    }
}