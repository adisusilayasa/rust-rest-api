use actix_web::{get, put, web, HttpRequest};
use actix_web::HttpResponse;
use sqlx::PgPool;
use crate::utils::auth::Claims;
use crate::utils::error::AppError;
use crate::utils::response::{Response, ResponseBuilder};
use crate::domains::user::service::{update_user_profile, get_user_profile};
use crate::domains::user::entity::UpdateProfileRequest;
use actix_web::HttpMessage;
use log::{error, warn};

#[get("/profile")]
pub async fn handle_get_profile(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .ok_or_else(|| {
            error!("Failed to get user claims from request");
            AppError::AuthenticationError("Session expired or invalid".to_string())
        })?;

    match get_user_profile(pool.get_ref(), &claims.sub).await {
        Ok(profile) => Ok(Response::ok(profile)),
        Err(AppError::ValidationError(e)) => {
            warn!("Validation error: {}", e);
            Ok(Response::bad_request(&e))
        },
        Err(AppError::NotFoundError(e)) => {
            warn!("User not found: {}", e);
            Ok(Response::not_found(&e))
        },
        Err(AppError::DatabaseError(e)) => {
            error!("Database error while fetching profile: {}", e);
            Ok(Response::internal_error("Failed to fetch user profile"))
        },
        Err(AppError::AuthenticationError(e)) => {
            warn!("Authentication error: {}", e);
            Ok(Response::unauthorized(&e))
        },
        Err(e) => {
            error!("Unexpected error while fetching profile: {}", e);
            Ok(Response::internal_error("An unexpected error occurred"))
        }
    }
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
        .ok_or_else(|| {
            error!("Failed to get user claims from request");
            AppError::AuthenticationError("Session expired or invalid".to_string())
        })?;

    match update_user_profile(pool.get_ref(), &claims.sub, &update_data.0).await {
        Ok(profile) => Ok(Response::ok(profile)),
        Err(AppError::ValidationError(e)) => {
            warn!("Validation error: {}", e);
            Ok(Response::bad_request(&e))
        },
        Err(AppError::NotFoundError(e)) => {
            warn!("User not found: {}", e);
            Ok(Response::not_found(&e))
        },
        Err(AppError::DatabaseError(e)) => {
            error!("Database error while updating profile: {}", e);
            Ok(Response::internal_error("Failed to update user profile"))
        },
        Err(AppError::AuthenticationError(e)) => {
            warn!("Authentication error: {}", e);
            Ok(Response::unauthorized(&e))
        },
        Err(e) => {
            error!("Unexpected error while updating profile: {}", e);
            Ok(Response::internal_error("An unexpected error occurred"))
        }
    }
}
