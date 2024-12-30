use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, ToSchema)]
pub struct ConnectionQuery {
    pub institution_id: Option<String>,
    pub provider: Option<String>,
    pub status: Option<ConnectionStatus>,
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionStatus {
    Active,
    Pending,
    Error,
    Disconnected,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ConnectionResponse {
    pub data: Vec<Connection>,
    pub page: u32,
    pub total_pages: u32,
    pub total_items: u64,
    pub has_more: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Connection {
    pub id: String,
    pub institution_id: String,
    pub provider: String,
    pub status: ConnectionStatus,
    pub last_sync: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ConnectionStats {
    pub total_connections: u64,
    pub active_connections: u64,
    pub by_status: Vec<StatusStat>,
    pub by_provider: Vec<ProviderStat>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StatusStat {
    pub status: ConnectionStatus,
    pub count: u64,
    pub percentage: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProviderStat {
    pub provider: String,
    pub count: u64,
    pub percentage: f64,
}
