use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use derive_more::Display;
use sqlx;
use log::error;
use serde_json::json;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Internal error: {}", _0)]
    InternalError(String),
    
    #[display(fmt = "Validation error: {}", _0)]
    ValidationError(String),
    
    #[display(fmt = "Authentication error: {}", _0)]
    AuthenticationError(String),
    
    #[display(fmt = "Not found: {}", _0)]
    NotFoundError(String),
    
    #[display(fmt = "Database error")]
    DatabaseError(sqlx::Error),
    
    #[display(fmt = "Rate limit exceeded: {}", _0)]
    RateLimitExceeded(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            AppError::NotFoundError(_) => StatusCode::NOT_FOUND,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::RateLimitExceeded(_) => StatusCode::TOO_MANY_REQUESTS,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(e) => {
                error!("Database error: {}", e);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({
                        "status": "error",
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        "message": "A database error occurred",
                        "data": null
                    }))
            },
            AppError::InternalError(msg) => {
                error!("Internal server error: {}", msg);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(json!({
                        "status": "error",
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        "message": msg,
                        "data": null
                    }))
            },
            AppError::ValidationError(msg) => {
                HttpResponse::build(StatusCode::BAD_REQUEST)
                    .json(json!({
                        "status": "error",
                        "code": StatusCode::BAD_REQUEST.as_u16(),
                        "message": msg,
                        "data": null
                    }))
            },
            AppError::AuthenticationError(msg) => {
                HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .json(json!({
                        "status": "error",
                        "code": StatusCode::UNAUTHORIZED.as_u16(),
                        "message": msg,
                        "data": null
                    }))
            },
            AppError::NotFoundError(msg) => {
                HttpResponse::build(StatusCode::NOT_FOUND)
                    .json(json!({
                        "status": "error",
                        "code": StatusCode::NOT_FOUND.as_u16(),
                        "message": msg,
                        "data": null
                    }))
            },
            AppError::RateLimitExceeded(msg) => {
                HttpResponse::build(StatusCode::TOO_MANY_REQUESTS)
                    .json(json!({
                        "status": "error",
                        "code": StatusCode::TOO_MANY_REQUESTS.as_u16(),
                        "message": msg,
                        "data": {
                            "retry_after": 300
                        }
                    }))
            },
        }
    }
}

// Helper methods for easier error creation
impl AppError {
    pub fn internal<T: ToString>(message: T) -> Self {
        AppError::InternalError(message.to_string())
    }
    
    pub fn validation<T: ToString>(message: T) -> Self {
        AppError::ValidationError(message.to_string())
    }
    
    pub fn authentication<T: ToString>(message: T) -> Self {
        AppError::AuthenticationError(message.to_string())
    }
    
    pub fn not_found<T: ToString>(message: T) -> Self {
        AppError::NotFoundError(message.to_string())
    }
    
    pub fn rate_limited<T: ToString>(message: T) -> Self {
        AppError::RateLimitExceeded(message.to_string())
    }
}