use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoCardlessInstitution {
    pub id: String,
    pub name: String,
    pub bic: Option<String>,
    pub transaction_total_days: i32,
    pub countries: Vec<String>,
    pub logo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoCardlessAccount {
    pub id: String,
    pub iban: String,
    pub institution_id: String,
    pub status: String,
    pub owner_name: String,
    pub product: Option<String>,
    pub balances: GoCardlessBalances,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoCardlessBalances {
    pub available: Option<f64>,
    pub current: f64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoCardlessTransaction {
    pub id: String,
    pub date: DateTime<Utc>,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub remittance_information_unstructured: Option<String>,
    pub creditor_name: Option<String>,
    pub debtor_name: Option<String>,
    pub transaction_type: Option<String>,
    pub booking_date: Option<DateTime<Utc>>,
    pub value_date: Option<DateTime<Utc>>,
}
