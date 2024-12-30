use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::{
    config::Config,
    utils::{ApiError, ApiResult},
};

use super::types::*;

pub struct TellerApi {
    client: Client,
    api_key: String,
    environment: String,
}

impl TellerApi {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            api_key: config.teller.api_key.clone(),
            environment: config.teller.environment.clone(),
        }
    }

    pub async fn get_institutions(&self) -> ApiResult<Vec<TellerInstitution>> {
        let url = format!("https://api.{}.teller.io/institutions", self.environment);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        let data = response.json().await?;
        Ok(data)
    }

    pub async fn get_accounts(&self, access_token: &str) -> ApiResult<Vec<TellerAccount>> {
        let url = format!("https://api.{}.teller.io/accounts", self.environment);
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.api_key)
            .header("Teller-Account-Token", access_token)
            .send()
            .await?;

        let data = response.json().await?;
        Ok(data)
    }

    pub async fn get_transactions(
        &self,
        access_token: &str,
        account_id: &str,
        from_id: Option<&str>,
    ) -> ApiResult<Vec<TellerTransaction>> {
        let url = format!(
            "https://api.{}.teller.io/accounts/{}/transactions",
            self.environment,
            account_id
        );
        
        let mut request = self.client
            .get(&url)
            .bearer_auth(&self.api_key)
            .header("Teller-Account-Token", access_token);

        if let Some(from_id) = from_id {
            request = request.query(&[("from_id", from_id)]);
        }

        let response = request.send().await?;
        let data = response.json().await?;
        Ok(data)
    }
}
