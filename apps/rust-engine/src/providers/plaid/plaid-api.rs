use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::{
    config::Config,
    utils::{ApiError, ApiResult},
};

use super::types::*;

pub struct PlaidApi {
    client: Client,
    client_id: String,
    secret: String,
    environment: String,
}

impl PlaidApi {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            client_id: config.plaid.client_id.clone(),
            secret: config.plaid.secret.clone(),
            environment: config.plaid.environment.clone(),
        }
    }

    pub async fn get_institutions(&self, country_codes: Option<&[&str]>) -> ApiResult<Vec<PlaidInstitution>> {
        let url = format!("https://{}.plaid.com/institutions/get", self.environment);
        
        let response = self.client
            .post(&url)
            .json(&serde_json::json!({
                "client_id": self.client_id,
                "secret": self.secret,
                "count": 500,
                "offset": 0,
                "country_codes": country_codes.unwrap_or(&["US", "CA", "GB"]),
            }))
            .send()
            .await?;

        let data = response.json().await?;
        Ok(data)
    }

    pub async fn get_accounts(&self, access_token: &str) -> ApiResult<Vec<PlaidAccount>> {
        let url = format!("https://{}.plaid.com/accounts/get", self.environment);
        
        let response = self.client
            .post(&url)
            .json(&serde_json::json!({
                "client_id": self.client_id,
                "secret": self.secret,
                "access_token": access_token,
            }))
            .send()
            .await?;

        let data = response.json().await?;
        Ok(data)
    }

    pub async fn get_transactions(
        &self,
        access_token: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> ApiResult<Vec<PlaidTransaction>> {
        let url = format!("https://{}.plaid.com/transactions/get", self.environment);
        
        let response = self.client
            .post(&url)
            .json(&serde_json::json!({
                "client_id": self.client_id,
                "secret": self.secret,
                "access_token": access_token,
                "start_date": start_date.format("%Y-%m-%d").to_string(),
                "end_date": end_date.format("%Y-%m-%d").to_string(),
            }))
            .send()
            .await?;

        let data = response.json().await?;
        Ok(data)
    }
}
