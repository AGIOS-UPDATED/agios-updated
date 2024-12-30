use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DbTransaction {
    pub id: String,
    pub account_id: String,
    pub amount: f64,
    pub currency: String,
    pub date: DateTime<Utc>,
    pub status: String,
    pub balance: Option<f64>,
    pub category: Option<String>,
    pub method: String,
    pub name: String,
    pub description: Option<String>,
    pub currency_rate: Option<f64>,
    pub currency_source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DbInstitution {
    pub id: String,
    pub name: String,
    pub logo: Option<String>,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DbAccount {
    pub id: String,
    pub name: String,
    pub currency: String,
    pub account_type: String,
    pub institution_id: String,
    pub balance_amount: f64,
    pub balance_currency: String,
    pub enrollment_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DbConnection {
    pub id: String,
    pub institution_id: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
