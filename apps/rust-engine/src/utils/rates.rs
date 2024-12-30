// src/utils/rates.rs

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Mutex;
use reqwest::Client; // Assuming you're using reqwest for HTTP requests

#[derive(Debug, Clone)]
pub struct ExchangeRates {
    pub base: String,
    pub rates: HashMap<String, f64>,
}

pub struct RatesClient {
    // Add necessary fields, e.g., API URL, API key, HTTP client, cache, etc.
    api_url: String,
    client: Client,
    cache: Mutex<Option<ExchangeRates>>, // Simple cache example
}

impl RatesClient {
    pub fn new(api_url: &str) -> Self {
        RatesClient {
            api_url: api_url.to_string(),
            client: Client::new(),
            cache: Mutex::new(None),
        }
    }

    /// Fetch rates from the API or cache
    pub async fn get_rates(&self) -> Result<ExchangeRates, String> {
        // Check cache first
        {
            let cache = self.cache.lock().unwrap();
            if let Some(ref rates) = *cache {
                return Ok(rates.clone());
            }
        }

        // Fetch from API
        let response = self
            .client
            .get(&self.api_url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !response.status().is_success() {
            return Err(format!(
                "Failed to fetch rates: HTTP {}",
                response.status()
            ));
        }

        let data: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

        // Parse the JSON into ExchangeRates
        let base = data
            .get("base")
            .and_then(|b| b.as_str())
            .ok_or("Missing 'base' in response")?
            .to_string();

        let rates_map = data
            .get("rates")
            .and_then(|r| r.as_object())
            .ok_or("Missing 'rates' in response")?
            .iter()
            .map(|(k, v)| {
                v.as_f64()
                    .map(|val| (k.clone(), val))
                    .ok_or_else(|| format!("Invalid rate value for {}", k))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        let exchange_rates = ExchangeRates {
            base,
            rates: rates_map,
        };

        // Update cache
        {
            let mut cache = self.cache.lock().unwrap();
            *cache = Some(exchange_rates.clone());
        }

        Ok(exchange_rates)
    }

    /// Your existing get_exchange_rates method
    pub async fn get_exchange_rates(
        &self,
        base: &str,
        symbols: &[String],
        _date: Option<DateTime<Utc>>,
    ) -> Result<HashMap<String, f64>, String> {
        // 1) Fetch the latest rates from the cache or API
        let exchange_rates = self.get_rates().await?;

        if base != exchange_rates.base {
            return Err(format!(
                "Requested base '{}' does not match the client base '{}'. \
                (Auto-conversion to other bases is not yet implemented.)",
                base, exchange_rates.base
            ));
        }

        // 2) Filter the map by requested symbols if provided
        let all_rates = &exchange_rates.rates;
        let filtered = if !symbols.is_empty() {
            all_rates
                .iter()
                .filter(|(k, _)| symbols.contains(k))
                .map(|(k, v)| (k.clone(), *v))
                .collect::<HashMap<_, _>>()
        } else {
            all_rates.clone()
        };

        Ok(filtered)
    }
}
