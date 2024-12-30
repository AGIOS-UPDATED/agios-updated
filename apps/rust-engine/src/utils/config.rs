use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub port: u16,
    pub plaid_client_id: String,
    pub plaid_secret: String,
    pub plaid_environment: String,
    pub gocardless_client_id: String,
    pub gocardless_secret: String,
    pub gocardless_environment: String,
    pub truelayer_client_id: String,
    pub truelayer_secret: String,
    pub truelayer_environment: String,
    pub wise_client_id: String,
    pub wise_secret: String,
    pub wise_environment: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            plaid_client_id: env::var("PLAID_CLIENT_ID")?,
            plaid_secret: env::var("PLAID_SECRET")?,
            plaid_environment: env::var("PLAID_ENVIRONMENT").unwrap_or_else(|_| "sandbox".to_string()),
            gocardless_client_id: env::var("GOCARDLESS_CLIENT_ID")?,
            gocardless_secret: env::var("GOCARDLESS_SECRET")?,
            gocardless_environment: env::var("GOCARDLESS_ENVIRONMENT").unwrap_or_else(|_| "sandbox".to_string()),
            truelayer_client_id: env::var("TRUELAYER_CLIENT_ID")?,
            truelayer_secret: env::var("TRUELAYER_SECRET")?,
            truelayer_environment: env::var("TRUELAYER_ENVIRONMENT").unwrap_or_else(|_| "sandbox".to_string()),
            wise_client_id: env::var("WISE_CLIENT_ID")?,
            wise_secret: env::var("WISE_SECRET")?,
            wise_environment: env::var("WISE_ENVIRONMENT").unwrap_or_else(|_| "sandbox".to_string()),
        })
    }
}
