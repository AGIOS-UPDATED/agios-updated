use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct TransactionsQuery {
    pub account_id: Option<String>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub category: Option<String>,
    pub merchant: Option<String>,
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
pub struct TransactionsResponse {
    pub data: Vec<EnrichedTransaction>,
    pub page: u32,
    pub total_pages: u32,
    pub total_items: u64,
    pub has_more: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EnrichedTransaction {
    pub id: String,
    pub account_id: String,
    pub date: DateTime<Utc>,
    pub description: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub category: Option<String>,
    pub merchant: Option<String>,
    pub merchant_logo: Option<String>,
    pub pending: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionStats {
    pub total_transactions: u64,
    pub total_amount: f64,
    pub average_amount: f64,
    pub categories: Vec<CategoryStat>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CategoryStat {
    pub category: String,
    pub count: u64,
    pub total_amount: f64,
    pub percentage: f64,
}
