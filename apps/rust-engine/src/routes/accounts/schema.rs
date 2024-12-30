use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, ToSchema)]
pub struct AccountQuery {
    pub connection_id: Option<String>,
    pub account_type: Option<String>,
    pub currency: Option<String>,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 {
    1
}

fn default_limit() -> u32 {
    20
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AccountsResponse {
    pub data: Vec<EnrichedAccount>,
    pub page: u32,
    pub total_pages: u32,
    pub total_items: u64,
    pub has_more: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EnrichedAccount {
    pub id: String,
    pub connection_id: String,
    pub name: String,
    pub account_type: String,
    pub currency: String,
    pub balance: Balance,
    pub institution: Institution,
    pub last_sync: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Balance {
    pub current: f64,
    pub available: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Institution {
    pub id: String,
    pub name: String,
    pub logo_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AccountStats {
    pub total_accounts: u64,
    pub total_balance: f64,
    pub by_type: Vec<TypeStat>,
    pub by_currency: Vec<CurrencyStat>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TypeStat {
    pub account_type: String,
    pub count: u64,
    pub total_balance: f64,
    pub percentage: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrencyStat {
    pub currency: String,
    pub count: u64,
    pub total_balance: f64,
    pub percentage: f64,
}
