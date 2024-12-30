use actix_web::{get, web, HttpResponse};
use sqlx::{PgPool, FromRow};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::json;

pub mod schema;
use crate::utils::logo::get_institution_logo;
use crate::error::AppResult;
use crate::routes::accounts::schema::{AccountQuery, Balance, EnrichedAccount, Institution};

#[derive(Debug, FromRow)]
struct Account {
    id: i32,
    connection_id: String,
    name: String,
    account_type: String,
    currency: String,
    current_balance: f64,
    available_balance: f64,
    credit_limit: f64,
    last_sync: NaiveDateTime,
    institution_id: i32,
    institution_name: String,
}

#[get("/api/v1/accounts")]
pub async fn get_accounts(
    pool: web::Data<PgPool>,
    query: web::Query<AccountQuery>,
) -> AppResult<HttpResponse> {
    let page = i64::from(query.page);
    let per_page = i64::from(query.limit.min(100));
    let offset = (page - 1) * per_page;

    let mut sql = "SELECT a.*, i.id as institution_id, i.name as institution_name 
         FROM accounts a 
         LEFT JOIN institutions i ON a.institution_id = i.id 
         WHERE 1=1".to_string();
    let mut params = vec![];

    if let Some(connection_id) = &query.connection_id {
        sql.push_str(" AND a.connection_id = $1");
        params.push(connection_id);
    }

    if let Some(account_type) = &query.account_type {
        sql.push_str(" AND a.account_type = $2");
        params.push(account_type);
    }

    if let Some(currency) = &query.currency {
        sql.push_str(" AND a.currency = $3");
        params.push(currency);
    }

    // Count total records
    let count_sql = format!("SELECT COUNT(*) FROM ({}) as count_query", sql);
    let total = sqlx::query_scalar::<_, i64>(&count_sql)
        .bind(query.connection_id.as_deref())
        .bind(query.account_type.as_deref())
        .bind(query.currency.as_deref())
        .fetch_one(pool.get_ref())
        .await?;

    // Add pagination
    sql.push_str(" ORDER BY a.id LIMIT $4 OFFSET $5");

    let accounts = sqlx::query_as::<_, Account>(&sql)
        .bind(query.connection_id.as_deref())
        .bind(query.account_type.as_deref())
        .bind(query.currency.as_deref())
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await?;

    let enriched_accounts: Vec<EnrichedAccount> = accounts
        .into_iter()
        .map(|account| EnrichedAccount {
            id: account.id.to_string(),
            connection_id: account.connection_id,
            name: account.name,
            account_type: account.account_type,
            currency: account.currency,
            balance: Balance {
                current: account.current_balance,
                available: Some(account.available_balance),
                limit: Some(account.credit_limit),
            },
            institution: Institution {
                id: account.institution_id.to_string(),
                name: account.institution_name,
                logo_url: get_institution_logo(&account.institution_id.to_string()).map(|s| s.to_string()),
            },
            last_sync: Some(DateTime::<Utc>::from_naive_utc_and_offset(account.last_sync, Utc)),
        })
        .collect();

    let total_pages = (total as f64 / per_page as f64).ceil() as i64;
    let has_more = page < total_pages;

    Ok(HttpResponse::Ok().json(json!({
        "data": enriched_accounts,
        "page": page,
        "total_pages": total_pages,
        "total_items": total,
        "has_more": has_more,
    })))
}
