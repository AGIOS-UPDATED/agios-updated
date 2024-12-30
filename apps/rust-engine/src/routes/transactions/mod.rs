use actix_web::{get, web, HttpResponse};
use sqlx::{PgPool, FromRow};
use serde::Serialize;
use chrono::NaiveDate;

use crate::{
    error::AppError,
    providers::ProviderFactory,
};

#[derive(Serialize, FromRow)]
pub struct Transaction {
    id: String,
    account_id: String,
    amount: f64,
    currency: String,
    description: String,
    date: NaiveDate,
    account_name: Option<String>,
    account_type: Option<String>,
}

#[derive(Serialize)]
pub struct TransactionsResponse {
    transactions: Vec<Transaction>,
    total: i64,
    page: i64,
    per_page: i64,
}

#[derive(serde::Deserialize)]
pub struct TransactionQuery {
    page: Option<i64>,
    per_page: Option<i64>,
    account_id: Option<String>,
    connection_id: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

#[get("/transactions")]
pub async fn get_transactions(
    query: web::Query<TransactionQuery>,
    db: web::Data<PgPool>,
    _provider_factory: web::Data<ProviderFactory>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10).min(100);
    let offset = ((page - 1) * per_page) as i64;

    // Build base query
    let mut sql_query = sqlx::QueryBuilder::new(
        "SELECT t.*, a.name as account_name, a.account_type 
         FROM transactions t 
         LEFT JOIN accounts a ON t.account_id = a.id 
         WHERE 1=1",
    );

    // Add filters
    if let Some(account_id) = &query.account_id {
        sql_query.push(" AND t.account_id = ");
        sql_query.push_bind(account_id);
    }

    if let Some(connection_id) = &query.connection_id {
        sql_query.push(" AND t.connection_id = ");
        sql_query.push_bind(connection_id);
    }

    if let Some(start_date) = &query.start_date {
        sql_query.push(" AND t.date >= ");
        sql_query.push_bind(start_date);
    }

    if let Some(end_date) = &query.end_date {
        sql_query.push(" AND t.date <= ");
        sql_query.push_bind(end_date);
    }

    // Get total count
    let count_sql = format!("SELECT COUNT(*) FROM ({}) AS t", sql_query.sql());
    let total: i64 = sqlx::query_scalar(&count_sql)
        .fetch_one(&**db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // Add sorting and pagination
    sql_query.push(" ORDER BY t.date DESC LIMIT ");
    sql_query.push_bind(per_page);
    sql_query.push(" OFFSET ");
    sql_query.push_bind(offset);

    let transactions: Vec<Transaction> = sql_query
        .build_query_as::<Transaction>()
        .fetch_all(&**db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(HttpResponse::Ok().json(TransactionsResponse {
        transactions,
        total,
        page,
        per_page,
    }))
}
