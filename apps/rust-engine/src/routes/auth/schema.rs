use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AuthRequest {
    pub code: String,
    pub institution_id: String,
    pub provider: String,
    pub redirect_uri: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: i32,
    pub connection_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenInfo {
    pub token_type: String,
    pub scope: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
    pub connection_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiKeyResponse {
    pub api_key: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}
