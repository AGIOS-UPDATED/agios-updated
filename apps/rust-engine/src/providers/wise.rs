use async_trait::async_trait;
use chrono::Utc;
use crate::providers::types::{
    Account, AccountType, Balance, ConnectionState, ConnectionStatus,
    Institution, Transaction, TransactionStatus,
};
use crate::providers::Provider;

pub struct WiseProvider {
    // Add configuration fields here
}

impl WiseProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Provider for WiseProvider {
    async fn exchange_token(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Wise token exchange
        Ok(("wise_mock_access_token".to_string(), "wise_mock_refresh_token".to_string()))
    }

    async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Wise token refresh
        Ok(("wise_mock_refreshed_access".to_string(), "wise_mock_refreshed_refresh".to_string()))
    }

    async fn get_accounts(
        &self,
        access_token: &str,
    ) -> Result<Vec<Account>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Wise account retrieval
        Ok(vec![
            Account {
                id: "wise_acc_1".to_string(),
                name: "Multi-Currency Account".to_string(),
                account_type: AccountType::Checking,
                balance: Balance {
                    amount: 1000.0,
                    currency: "EUR".to_string(),
                },
                currency: "EUR".to_string(),
                institution_id: "wise_inst_1".to_string(),
                last_sync: Some(Utc::now()),
            }
        ])
    }

    async fn get_account_balance(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> Result<Balance, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Wise balance retrieval
        Ok(Balance {
            amount: 1000.0,
            currency: "EUR".to_string(),
        })
    }

    async fn get_transactions(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Wise transaction retrieval
        Ok(vec![
            Transaction {
                id: "wise_tx_1".to_string(),
                account_id: account_id.to_string(),
                amount: 50.0,
                currency: "EUR".to_string(),
                date: Utc::now(),
                description: "International Transfer".to_string(),
                merchant: Some("Wise Transfer".to_string()),
                category: Some("Transfer".to_string()),
                status: TransactionStatus::Posted,
            }
        ])
    }

    async fn get_institutions(&self) -> Result<Vec<Institution>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Wise institutions retrieval
        Ok(vec![
            Institution {
                id: "wise_inst_1".to_string(),
                name: "Wise".to_string(),
                logo_url: Some("https://wise.com/logo.png".to_string()),
                website: Some("https://wise.com".to_string()),
                country: "GB".to_string(),
            }
        ])
    }

    async fn get_connection_status(
        &self,
        access_token: &str,
    ) -> Result<ConnectionStatus, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Wise connection status check
        Ok(ConnectionStatus {
            status: ConnectionState::Connected,
        })
    }

    async fn delete_connection(
        &self,
        access_token: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Wise connection deletion
        Ok(())
    }
}
