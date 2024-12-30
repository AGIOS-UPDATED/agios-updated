use actix_web::{get, post, web, HttpResponse};
use sqlx::{PgPool, FromRow};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::{
    error::AppError,
    providers::ProviderFactory,
    utils::get_institution_logo,
};

#[derive(Serialize, FromRow)]
pub struct Connection {
    id: String,
    institution_id: String,
    institution_name: String,
    status: String,
    last_sync: Option<NaiveDateTime>,
}

#[derive(Serialize, FromRow)]
pub struct Institution {
    id: String,
    name: String,
    logo_url: Option<String>,
    website_url: Option<String>,
    primary_color: Option<String>,
    country: String,
    provider: String,
    oauth_support: bool,
    last_update: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct InstitutionsResponse {
    institutions: Vec<Institution>,
    total: i64,
    page: i64,
    per_page: i64,
}

#[derive(Deserialize)]
pub struct InstitutionQuery {
    page: Option<i64>,
    per_page: Option<i64>,
    country: Option<String>,
    provider: Option<String>,
    oauth_only: Option<bool>,
}

#[get("/institutions")]
pub async fn get_institutions(
    query: web::Query<InstitutionQuery>,
    db: web::Data<PgPool>,
    _provider_factory: web::Data<ProviderFactory>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10).min(100);
    let offset = ((page - 1) * per_page) as i64;

    // Build base query
    let mut sql_query = sqlx::QueryBuilder::new(
        "SELECT * FROM institutions WHERE 1=1",
    );

    // Add filters
    if let Some(country) = &query.country {
        sql_query.push(" AND country = ");
        sql_query.push_bind(country);
    }

    if let Some(provider) = &query.provider {
        sql_query.push(" AND provider = ");
        sql_query.push_bind(provider);
    }

    if let Some(true) = query.oauth_only {
        sql_query.push(" AND oauth_support = true");
    }

    // Get total count
    let count_sql = format!("SELECT COUNT(*) FROM ({}) AS t", sql_query.sql());
    let total: i64 = sqlx::query_scalar(&count_sql)
        .fetch_one(&**db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;


    // Add pagination
    sql_query.push(" ORDER BY name ASC LIMIT ");
    sql_query.push_bind(per_page);
    sql_query.push(" OFFSET ");
    sql_query.push_bind(offset);

    let mut institutions: Vec<Institution> = sql_query
        .build_query_as::<Institution>()
        .fetch_all(&**db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;


    // Update logo URLs
    for institution in &mut institutions {
        if let Some(logo) = get_institution_logo(&institution.id) {
            institution.logo_url = Some(logo.to_string());
        }
    }

    Ok(HttpResponse::Ok().json(InstitutionsResponse {
        institutions,
        total,
        page,
        per_page,
    }))
}

#[get("/institutions/{id}")]
pub async fn get_institution(
    path: web::Path<String>,
    db: web::Data<PgPool>,
    _provider_factory: web::Data<ProviderFactory>,
) -> Result<HttpResponse, AppError> {
    let mut institution = sqlx::query_as::<_, Institution>(
        "SELECT * FROM institutions WHERE id = $1",
    )
    .bind(path.into_inner())
    .fetch_optional(&**db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
    .ok_or_else(|| AppError::NotFound("Institution not found".to_string()))?;

    // Update logo URL
    if let Some(logo) = get_institution_logo(&institution.id) {
        institution.logo_url = Some(logo.to_string());
    }

    Ok(HttpResponse::Ok().json(institution))
}

#[post("/institutions/{id}/usage")]
pub async fn update_institution_usage(
    path: web::Path<String>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    sqlx::query(
        "INSERT INTO institution_usage (institution_id, action) VALUES ($1, 'view')",
    )
    .bind(path.into_inner())
    .execute(&**db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(HttpResponse::NoContent().finish())
}
