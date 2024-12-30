use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;
use crate::providers::types::{
    Account, AccountType, Balance, ConnectionState, ConnectionStatus,
    Institution, Transaction, TransactionStatus,
};
use crate::providers::Provider;
use crate::utils::config::Config;

pub struct PlaidProvider {
    config: Arc<Config>,
}

impl PlaidProvider {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Provider for PlaidProvider {
    async fn exchange_token(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Plaid token exchange
        Ok(("plaid_mock_access_token".to_string(), "plaid_mock_refresh_token".to_string()))
    }

    async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Plaid token refresh
        Ok(("plaid_mock_refreshed_access".to_string(), "plaid_mock_refreshed_refresh".to_string()))
    }

    async fn get_accounts(
        &self,
        access_token: &str,
    ) -> Result<Vec<Account>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Plaid account retrieval
        Ok(vec![
            Account {
                id: "plaid_acc_1".to_string(),
                name: "Checking Account".to_string(),
                account_type: AccountType::Checking,
                balance: Balance {
                    amount: 1000.0,
                    currency: "USD".to_string(),
                },
                currency: "USD".to_string(),
                institution_id: "plaid_inst_1".to_string(),
                last_sync: Some(Utc::now()),
            }
        ])
    }

    async fn get_account_balance(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> Result<Balance, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Plaid balance retrieval
        Ok(Balance {
            amount: 1000.0,
            currency: "USD".to_string(),
        })
    }

    async fn get_transactions(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Plaid transaction retrieval
        Ok(vec![
            Transaction {
                id: "plaid_tx_1".to_string(),
                account_id: account_id.to_string(),
                amount: 50.0,
                currency: "USD".to_string(),
                date: Utc::now(),
                description: "Coffee Shop".to_string(),
                merchant: Some("Starbucks".to_string()),
                category: Some("Food and Drink".to_string()),
                status: TransactionStatus::Posted,
            }
        ])
    }

    async fn get_institutions(&self) -> Result<Vec<Institution>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Plaid institutions retrieval
        Ok(vec![
            Institution {
                id: "plaid_inst_1".to_string(),
                name: "Chase".to_string(),
                logo_url: Some("https://plaid.com/logos/chase.png".to_string()),
                website: Some("https://chase.com".to_string()),
                country: "US".to_string(),
            }
        ])
    }

    async fn get_connection_status(
        &self,
        access_token: &str,
    ) -> Result<ConnectionStatus, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Plaid connection status check
        Ok(ConnectionStatus {
            status: ConnectionState::Connected,
        })
    }

    async fn delete_connection(
        &self,
        access_token: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Plaid connection deletion
        Ok(())
    }
}
