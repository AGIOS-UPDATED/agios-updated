use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub request_id: String,
}

impl ErrorResponse {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            request_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn disconnected(message: impl Into<String>) -> Self {
        Self::new("disconnected", message)
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new("internal_server_error", message)
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Headers {
    pub authorization: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SuccessResponse<T> {
    pub data: T,
    pub request_id: String,
}

impl<T> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            request_id: Uuid::new_v4().to_string(),
        }
    }
}
