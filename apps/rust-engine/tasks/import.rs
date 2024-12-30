use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::{
    config::Config,
    utils::{ApiError, ApiResult},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportData {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: String,
    pub currency: String,
    pub balance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub account_id: String,
    pub date: DateTime<Utc>,
    pub description: String,
    pub amount: f64,
    pub currency: String,
}

pub async fn import_data(
    config: &Config,
    file_path: &Path,
) -> ApiResult<ImportData> {
    // Read file
    let content = tokio::fs::read_to_string(file_path)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to read file: {}", e)))?;

    // Parse JSON
    let data: ImportData = serde_json::from_str(&content)
        .map_err(|e| ApiError::BadRequest(format!("Invalid JSON format: {}", e)))?;

    // Validate data
    validate_data(&data)?;

    // Store in database
    // TODO: Implement database storage

    Ok(data)
}

fn validate_data(data: &ImportData) -> ApiResult<()> {
    // Validate accounts
    for account in &data.accounts {
        if account.id.is_empty() || account.name.is_empty() {
            return Err(ApiError::BadRequest("Invalid account data".to_string()));
        }
    }

    // Validate transactions
    for tx in &data.transactions {
        if tx.id.is_empty() || tx.account_id.is_empty() {
            return Err(ApiError::BadRequest("Invalid transaction data".to_string()));
        }
    }

    Ok(())
}
