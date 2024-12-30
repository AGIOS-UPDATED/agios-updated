use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBindings {
    pub api_secret_key: String,
    pub gocardless_secret_id: String,
    pub gocardless_secret_key: String,
    pub plaid_client_id: String,
    pub plaid_environment: String,
    pub plaid_secret: String,
    pub typesense_api_key: String,
    pub typesense_endpoint_au: String,
    pub typesense_endpoint_eu: String,
    pub typesense_endpoint_us: String,
    pub typesense_endpoint: String,
    pub redis_url: String,
    pub redis_token: String,
}

impl AppBindings {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            api_secret_key: std::env::var("API_SECRET_KEY")?,
            gocardless_secret_id: std::env::var("GOCARDLESS_SECRET_ID")?,
            gocardless_secret_key: std::env::var("GOCARDLESS_SECRET_KEY")?,
            plaid_client_id: std::env::var("PLAID_CLIENT_ID")?,
            plaid_environment: std::env::var("PLAID_ENVIRONMENT")
                .unwrap_or_else(|_| "sandbox".to_string()),
            plaid_secret: std::env::var("PLAID_SECRET")?,
            typesense_api_key: std::env::var("TYPESENSE_API_KEY")?,
            typesense_endpoint_au: std::env::var("TYPESENSE_ENDPOINT_AU")?,
            typesense_endpoint_eu: std::env::var("TYPESENSE_ENDPOINT_EU")?,
            typesense_endpoint_us: std::env::var("TYPESENSE_ENDPOINT_US")?,
            typesense_endpoint: std::env::var("TYPESENSE_ENDPOINT")?,
            redis_url: std::env::var("UPSTASH_REDIS_REST_URL")?,
            redis_token: std::env::var("UPSTASH_REDIS_REST_TOKEN")?,
        })
    }
}
