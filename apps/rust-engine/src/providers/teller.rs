use async_trait::async_trait;
use std::error::Error;

use super::interface::Provider;
use super::types::*;

pub struct TellerProvider {
    api_key: String,
    environment: String,
}

impl TellerProvider {
    pub fn new(api_key: String, environment: String) -> Self {
        Self {
            api_key,
            environment,
        }
    }
}

#[async_trait]
impl Provider for TellerProvider {
    async fn get_transactions(
        &self,
        params: GetTransactionsRequest,
    ) -> Result<Vec<Transaction>, Box<dyn Error>> {
        // TODO: Implement Teller transactions API call
        Ok(vec![])
    }

    async fn get_accounts(
        &self,
        params: GetAccountsRequest,
    ) -> Result<Vec<Account>, Box<dyn Error>> {
        // TODO: Implement Teller accounts API call
        Ok(vec![])
    }

    async fn get_account_balance(
        &self,
        params: GetAccountBalanceRequest,
    ) -> Result<Balance, Box<dyn Error>> {
        // TODO: Implement Teller balance API call
        Ok(Balance {
            amount: 0.0,
            currency: "USD".to_string(),
        })
    }

    async fn get_institutions(
        &self,
        params: GetInstitutionsRequest,
    ) -> Result<Vec<Institution>, Box<dyn Error>> {
        // TODO: Implement Teller institutions API call
        Ok(vec![])
    }

    async fn get_health_check(&self) -> Result<bool, Box<dyn Error>> {
        // TODO: Implement Teller health check
        Ok(true)
    }

    async fn delete_accounts(&self, params: DeleteAccountsRequest) -> Result<(), Box<dyn Error>> {
        // TODO: Implement Teller account deletion
        Ok(())
    }

    async fn get_connection_status(
        &self,
        params: GetConnectionStatusRequest,
    ) -> Result<ConnectionStatus, Box<dyn Error>> {
        // TODO: Implement Teller connection status check
        Ok(ConnectionStatus {
            status: ConnectionState::Connected,
        })
    }

    async fn delete_connection(
        &self,
        params: DeleteConnectionRequest,
    ) -> Result<(), Box<dyn Error>> {
        // TODO: Implement Teller connection deletion
        Ok(())
    }

    async fn exchange_token(&self, code: &str, redirect_uri: &str) -> Result<(String, String), Box<dyn Error>> {
        // TODO: Implement Teller token exchange
        Ok(("dummy_access_token".to_string(), "dummy_refresh_token".to_string()))
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<(String, String), Box<dyn Error>> {
        // TODO: Implement Teller token refresh
        Ok(("new_access_token".to_string(), "new_refresh_token".to_string()))
    }

    async fn health_check(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Implement Teller health check
        Ok(())
    }
}
