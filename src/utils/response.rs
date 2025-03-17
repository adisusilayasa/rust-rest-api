use actix_web::{HttpResponse, http::StatusCode};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    status: String,
    code: u16,
    message: Option<String>,
    data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: "success".to_string(),
            code: StatusCode::OK.as_u16(),
            message: None,
            data: Some(data),
        }
    }

    pub fn error(status: StatusCode, message: &str) -> ApiResponse<()> {
        ApiResponse {
            status: "error".to_string(),
            code: status.as_u16(),
            message: Some(message.to_string()),
            data: None,
        }
    }

    pub fn error_with_data(status: StatusCode, message: &str, data: T) -> Self {
        Self {
            status: "error".to_string(),
            code: status.as_u16(),
            message: Some(message.to_string()),
            data: Some(data),
        }
    }

    pub fn into_response(self) -> HttpResponse {
        HttpResponse::build(StatusCode::from_u16(self.code).unwrap())
            .json(self)
    }
}

pub trait ResponseBuilder {
    fn ok<T: Serialize>(data: T) -> HttpResponse {
        ApiResponse::success(data).into_response()
    }

    fn created<T: Serialize>(data: T) -> HttpResponse {
        ApiResponse {
            status: "success".to_string(),
            code: StatusCode::CREATED.as_u16(),
            message: None,
            data: Some(data),
        }.into_response()
    }

    fn bad_request(message: &str) -> HttpResponse {
        ApiResponse::<()>::error(StatusCode::BAD_REQUEST, message).into_response()
    }

    fn bad_request_with_data<T: Serialize>(message: &str, data: T) -> HttpResponse {
        ApiResponse::error_with_data(StatusCode::BAD_REQUEST, message, data).into_response()
    }

    fn unauthorized(message: &str) -> HttpResponse {
        ApiResponse::<()>::error(StatusCode::UNAUTHORIZED, message).into_response()
    }

    fn forbidden(message: &str) -> HttpResponse {
        ApiResponse::<()>::error(StatusCode::FORBIDDEN, message).into_response()
    }

    fn not_found(message: &str) -> HttpResponse {
        ApiResponse::<()>::error(StatusCode::NOT_FOUND, message).into_response()
    }

    fn internal_error(message: &str) -> HttpResponse {
        ApiResponse::<()>::error(StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}

pub struct Response;
impl ResponseBuilder for Response {}