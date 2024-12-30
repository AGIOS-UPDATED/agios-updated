use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaidInstitution {
    pub institution_id: String,
    pub name: String,
    pub country_codes: Vec<String>,
    pub url: Option<String>,
    pub primary_color: Option<String>,
    pub logo: Option<String>,
    pub oauth: bool,
    pub products: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaidAccount {
    pub account_id: String,
    pub balances: PlaidBalances,
    pub mask: Option<String>,
    pub name: String,
    pub official_name: Option<String>,
    pub r#type: String,
    pub subtype: Option<String>,
    pub verification_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaidBalances {
    pub available: Option<f64>,
    pub current: f64,
    pub limit: Option<f64>,
    pub iso_currency_code: Option<String>,
    pub unofficial_currency_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaidTransaction {
    pub transaction_id: String,
    pub account_id: String,
    pub amount: f64,
    pub iso_currency_code: Option<String>,
    pub unofficial_currency_code: Option<String>,
    pub date: DateTime<Utc>,
    pub name: String,
    pub merchant_name: Option<String>,
    pub payment_channel: String,
    pub pending: bool,
    pub transaction_type: Option<String>,
    pub category: Option<Vec<String>>,
    pub location: Option<PlaidLocation>,
}
