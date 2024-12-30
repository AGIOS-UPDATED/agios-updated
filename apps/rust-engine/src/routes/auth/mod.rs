use actix_web::{post, web, HttpResponse};
use sqlx::{PgPool, FromRow};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

use crate::{
    error::AppError,
    providers::ProviderFactory,
    utils::ApiError,
};

#[derive(Debug, Serialize, FromRow)]
pub struct Connection {
    pub id: String,
    pub provider: String,
    pub status: String,
    pub last_sync: Option<NaiveDateTime>,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeTokenRequest {
    pub provider: String,
    pub code: String,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub provider: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[post("/api/v1/auth/exchange")]
pub async fn exchange_token(
    request: web::Json<ExchangeTokenRequest>,
    db: web::Data<PgPool>,
    provider_factory: web::Data<ProviderFactory>,
) -> Result<HttpResponse, AppError> {
    let provider = provider_factory
        .get_provider(&request.provider)
        .ok_or(AppError::BadRequest("Invalid provider".to_string()))?;

    let (access_token, refresh_token) = provider
        .exchange_token(&request.code, &request.redirect_uri)
        .await
        .map_err(|e| AppError::External(e.to_string()))?;

    // Create connection record
    let connection = sqlx::query_as::<_, Connection>(
        r#"
        INSERT INTO connections (provider, status, access_token, refresh_token)
        VALUES ($1, 'active', $2, $3)
        RETURNING id, provider, status, last_sync, refresh_token
        "#,
    )
    .bind(&request.provider)
    .bind(&access_token)
    .bind(&refresh_token)
    .fetch_one(&**db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token,
        refresh_token,
    }))
}

#[post("/api/v1/auth/refresh")]
pub async fn refresh_token_handler(
    request: web::Json<RefreshTokenRequest>,
    db: web::Data<PgPool>,
    provider_factory: web::Data<ProviderFactory>,
) -> Result<HttpResponse, AppError> {
    // Get connection
    let connection = sqlx::query_as::<_, Connection>(
        r#"
        SELECT id, provider, status, last_sync, refresh_token
        FROM connections 
        WHERE provider = $1 AND refresh_token = $2
        "#,
    )
    .bind(&request.provider)
    .bind(&request.refresh_token)
    .fetch_optional(&**db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
    .ok_or(AppError::NotFound("Connection not found".to_string()))?;

    // Get provider
    let provider = provider_factory
        .get_provider(&connection.provider)
        .ok_or(AppError::BadRequest("Invalid provider".to_string()))?;

    // Refresh token
    let (new_access_token, new_refresh_token) = provider
        .refresh_token(&connection.refresh_token)
        .await
        .map_err(|e| AppError::External(e.to_string()))?;

    // Update connection
    sqlx::query(
        r#"
        UPDATE connections
        SET access_token = $1, refresh_token = $2
        WHERE id = $3
        "#,
    )
    .bind(&new_access_token)
    .bind(&new_refresh_token)
    .bind(&connection.id)
    .execute(&**db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
    }))
}
