use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, ToSchema)]
pub struct InstitutionQuery {
    pub country: Option<String>,
    pub provider: Option<String>,
    pub search: Option<String>,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 {
    1
}

fn default_limit() -> u32 {
    20
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InstitutionsResponse {
    pub data: Vec<EnrichedInstitution>,
    pub page: u32,
    pub total_pages: u32,
    pub total_items: u64,
    pub has_more: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EnrichedInstitution {
    pub id: String,
    pub name: String,
    pub country: String,
    pub provider: String,
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub url: Option<String>,
    pub oauth_support: bool,
    pub products: Vec<String>,
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUsageRequest {
    pub institution_id: String,
    pub provider: String,
    pub action: UsageAction,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UsageAction {
    View,
    Connect,
    Disconnect,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InstitutionStats {
    pub total_institutions: u64,
    pub total_connections: u64,
    pub by_country: Vec<CountryStat>,
    pub by_provider: Vec<ProviderStat>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CountryStat {
    pub country: String,
    pub count: u64,
    pub percentage: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProviderStat {
    pub provider: String,
    pub count: u64,
    pub percentage: f64,
}
