use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::providers::types::*;

pub fn transform_account(account: PlaidAccount) -> Account {
    Account {
        id: format!("pl_{}", account.account_id),
        connection_id: account.connection_id,
        name: account.name,
        account_type: transform_account_type(account.r#type),
        currency: account.balances.iso_currency_code.unwrap_or_else(|| "USD".to_string()),
        balance: Balance {
            current: account.balances.current.unwrap_or(0.0),
            available: account.balances.available,
            limit: account.balances.limit,
        },
        last_sync: Some(Utc::now()),
    }
}

pub fn transform_transaction(transaction: PlaidTransaction) -> Transaction {
    Transaction {
        id: format!("pl_{}", transaction.transaction_id),
        account_id: format!("pl_{}", transaction.account_id),
        date: transaction.date,
        description: Some(transaction.name),
        amount: transaction.amount,
        currency: transaction.iso_currency_code.unwrap_or_else(|| "USD".to_string()),
        category: transaction.category.map(|c| c.join("/")),
        merchant: transaction.merchant_name,
        pending: transaction.pending,
    }
}

pub fn transform_institution(institution: PlaidInstitution) -> Institution {
    Institution {
        id: format!("pl_{}", institution.institution_id),
        name: institution.name,
        country: institution.country_codes.first().cloned().unwrap_or_else(|| "US".to_string()),
        provider: "plaid".to_string(),
        logo_url: institution.logo.map(|l| l.url),
        primary_color: institution.primary_color,
        url: None,
        oauth_support: institution.oauth,
        products: institution.products.into_iter().map(|p| p.to_string()).collect(),
        last_update: Utc::now(),
    }
}

fn transform_account_type(account_type: String) -> String {
    match account_type.as_str() {
        "depository" => "checking",
        "credit" => "credit",
        "loan" => "loan",
        "investment" => "investment",
        _ => "other",
    }.to_string()
}

#[derive(Debug, Deserialize)]
pub struct PlaidAccount {
    pub account_id: String,
    pub connection_id: String,
    pub name: String,
    pub r#type: String,
    pub balances: PlaidBalance,
}

#[derive(Debug, Deserialize)]
pub struct PlaidBalance {
    pub current: Option<f64>,
    pub available: Option<f64>,
    pub limit: Option<f64>,
    pub iso_currency_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PlaidTransaction {
    pub transaction_id: String,
    pub account_id: String,
    pub date: DateTime<Utc>,
    pub name: String,
    pub amount: f64,
    pub iso_currency_code: Option<String>,
    pub category: Option<Vec<String>>,
    pub merchant_name: Option<String>,
    pub pending: bool,
}

#[derive(Debug, Deserialize)]
pub struct PlaidInstitution {
    pub institution_id: String,
    pub name: String,
    pub country_codes: Vec<String>,
    pub products: Vec<String>,
    pub logo: Option<PlaidLogo>,
    pub primary_color: Option<String>,
    pub oauth: bool,
}

#[derive(Debug, Deserialize)]
pub struct PlaidLogo {
    pub url: String,
}
