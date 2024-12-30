use serde::{Deserialize, Serialize};
use crate::providers::types::{Account, Institution, Transaction};

#[derive(Debug, Deserialize)]
pub struct AccountQuery {
    pub connection_id: Option<String>,
    pub account_id: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct AccountsResponse {
    pub accounts: Vec<Account>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub redirect_uri: String,
    pub institution_id: String,
    pub provider: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i32,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ConnectionQuery {
    pub connection_id: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct ConnectionsResponse {
    pub connections: Vec<Connection>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Serialize)]
pub struct Connection {
    pub id: String,
    pub institution_id: String,
    pub provider: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionQuery {
    pub account_id: Option<String>,
    pub connection_id: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct TransactionsResponse {
    pub transactions: Vec<Transaction>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}
