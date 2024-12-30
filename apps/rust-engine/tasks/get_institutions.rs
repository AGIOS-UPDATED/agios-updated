use std::collections::HashMap;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    providers::ProviderFactory,
    utils::{ApiError, ApiResult},
};

pub async fn get_institutions(
    config: &Config,
    provider_factory: &ProviderFactory,
    provider: &str,
    country: Option<String>,
) -> ApiResult<Vec<Institution>> {
    let provider = provider_factory
        .get_provider(provider)
        .ok_or_else(|| ApiError::BadRequest("Invalid provider".to_string()))?;

    let institutions = provider.get_institutions(country.as_deref()).await?;

    Ok(institutions)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Institution {
    pub id: String,
    pub name: String,
    pub country: String,
    pub provider: String,
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub url: Option<String>,
    pub oauth_support: bool,
    pub products: Vec<String>,
    pub last_update: chrono::DateTime<Utc>,
}
