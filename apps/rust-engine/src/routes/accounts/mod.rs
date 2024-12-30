use actix_web::{get, web, HttpResponse};
use sqlx::{PgPool, postgres::PgRow, FromRow};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

pub mod schema; // This makes the schema module public
use crate::utils::logo::get_institution_logo;
use crate::utils::search::PaginatedResponse;
use crate::error::AppResult;
use crate::routes::accounts::schema::AccountQuery;

pub use schema::{EnrichedAccount,Institution};

use crate::{
    error::AppError,
    providers::ProviderFactory,
};

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct Balance {
    pub currency: String,
    pub amount: i64,
    pub limit : i64,
}


#[derive(Debug, FromRow)]
struct Account {
    id: i32,
    connection_id: String,
    name: String,
    account_type: String,
    currency: String,
    current_balance: i64,
    available_balance: i64,
    credit_limit: i64,
    last_sync: NaiveDateTime,
    institution_id: i32,
    institution_name: String,
}

/// Get accounts
///
/// Retrieve a paginated list of accounts with optional filtering
#[utoipa::path(
    get,
    path = "/api/v1/accounts",
    params(
        ("connection_id" = Option<String>, Query, description = "Filter by connection ID"),
        ("account_type" = Option<String>, Query, description = "Filter by account type"),
        ("currency" = Option<String>, Query, description = "Filter by currency"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of accounts", body = AccountsResponse),
        (status = 400, description = "Invalid request parameters"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/api/v1/accounts")]
pub async fn get_accounts(
    query: web::Query<AccountQuery>,
    db: web::Data<PgPool>,
    _provider_factory: web::Data<ProviderFactory>,
) -> AppResult<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10).min(100);
    let offset = ((page - 1) * per_page) as i64;

    // Build base query
    let mut sql_query = sqlx::QueryBuilder::new(
        r#"
        SELECT 
            a.id,
            a.connection_id,
            a.name,
            a.account_type,
            a.currency,
            a.current_balance,
            a.available_balance,
            a.credit_limit,
            a.last_sync,
            c.institution_id,
            i.name as institution_name
        FROM accounts a
        JOIN connections c ON c.id = a.connection_id
        JOIN institutions i ON i.id = c.institution_id
        "#,
    );

    // Add filters
    if let Some(connection_id) = &query.connection_id {
        sql_query.push(" WHERE a.connection_id = ");
        sql_query.push_bind(connection_id);
    }

    if let Some(account_type) = &query.account_type {
        sql_query.push(if query.connection_id.is_some() { " AND " } else { " WHERE " });
        sql_query.push("a.account_type = ");
        sql_query.push_bind(account_type);
    }

    if let Some(currency) = &query.currency {
        sql_query.push(if query.connection_id.is_some() || query.account_type.is_some() { " AND " } else { " WHERE " });
        sql_query.push("a.currency = ");
        sql_query.push_bind(currency);
    }

    // Get total count
    let count_sql = format!("SELECT COUNT(*) FROM ({}) AS t", sql_query.sql());
    let total: i64 = sqlx::query_scalar(&count_sql)
        .fetch_one(&**db)
        .await?;

    // Add pagination
    sql_query.push(" ORDER BY a.name ASC LIMIT ");
    sql_query.push_bind(per_page);
    sql_query.push(" OFFSET ");
    sql_query.push_bind(offset);

    let accounts: Vec<Account> = sql_query
        .build_query_as::<Account>()
        .fetch_all(&**db)
        .await?;

    // Transform into enriched accounts
    let enriched_accounts = accounts
        .into_iter()
        .map(|account| {
            let institution_id = account.institution_id.clone();
            EnrichedAccount {
                id: account.id.to_string(),
                connection_id: account.connection_id,
                name: account.name,
                account_type: account.account_type,
                currency: account.currency,
                balance: Balance {
                    current: account.current_balance,
                    available: account.available_balance,
                    limit: account.credit_limit,
                },
                institution: Institution {
                    id: institution_id,
                    name: account.institution_name,
                    logo_url: get_institution_logo(&institution_id),
                },
                last_sync: account.last_sync,
            }
        })
        .collect();

    // Create paginated response
    let response = PaginatedResponse::new(
        enriched_accounts,
        page,
        per_page,
        total as i64,
    );

    Ok(HttpResponse::Ok().json(response))
}
