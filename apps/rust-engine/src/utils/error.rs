use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    InternalServerError(String),
    ServiceUnavailable(String),
    External(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            ApiError::ServiceUnavailable(msg) => write!(f, "Service Unavailable: {}", msg),
            ApiError::External(msg) => write!(f, "External Error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorResponse::new(msg)),
            ApiError::Unauthorized(msg) => HttpResponse::Unauthorized().json(ErrorResponse::new(msg)),
            ApiError::NotFound(msg) => HttpResponse::NotFound().json(ErrorResponse::new(msg)),
            ApiError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse::new(msg))
            }
            ApiError::ServiceUnavailable(msg) => {
                HttpResponse::ServiceUnavailable().json(ErrorResponse::new(msg))
            }
            ApiError::External(msg) => {
                HttpResponse::ServiceUnavailable().json(ErrorResponse::new(msg))
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    fn new(msg: &str) -> Self {
        ErrorResponse {
            error: msg.to_string(),
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
