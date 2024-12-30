use actix_web::{delete, get, web, HttpResponse};
use sqlx::{PgPool, FromRow};
use serde::Serialize;
use chrono::NaiveDateTime;

use crate::{
    error::AppError,
    providers::ProviderFactory,
};

#[derive(Serialize, FromRow)]
pub struct Connection {
    id: String,
    institution_id: String,
    institution_name: String,
    status: String,
    last_sync: Option<NaiveDateTime>,
}


// #[derive(Serialize, FromRow)]
// pub struct Connection {
//     id: String,
//     institution_id: String,
//     institution_name: String,
//     status: String,
//     last_sync: Option<NaiveDateTime>,
// }

#[derive(Serialize)]
pub struct ConnectionsResponse {
    connections: Vec<Connection>,
    total: i64,
    page: i64,
    per_page: i64,
}

#[get("/connections")]
pub async fn get_connections(
    db: web::Data<PgPool>,
    _provider_factory: web::Data<ProviderFactory>,
) -> Result<HttpResponse, AppError> {
    let connections: Vec<Connection> = sqlx::query_as::<_, Connection>(
        r#"
        SELECT 
            c.id,
            c.institution_id,
            i.name as institution_name,
            c.status,
            c.last_sync
        FROM connections c
        JOIN institutions i ON i.id = c.institution_id
        ORDER BY c.last_sync DESC
        "#,
    )
    .fetch_all(&**db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;


    let total = connections.len() as i64;

    Ok(HttpResponse::Ok().json(ConnectionsResponse {
        connections,
        total,
        page: 1,
        per_page: total,
    }))
}

#[delete("/connections/{id}")]
pub async fn delete_connection(
    path: web::Path<String>,
    db: web::Data<PgPool>,
    _provider_factory: web::Data<ProviderFactory>,
) -> Result<HttpResponse, AppError> {
    let connection_id = path.into_inner();

    // Check if connection exists
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM connections WHERE id = $1)",
    )
    .bind(&connection_id)
    .fetch_one(&**db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    if !exists {
        return Err(AppError::NotFound("Connection not found".to_string()));
    }

    // Delete connection
    sqlx::query("DELETE FROM connections WHERE id = $1")
        .bind(&connection_id)
        .execute(&**db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(HttpResponse::NoContent().finish())
}
