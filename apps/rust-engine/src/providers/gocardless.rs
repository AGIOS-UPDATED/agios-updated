use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use serde_json::Value;
use crate::config::Config;

use super::interface::Provider;
use super::types::*;

pub struct GoCardlessProvider {
    config: Arc<Config>,
}

impl GoCardlessProvider {
    pub fn new(config: &Arc<Config>) -> Self {
        Self {
            config: Arc::clone(config),
        }
    }
}

#[async_trait]
impl Provider for GoCardlessProvider {
    async fn get_transactions(
        &self,
        params: GetTransactionsRequest,
    ) -> Result<Vec<Transaction>, Box<dyn Error>> {
        // TODO: Implement GoCardless transactions API call
        Ok(vec![])
    }

    async fn get_accounts(
        &self,
        params: GetAccountsRequest,
    ) -> Result<Vec<Account>, Box<dyn Error>> {
        // TODO: Implement GoCardless accounts API call
        Ok(vec![])
    }

    async fn get_account_balance(
        &self,
        params: GetAccountBalanceRequest,
    ) -> Result<Balance, Box<dyn Error>> {
        // TODO: Implement GoCardless balance API call
        Ok(Balance {
            amount: 0.0,
            currency: "GBP".to_string(),
        })
    }

    async fn get_institutions(
        &self,
        params: GetInstitutionsRequest,
    ) -> Result<Vec<Institution>, Box<dyn Error>> {
        // TODO: Implement GoCardless institutions API call
        Ok(vec![])
    }

    async fn get_health_check(&self) -> Result<bool, Box<dyn Error>> {
        // TODO: Implement GoCardless health check
        Ok(true)
    }

    async fn delete_accounts(&self, params: DeleteAccountsRequest) -> Result<(), Box<dyn Error>> {
        // TODO: Implement GoCardless account deletion
        Ok(())
    }

    async fn get_connection_status(
        &self,
        params: GetConnectionStatusRequest,
    ) -> Result<ConnectionStatus, Box<dyn Error>> {
        // TODO: Implement GoCardless connection status check
        Ok(ConnectionStatus {
            status: ConnectionState::Connected,
        })
    }

    async fn delete_connection(
        &self,
        params: DeleteConnectionRequest,
    ) -> Result<(), Box<dyn Error>> {
        // TODO: Implement GoCardless connection deletion
        Ok(())
    }

    async fn exchange_token(&self, code: &str, redirect_uri: &str) -> Result<(String, String), Box<dyn Error>> {
        // Implement token exchange
        todo!()
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<(String, String), Box<dyn Error>> {
        // Implement token refresh
        todo!()
    }

    async fn health_check(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Implement GoCardless health check
        Ok(())
    }

    async fn get_accounts_by_access_token(&self, access_token: &str) -> Result<Vec<Value>, Box<dyn Error>> {
        // Implement get accounts
        todo!()
    }

    async fn get_transactions_by_access_token_and_account_id(&self, access_token: &str, account_id: &str) -> Result<Vec<Value>, Box<dyn Error>> {
        // Implement get transactions
        todo!()
    }

    async fn get_institutions_by_config(&self) -> Result<Vec<Value>, Box<dyn Error>> {
        // Implement get institutions
        todo!()
    }

    async fn get_institution_by_id(&self, id: &str) -> Result<Value, Box<dyn Error>> {
        // Implement get institution
        todo!()
    }
}
