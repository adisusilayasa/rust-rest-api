use actix_web::{get, put, web, HttpRequest };
use actix_web::HttpResponse;
use sqlx::PgPool;
use crate::utils::auth::Claims;
use crate::utils::error::AppError;
use crate::domains::user::service::{get_user_profile, update_user_profile};
use crate::domains::user::entity::UpdateProfileRequest;
use actix_web::HttpMessage;

#[get("/profile")]
pub async fn handle_get_profile(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .ok_or_else(|| AppError::AuthenticationError("Invalid token".to_string()))?;

    let profile = get_user_profile(pool.get_ref(), &claims.sub).await?;
    Ok(HttpResponse::Ok().json(profile))
}

#[put("/profile")]
pub async fn handle_update_profile(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    update_data: web::Json<UpdateProfileRequest>,
) -> Result<HttpResponse, AppError> {
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .ok_or_else(|| AppError::AuthenticationError("Invalid token".to_string()))?;

    let profile = update_user_profile(
        pool.get_ref(),
        &claims.sub,
        &update_data.0
    ).await?;
    
    Ok(HttpResponse::Ok().json(profile))
}