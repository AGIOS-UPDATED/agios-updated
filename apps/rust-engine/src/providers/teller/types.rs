use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct TellerInstitution {
    pub id: String,
    pub name: String,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TellerAccount {
    pub id: String,
    pub currency: String,
    pub enrollment_id: String,
    pub institution: TellerInstitution,
    pub last_four: String,
    pub links: TellerLinks,
    pub name: String,
    pub r#type: String,
    pub subtype: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TellerLinks {
    pub balances: String,
    pub transactions: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TellerTransaction {
    pub id: String,
    pub account_id: String,
    pub amount: String,
    pub date: DateTime<Utc>,
    pub description: String,
    pub details: TellerTransactionDetails,
    pub running_balance: Option<String>,
    pub status: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TellerTransactionDetails {
    pub category: Option<String>,
    pub counterparty: Option<TellerCounterparty>,
    pub processing_status: String,
}
