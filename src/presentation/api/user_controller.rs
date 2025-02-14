use actix_web::{get, web, HttpResponse, Result};
use crate::application::services::user_application_service::UserApplicationService;
use crate::infrastructure::persistence::postgres_user_repository::PostgresUserRepository;
use crate::infrastructure::auth::middleware::Claims;
use crate::utils::error::AppError;

#[get("/profile")]
pub async fn get_profile(
    user_service: web::Data<UserApplicationService<PostgresUserRepository>>,
    claims: web::ReqData<Claims>,
) -> Result<HttpResponse, AppError> {
    match user_service.get_profile(&claims.sub).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            log::error!("Failed to get user profile: {}", e);
            Err(e)
        }
    }
}