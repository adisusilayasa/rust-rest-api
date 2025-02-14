use actix_web::{get, web, HttpResponse};
use log;
use crate::domains::user::service::UserService;
use crate::domains::user::repository::PostgresUserRepository;
use crate::shared::auth::Claims;
use crate::shared::error::AppError;
use actix_web::HttpMessage;

#[get("/profile")]
pub async fn get_profile(
    req: actix_web::HttpRequest,
    user_service: web::Data<UserService<PostgresUserRepository>>,
) -> Result<HttpResponse, AppError> {
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>()
        .ok_or_else(|| AppError::AuthenticationError("Invalid token".to_string()))?;

    match user_service.get_profile(&claims.sub).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            log::error!("Failed to get user profile: {}", e);
            Err(e)
        }
    }
}