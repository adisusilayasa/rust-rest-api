use actix_web::{post, web, HttpResponse, Result};
use validator::Validate;
use log::{error, warn};

use crate::application::dtos::auth_dto::{LoginRequest, RegisterRequest};
use crate::application::services::auth_application_service::AuthApplicationService;
use crate::infrastructure::persistence::postgres_user_repository::PostgresUserRepository;
use crate::utils::error::AppError;

#[post("/register")]
pub async fn register(
    auth_service: web::Data<AuthApplicationService<PostgresUserRepository>>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    if let Err(errors) = req.validate() {
        warn!("Validation failed for registration request: {:?}", errors);
        return Err(AppError::ValidationError(format!("{:?}", errors)));
    }

    match auth_service.register(req.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => {
            error!("Registration failed: {}", e);
            Err(e)
        }
    }
}

#[post("/login")]
pub async fn login(
    auth_service: web::Data<AuthApplicationService<PostgresUserRepository>>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    match auth_service.login(req.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => {
            error!("Login failed: {}", e);
            Err(e)
        }
    }
}