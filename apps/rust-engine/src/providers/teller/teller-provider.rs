use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    providers::{interface::Provider, types::*},
    utils::{ApiError, ApiResult, retry, RetryConfig},
};

pub struct TellerProvider {
    client: Client,
    api_key: String,
    environment: String,
}

impl TellerProvider {
    pub fn new(api_key: String, environment: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            environment: environment.to_lowercase(),
        }
    }

    fn api_url(&self) -> String {
        match self.environment.as_str() {
            "sandbox" => "https://api.teller.io/sandbox".to_string(),
            "development" => "https://api.teller.io/development".to_string(),
            _ => "https://api.teller.io".to_string(),
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
                .header("Authorization", format!("Basic {}", self.api_key))
                .header("Content-Type", "application/json");

            if let Some(body) = &body {
                req = req.json(body);
            }

            let response = req.send().await?;

            if !response.status().is_success() {
                let error: TellerError = response.json().await?;
                return Err(ApiError::Provider(error.message));
            }

            Ok(response.json::<T>().await?)
        }, config).await
    }
}

#[async_trait]
impl Provider for TellerProvider {
    async fn exchange_token(
        &self,
        code: &str,
        _redirect_uri: &str,
    ) -> ApiResult<TokenResponse> {
        let response: TellerTokenResponse = self.request(
            reqwest::Method::POST,
            "/auth/exchange",
            Some(serde_json::json!({
                "enrollment_id": code,
            })),
        ).await?;

        Ok(TokenResponse {
            access_token: response.access_token,
            refresh_token: Some(response.access_token.clone()), // Teller uses same token
            expires_in: 7200, // 2 hours
            expires_at: Utc::now() + chrono::Duration::hours(2),
        })
    }

    async fn refresh_token(&self, token: &str) -> ApiResult<TokenResponse> {
        // Teller tokens don't expire, but we'll implement refresh for consistency
        Ok(TokenResponse {
            access_token: token.to_string(),
            refresh_token: Some(token.to_string()),
            expires_in: 7200,
            expires_at: Utc::now() + chrono::Duration::hours(2),
        })
    }

    async fn get_accounts(&self, access_token: &str) -> ApiResult<Vec<Account>> {
        let accounts: Vec<TellerAccount> = self.request(
            reqwest::Method::GET,
            "/accounts",
            None,
        ).await?;

        Ok(accounts.into_iter().map(|acc| Account {
            id: format!("tel_{}", acc.id),
            connection_id: access_token.to_string(),
            name: acc.name,
            account_type: acc.r#type,
            currency: acc.currency.unwrap_or_else(|| "USD".to_string()),
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
        let account_id = account_id.strip_prefix("tel_").unwrap_or(account_id);
        let transactions: Vec<TellerTransaction> = self.request(
            reqwest::Method::GET,
            &format!("/accounts/{}/transactions", account_id),
            None,
        ).await?;

        Ok(transactions.into_iter()
            .filter(|tx| {
                let date = DateTime::parse_from_rfc3339(&tx.date)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());
                
                from_date.map_or(true, |from| date >= from) &&
                to_date.map_or(true, |to| date <= to)
            })
            .map(|tx| Transaction {
                id: format!("tel_{}", tx.id),
                account_id: format!("tel_{}", tx.account_id),
                date: DateTime::parse_from_rfc3339(&tx.date)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                description: Some(tx.description),
                amount: tx.amount,
                currency: tx.currency.unwrap_or_else(|| "USD".to_string()),
                category: tx.category,
                merchant: tx.merchant.map(|m| m.name),
                pending: tx.status == "pending",
            })
            .collect())
    }

    async fn get_institution(&self, institution_id: &str) -> ApiResult<Institution> {
        Ok(Institution {
            id: format!("tel_{}", institution_id),
            name: "Teller Institution".to_string(), // Teller doesn't provide institution details
            country: "US".to_string(),
            provider: "teller".to_string(),
            logo_url: None,
            primary_color: None,
            url: None,
            oauth_support: true,
            products: vec!["accounts".to_string(), "transactions".to_string()],
            last_update: Utc::now(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct TellerError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct TellerTokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct TellerAccount {
    id: String,
    name: String,
    r#type: String,
    currency: Option<String>,
    balance: TellerBalance,
}

#[derive(Debug, Deserialize)]
struct TellerBalance {
    current: f64,
    available: f64,
    limit: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct TellerTransaction {
    id: String,
    account_id: String,
    date: String,
    description: String,
    amount: f64,
    currency: Option<String>,
    status: String,
    category: Option<String>,
    merchant: Option<TellerMerchant>,
}

#[derive(Debug, Deserialize)]
struct TellerMerchant {
    name: String,
}
