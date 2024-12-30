use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use std::error::Error;
use crate::utils::config;

pub mod types;
pub use types::*;

pub mod plaid;
pub mod wise;

#[async_trait]
pub trait Provider: Send + Sync + 'static {
    async fn exchange_token(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<(String, String), Box<dyn Error + Send + Sync + 'static>>;
    
    async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<(String, String), Box<dyn Error + Send + Sync + 'static>>;
    
    async fn get_accounts(
        &self,
        access_token: &str,
    ) -> Result<Vec<Account>, Box<dyn Error + Send + Sync + 'static>>;
    
    async fn get_account_balance(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> Result<Balance, Box<dyn Error + Send + Sync + 'static>>;
    
    async fn get_transactions(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> Result<Vec<Transaction>, Box<dyn Error + Send + Sync + 'static>>;
    
    async fn get_institutions(&self) -> Result<Vec<Institution>, Box<dyn Error + Send + Sync + 'static>>;
    
    async fn get_connection_status(
        &self,
        access_token: &str,
    ) -> Result<ConnectionStatus, Box<dyn Error + Send + Sync + 'static>>;
    
    async fn delete_connection(
        &self,
        access_token: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
}

pub struct ProviderFactory {
    providers: HashMap<String, Arc<dyn Provider>>,
    config: Arc<config::Config>,
}

impl ProviderFactory {
    pub fn new(config: Arc<config::Config>) -> Self {
        let mut providers: HashMap<String, Arc<dyn Provider>> = HashMap::new();

        providers.insert(
            "plaid".to_string(), 
            Arc::new(plaid::PlaidProvider::new(config.clone())) as Arc<dyn Provider>
        );
        providers.insert(
            "wise".to_string(), 
            Arc::new(wise::WiseProvider::new(config.clone())) as Arc<dyn Provider>
        );

        Self { 
            providers,
            config, 
        }
    }

    pub fn get_provider(&self, provider: &str) -> Option<Arc<dyn Provider>> {
        self.providers.get(provider).cloned()
    }
}
