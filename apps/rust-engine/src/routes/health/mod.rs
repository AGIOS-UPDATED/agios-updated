use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use redis::aio::Connection;
use serde::Serialize;
use log;

use crate::error::AppError;

#[derive(Serialize)]
pub struct HealthStatus {
    status: String,
    database: bool,
    cache: bool,
}

/// Health check
///
/// Check the health status of the API and its dependencies
#[get("/health")]
pub async fn health_check(
    db: web::Data<PgPool>,
    redis: web::Data<redis::Client>,
) -> Result<HttpResponse, AppError> {
    let mut status = HealthStatus {
        status: "ok".to_string(),
        database: false,
        cache: false,
    };

    // Check database connection
    if let Err(e) = sqlx::query("SELECT 1").execute(&**db).await {
        status.status = "error".to_string();
        log::error!("Database health check failed: {}", e);
    } else {
        status.database = true;
    }

    // Check Redis connection
    match redis.get_async_connection().await {
        Ok(mut conn) => {
            if let Err(e) = redis::cmd("PING").query_async::<_, String>(&mut conn).await {
                status.status = "error".to_string();
                log::error!("Redis health check failed: {}", e);
            } else {
                status.cache = true;
            }
        }
        Err(e) => {
            status.status = "error".to_string();
            log::error!("Redis connection failed: {}", e);
        }
    }

    Ok(HttpResponse::Ok().json(status))
}
