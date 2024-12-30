use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: AccountType,
    pub balance: Balance,
    pub currency: String,
    pub institution_id: String,
    pub last_sync: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountType {
    Checking,
    Savings,
    Credit,
    Investment,
    Loan,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub account_id: String,
    pub amount: f64,
    pub currency: String,
    pub date: DateTime<Utc>,
    pub description: String,
    pub merchant: Option<String>,
    pub category: Option<String>,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Posted,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Institution {
    pub id: String,
    pub name: String,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub status: ConnectionState,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConnectionState {
    Connected,
    Disconnected,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountsRequest {
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionsRequest {
    pub access_token: String,
    pub account_id: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountBalanceRequest {
    pub access_token: String,
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInstitutionsRequest {
    pub country: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetConnectionStatusRequest {
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteConnectionRequest {
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAccountsRequest {
    pub access_token: String,
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Provider {
    Teller,
    Plaid,
    Gocardless,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Provider::Teller => write!(f, "teller"),
            Provider::Plaid => write!(f, "plaid"),
            Provider::Gocardless => write!(f, "gocardless"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NewAccount {
    pub id: String,
    pub connection_id: String,
    pub name: String,
    pub account_type: String,
    pub currency: String,
    pub balance: Option<f64>,
    pub available_balance: Option<f64>,
    pub last_sync: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NewTransaction {
    pub id: String,
    pub account_id: String,
    pub date: DateTime<Utc>,
    pub description: String,
    pub amount: f64,
    pub currency: String,
    pub pending: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NewInstitution {
    pub id: String,
    pub name: String,
    pub logo: Option<String>,
    pub provider: String,
    pub country: String,
    pub primary_color: Option<String>,
    pub url: Option<String>,
    pub oauth_support: bool,
    pub products: Vec<String>,
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NewProvider {
    Plaid,
    Teller,
    GoCardless,
}

impl fmt::Display for NewProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NewProvider::Plaid => write!(f, "plaid"),
            NewProvider::Teller => write!(f, "teller"),
            NewProvider::GoCardless => write!(f, "gocardless"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GetAccountBalanceResponse {
    pub currency: String,
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub healthy: bool,
}

#[derive(Debug, Serialize)]
pub struct GetHealthCheckResponse {
    pub teller: HealthCheckResponse,
    pub gocardless: HealthCheckResponse,
    pub plaid: HealthCheckResponse,
}
