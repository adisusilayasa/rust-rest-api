use bcrypt::{hash, DEFAULT_COST};
use crate::application::dtos::auth_dto::{AuthResponse, LoginRequest, RegisterRequest};
use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::services::auth_service::AuthService;
use crate::infrastructure::auth::jwt;
use crate::utils::error::AppError;

pub struct AuthApplicationService<T: UserRepository> {
    auth_service: AuthService<T>,
}

impl<T: UserRepository> AuthApplicationService<T> {
    pub fn new(user_repository: T) -> Self {
        Self {
            auth_service: AuthService::new(user_repository),
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse, AppError> {
        // Check if user already exists
        if let Ok(Some(_)) = self.auth_service.user_repository.find_by_email(&req.email).await {
            log::error!("Registration failed: Email {} already registered", req.email);
            return Err(AppError::ValidationError("Email already registered".to_string()));
        }

        let password_hash = hash(req.password.as_bytes(), DEFAULT_COST)
            .map_err(|e| {
                log::error!("Password hashing error: {}", e);
                AppError::AuthError(format!("Password hashing error: {}", e))
            })?;

        let user = User::new(req.email.clone(), password_hash);
        let created_user = self.auth_service.user_repository.create(&user).await
            .map_err(|e| {
                log::error!("Failed to create user: {}", e);
                AppError::DatabaseError(sqlx::Error::Protocol(e.to_string()))
            })?;

        jwt::generate_token(&created_user.id.to_string())
            .map_err(|e| {
                log::error!("Token generation error: {}", e);
                AppError::AuthError(e.to_string())
            })
    }

    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, AppError> {
        let user = self.auth_service.user_repository.find_by_email(&req.email).await
            .map_err(|e| AppError::DatabaseError(sqlx::Error::Protocol(e.to_string())))?
            .ok_or_else(|| AppError::AuthError("Invalid credentials".to_string()))?;

        if !user.verify_password(&req.password)
            .map_err(|e| AppError::AuthError(e))? {
            return Err(AppError::AuthError("Invalid credentials".to_string()));
        }

        jwt::generate_token(&user.id.to_string())
            .map_err(|e| AppError::AuthError(e.to_string()))
    }
}