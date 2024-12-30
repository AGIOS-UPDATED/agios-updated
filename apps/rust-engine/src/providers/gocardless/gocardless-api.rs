use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::{
    config::Config,
    utils::{ApiError, ApiResult},
};

use super::types::*;

pub struct GoCardlessApi {
    client: Client,
    api_key: String,
    environment: String,
}

impl GoCardlessApi {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            api_key: config.gocardless.api_key.clone(),
            environment: config.gocardless.environment.clone(),
        }
    }

    pub async fn get_institutions(&self, country: Option<&str>) -> ApiResult<Vec<GoCardlessInstitution>> {
        let url = format!(
            "https://{}-api.gocardless.com/institutions",
            self.environment
        );
        
        let mut request = self.client
            .get(&url)
            .bearer_auth(&self.api_key);

        if let Some(country) = country {
            request = request.query(&[("country", country)]);
        }

        let response = request.send().await?;
        let data = response.json().await?;
        Ok(data)
    }

    pub async fn get_accounts(&self, requisition_id: &str) -> ApiResult<Vec<GoCardlessAccount>> {
        let url = format!(
            "https://{}-api.gocardless.com/accounts",
            self.environment
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.api_key)
            .query(&[("requisition_id", requisition_id)])
            .send()
            .await?;

        let data = response.json().await?;
        Ok(data)
    }

    pub async fn get_transactions(
        &self,
        account_id: &str,
        date_from: Option<DateTime<Utc>>,
        date_to: Option<DateTime<Utc>>,
    ) -> ApiResult<Vec<GoCardlessTransaction>> {
        let url = format!(
            "https://{}-api.gocardless.com/accounts/{}/transactions",
            self.environment,
            account_id
        );
        
        let mut request = self.client
            .get(&url)
            .bearer_auth(&self.api_key);

        if let Some(date_from) = date_from {
            request = request.query(&[("date_from", date_from.format("%Y-%m-%d").to_string())]);
        }

        if let Some(date_to) = date_to {
            request = request.query(&[("date_to", date_to.format("%Y-%m-%d").to_string())]);
        }

        let response = request.send().await?;
        let data = response.json().await?;
        Ok(data)
    }
}
