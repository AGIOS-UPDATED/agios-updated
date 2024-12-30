use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    providers::{interface::Provider, types::*},
    utils::{ApiError, ApiResult, retry, RetryConfig},
};

pub struct GoCardlessProvider {
    client: Client,
    secret_id: String,
    secret_key: String,
    environment: String,
}

impl GoCardlessProvider {
    pub fn new(secret_id: String, secret_key: String, environment: String) -> Self {
        Self {
            client: Client::new(),
            secret_id,
            secret_key,
            environment: environment.to_lowercase(),
        }
    }

    fn api_url(&self) -> String {
        match self.environment.as_str() {
            "sandbox" => "https://bankable.gocardless.com/api/v2/sandbox".to_string(),
            _ => "https://bankable.gocardless.com/api/v2".to_string(),
        }
    }

    async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> ApiResult<T> {
        let url = format!("{}{}", self.api_url(), path);
        let config = RetryConfig::default();

        retry(|| async {
            let mut req = self.client
                .request(method.clone(), &url)
                .basic_auth(&self.secret_id, Some(&self.secret_key))
                .header("Content-Type", "application/json")
                .header("Accept", "application/json");

            if let Some(body) = &body {
                req = req.json(body);
            }

            let response = req.send().await?;

            if !response.status().is_success() {
                let error: GoCardlessError = response.json().await?;
                return Err(ApiError::Provider(error.message));
            }

            Ok(response.json::<T>().await?)
        }, config).await
    }
}

#[async_trait]
impl Provider for GoCardlessProvider {
    async fn exchange_token(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> ApiResult<TokenResponse> {
        let response: GoCardlessTokenResponse = self.request(
            reqwest::Method::POST,
            "/auth/access-token",
            Some(serde_json::json!({
                "code": code,
                "redirect_uri": redirect_uri,
            })),
        ).await?;

        Ok(TokenResponse {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_in: response.expires_in,
            expires_at: Utc::now() + chrono::Duration::seconds(response.expires_in as i64),
        })
    }

    async fn refresh_token(&self, token: &str) -> ApiResult<TokenResponse> {
        let response: GoCardlessTokenResponse = self.request(
            reqwest::Method::POST,
            "/auth/refresh",
            Some(serde_json::json!({
                "refresh_token": token,
            })),
        ).await?;

        Ok(TokenResponse {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_in: response.expires_in,
            expires_at: Utc::now() + chrono::Duration::seconds(response.expires_in as i64),
        })
    }

    async fn get_accounts(&self, access_token: &str) -> ApiResult<Vec<Account>> {
        let accounts: GoCardlessAccountsResponse = self.request(
            reqwest::Method::GET,
            "/accounts",
            None,
        ).await?;

        Ok(accounts.accounts.into_iter().map(|acc| Account {
            id: format!("gc_{}", acc.id),
            connection_id: access_token.to_string(),
            name: acc.name,
            account_type: acc.account_type,
            currency: acc.currency,
            balance: Balance {
                current: acc.balance.current,
                available: Some(acc.balance.available),
                limit: acc.balance.limit,
            },
            last_sync: Some(Utc::now()),
        }).collect())
    }

    async fn get_transactions(
        &self,
        access_token: &str,
        account_id: &str,
        from_date: Option<DateTime<Utc>>,
        to_date: Option<DateTime<Utc>>,
    ) -> ApiResult<Vec<Transaction>> {
        let account_id = account_id.strip_prefix("gc_").unwrap_or(account_id);
        let mut params = vec![];
        
        if let Some(from) = from_date {
            params.push(("from_date", from.to_rfc3339()));
        }
        if let Some(to) = to_date {
            params.push(("to_date", to.to_rfc3339()));
        }

        let transactions: GoCardlessTransactionsResponse = self.request(
            reqwest::Method::GET,
            &format!("/accounts/{}/transactions", account_id),
            None,
        ).await?;

        Ok(transactions.transactions.into_iter().map(|tx| Transaction {
            id: format!("gc_{}", tx.id),
            account_id: format!("gc_{}", tx.account_id),
            date: tx.date,
            description: Some(tx.description),
            amount: tx.amount,
            currency: tx.currency,
            category: tx.category,
            merchant: tx.merchant_name,
            pending: tx.status == "pending",
        }).collect())
    }

    async fn get_institution(&self, institution_id: &str) -> ApiResult<Institution> {
        let institution: GoCardlessInstitution = self.request(
            reqwest::Method::GET,
            &format!("/institutions/{}", institution_id),
            None,
        ).await?;

        Ok(Institution {
            id: format!("gc_{}", institution.id),
            name: institution.name,
            country: institution.country,
            provider: "gocardless".to_string(),
            logo_url: institution.logo_url,
            primary_color: institution.color,
            url: institution.url,
            oauth_support: institution.oauth_enabled,
            products: institution.capabilities,
            last_update: Utc::now(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct GoCardlessError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct GoCardlessTokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: i32,
}

#[derive(Debug, Deserialize)]
struct GoCardlessAccountsResponse {
    accounts: Vec<GoCardlessAccount>,
}

#[derive(Debug, Deserialize)]
struct GoCardlessAccount {
    id: String,
    name: String,
    account_type: String,
    currency: String,
    balance: GoCardlessBalance,
}

#[derive(Debug, Deserialize)]
struct GoCardlessBalance {
    current: f64,
    available: f64,
    limit: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct GoCardlessTransactionsResponse {
    transactions: Vec<GoCardlessTransaction>,
}

#[derive(Debug, Deserialize)]
struct GoCardlessTransaction {
    id: String,
    account_id: String,
    date: DateTime<Utc>,
    description: String,
    amount: f64,
    currency: String,
    status: String,
    category: Option<String>,
    merchant_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoCardlessInstitution {
    id: String,
    name: String,
    country: String,
    logo_url: Option<String>,
    color: Option<String>,
    url: Option<String>,
    oauth_enabled: bool,
    capabilities: Vec<String>,
}
