use async_trait::async_trait;
use std::error::Error;

use super::types::{
    Account, Balance, ConnectionStatus, DeleteAccountsRequest, DeleteConnectionRequest,
    GetAccountBalanceRequest, GetAccountsRequest, GetConnectionStatusRequest, GetInstitutionsRequest,
    GetTransactionsRequest, Institution, Transaction,
};

#[async_trait]
pub trait Provider: Send + Sync {
    async fn get_transactions(
        &self,
        params: GetTransactionsRequest,
    ) -> Result<Vec<Transaction>, Box<dyn Error>>;

    async fn get_accounts(
        &self,
        params: GetAccountsRequest,
    ) -> Result<Vec<Account>, Box<dyn Error>>;

    async fn get_account_balance(
        &self,
        params: GetAccountBalanceRequest,
    ) -> Result<Balance, Box<dyn Error>>;

    async fn get_institutions(
        &self,
        params: GetInstitutionsRequest,
    ) -> Result<Vec<Institution>, Box<dyn Error>>;

    async fn get_health_check(&self) -> Result<bool, Box<dyn Error>>;

    async fn delete_accounts(&self, params: DeleteAccountsRequest) -> Result<(), Box<dyn Error>>;

    async fn get_connection_status(
        &self,
        params: GetConnectionStatusRequest,
    ) -> Result<ConnectionStatus, Box<dyn Error>>;

    async fn delete_connection(
        &self,
        params: DeleteConnectionRequest,
    ) -> Result<(), Box<dyn Error>>;

    async fn exchange_token(&self, code: &str, redirect_uri: &str) -> Result<(String, String), Box<dyn Error>>;
    async fn refresh_token(&self, refresh_token: &str) -> Result<(String, String), Box<dyn Error>>;
    async fn health_check(&self) -> Result<(), Box<dyn Error>>;
}
