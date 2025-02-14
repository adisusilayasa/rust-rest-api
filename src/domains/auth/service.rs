use bcrypt::{hash, DEFAULT_COST};
use crate::domains::user::repository::UserRepository;
use crate::domains::user::entity::User;
use crate::shared::auth;
use crate::shared::error::AppError;

pub struct AuthService<T: UserRepository> {
    user_repository: T,
}

impl<T: UserRepository> AuthService<T> {
    pub fn new(user_repository: T, _jwt_secret: String) -> Self {
        Self { user_repository }
    }

    pub async fn register(&self, email: String, password: String) -> Result<User, AppError> {
        if let Ok(Some(_)) = self.user_repository.find_by_email(&email).await {
            return Err(AppError::ValidationError("Email already exists".to_string()));
        }

        let password_hash = hash(password.as_bytes(), DEFAULT_COST)
            .map_err(|e| AppError::InternalError(format!("Password hashing error: {}", e)))?;

        let user = User::new(email, password_hash);
        self.user_repository.create(&user).await
            .map_err(|e| AppError::DatabaseError(sqlx::Error::Protocol(e)))
    }

    pub async fn login(&self, email: String, password: String) -> Result<String, AppError> {
        let user = self.user_repository.find_by_email(&email).await
            .map_err(|e| AppError::DatabaseError(sqlx::Error::Protocol(e)))?
            .ok_or_else(|| AppError::AuthenticationError("Invalid credentials".to_string()))?;

        if !user.verify_password(&password)
            .map_err(|e| AppError::InternalError(e))? {
            return Err(AppError::AuthenticationError("Invalid credentials".to_string()));
        }

        auth::generate_token(user.id)
    }
}