use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Authorization failed: {0}")]
    Authorization(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("External error: {0}")]
    External(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub request_id: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let code = match self {
            AppError::Authentication(_) => "authentication_error",
            AppError::Authorization(_) => "authorization_error",
            AppError::BadRequest(_) => "bad_request",
            AppError::NotFound(_) => "not_found",
            AppError::Provider(_) => "provider_error",
            AppError::Cache(_) => "cache_error",
            AppError::Database(_) => "database_error",
            AppError::Internal(_) => "internal_error",
            AppError::External(_) => "external_error",
        };

        HttpResponse::build(status).json(ErrorResponse {
            code: code.to_string(),
            message: self.to_string(),
            request_id: Uuid::new_v4().to_string(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Authentication(_) => StatusCode::UNAUTHORIZED,
            AppError::Authorization(_) => StatusCode::FORBIDDEN,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Provider(_) => StatusCode::BAD_GATEWAY,
            AppError::Cache(_) 
            | AppError::Database(_) 
            | AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::External(_) => StatusCode::INTERNAL_SERVER_ERROR, // Added this match arm
        }
    }
}

// Implement From traits for common error types
impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::Cache(err.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Provider(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
