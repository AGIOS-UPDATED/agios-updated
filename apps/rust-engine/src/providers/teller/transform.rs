use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::providers::types::*;

pub fn transform_account(account: TellerAccount) -> Account {
    Account {
        id: format!("tel_{}", account.id),
        connection_id: account.connection_id,
        name: account.name,
        account_type: transform_account_type(account.r#type),
        currency: account.currency.unwrap_or_else(|| "USD".to_string()),
        balance: Balance {
            current: account.balance.current,
            available: Some(account.balance.available),
            limit: account.balance.limit,
        },
        last_sync: Some(Utc::now()),
    }
}

pub fn transform_transaction(transaction: TellerTransaction) -> Transaction {
    Transaction {
        id: format!("tel_{}", transaction.id),
        account_id: format!("tel_{}", transaction.account_id),
        date: DateTime::parse_from_rfc3339(&transaction.date)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        description: Some(transaction.description),
        amount: transaction.amount,
        currency: transaction.currency.unwrap_or_else(|| "USD".to_string()),
        category: transaction.category,
        merchant: transaction.merchant.map(|m| m.name),
        pending: transaction.status == "pending",
    }
}

pub fn transform_institution(institution_id: &str) -> Institution {
    Institution {
        id: format!("tel_{}", institution_id),
        name: "Teller Institution".to_string(),
        country: "US".to_string(),
        provider: "teller".to_string(),
        logo_url: None,
        primary_color: None,
        url: None,
        oauth_support: true,
        products: vec!["accounts".to_string(), "transactions".to_string()],
        last_update: Utc::now(),
    }
}

fn transform_account_type(account_type: String) -> String {
    match account_type.as_str() {
        "depository" => "checking",
        "credit" => "credit",
        _ => "other",
    }.to_string()
}

#[derive(Debug, Deserialize)]
pub struct TellerAccount {
    pub id: String,
    pub connection_id: String,
    pub name: String,
    pub r#type: String,
    pub currency: Option<String>,
    pub balance: TellerBalance,
}

#[derive(Debug, Deserialize)]
pub struct TellerBalance {
    pub current: f64,
    pub available: f64,
    pub limit: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct TellerTransaction {
    pub id: String,
    pub account_id: String,
    pub date: String,
    pub description: String,
    pub amount: f64,
    pub currency: Option<String>,
    pub status: String,
    pub category: Option<String>,
    pub merchant: Option<TellerMerchant>,
}

#[derive(Debug, Deserialize)]
pub struct TellerMerchant {
    pub name: String,
}
