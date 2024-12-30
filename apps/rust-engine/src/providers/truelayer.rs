use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use serde_json::Value;
use crate::config::Config;

use super::Provider;

pub struct TrueLayerProvider {
    config: Arc<Config>,
}

impl TrueLayerProvider {
    pub fn new(config: &Arc<Config>) -> Self {
        Self {
            config: Arc::clone(config),
        }
    }
}

#[async_trait]
impl Provider for TrueLayerProvider {
    async fn exchange_token(
        &self,
        _code: &str,
        _redirect_uri: &str,
    ) -> Result<(String, String), Box<dyn Error>> {
        todo!()
    }

    async fn refresh_token(
        &self,
        _refresh_token: &str,
    ) -> Result<(String, String), Box<dyn Error>> {
        todo!()
    }

    async fn get_accounts(
        &self,
        _access_token: &str,
    ) -> Result<Vec<Value>, Box<dyn Error>> {
        todo!()
    }

    async fn get_transactions(
        &self,
        _access_token: &str,
        _account_id: &str,
    ) -> Result<Vec<Value>, Box<dyn Error>> {
        todo!()
    }

    async fn get_institutions(
        &self,
    ) -> Result<Vec<Value>, Box<dyn Error>> {
        todo!()
    }

    async fn get_institution(
        &self,
        _id: &str,
    ) -> Result<Value, Box<dyn Error>> {
        todo!()
    }
}
