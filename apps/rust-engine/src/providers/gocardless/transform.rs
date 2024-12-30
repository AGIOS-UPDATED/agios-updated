use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::providers::types::*;

pub fn transform_account(account: GoCardlessAccount) -> Account {
    Account {
        id: format!("gc_{}", account.id),
        connection_id: account.connection_id,
        name: account.name,
        account_type: transform_account_type(account.account_type),
        currency: account.currency,
        balance: Balance {
            current: account.balance.current,
            available: Some(account.balance.available),
            limit: account.balance.limit,
        },
        last_sync: Some(Utc::now()),
    }
}

pub fn transform_transaction(transaction: GoCardlessTransaction) -> Transaction {
    Transaction {
        id: format!("gc_{}", transaction.id),
        account_id: format!("gc_{}", transaction.account_id),
        date: transaction.date,
        description: Some(transaction.description),
        amount: transaction.amount,
        currency: transaction.currency,
        category: transaction.category,
        merchant: transaction.merchant_name,
        pending: transaction.status == "pending",
    }
}

pub fn transform_institution(institution: GoCardlessInstitution) -> Institution {
    Institution {
        id: format!("gc_{}", institution.id),
        name: institution.name,
        country: institution.country,
        provider: "gocardless".to_string(),
        logo_url: institution.logo_url,
        primary_color: institution.color,
        url: institution.url,
        oauth_support: institution.oauth_enabled,
        products: institution.capabilities,
        last_update: Utc::now(),
    }
}

fn transform_account_type(account_type: String) -> String {
    match account_type.as_str() {
        "current" => "checking",
        "savings" => "savings",
        "credit_card" => "credit",
        _ => "other",
    }.to_string()
}

#[derive(Debug, Deserialize)]
pub struct GoCardlessAccount {
    pub id: String,
    pub connection_id: String,
    pub name: String,
    pub account_type: String,
    pub currency: String,
    pub balance: GoCardlessBalance,
}

#[derive(Debug, Deserialize)]
pub struct GoCardlessBalance {
    pub current: f64,
    pub available: f64,
    pub limit: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct GoCardlessTransaction {
    pub id: String,
    pub account_id: String,
    pub date: DateTime<Utc>,
    pub description: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub category: Option<String>,
    pub merchant_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GoCardlessInstitution {
    pub id: String,
    pub name: String,
    pub country: String,
    pub logo_url: Option<String>,
    pub color: Option<String>,
    pub url: Option<String>,
    pub oauth_enabled: bool,
    pub capabilities: Vec<String>,
}
