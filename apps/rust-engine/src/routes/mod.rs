use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    providers::ProviderFactory,
    utils::ApiError,
};

pub mod accounts;
pub mod auth;
pub mod connections;
pub mod health;
pub mod institutions;
pub mod rates;
pub mod transactions;


// Re-export these if needed in other parts of the code
pub use accounts::get_accounts;
pub use auth::{exchange_token, refresh_token_handler};
pub use connections::{delete_connection, get_connections};
pub use institutions::{get_institution, get_institutions, update_institution_usage};
pub use rates::get_rates;
pub use transactions::get_transactions;
pub use health::health_check;

/// Example struct to represent an empty JSON response.
/// Derives `Serialize` so it can be converted to JSON.
#[derive(Debug, Serialize)]
pub struct EmptyResponse {}

#[derive(Debug, Deserialize)]
pub struct GetAccountsRequest {
    pub provider: String,
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct GetTransactionsRequest {
    pub provider: String,
    pub access_token: String,
    pub account_id: String,
}

#[derive(Deserialize)]
pub struct TokenRequest {
    code: String,
    redirect_uri: String,
    provider: String,
}

#[derive(Deserialize)]
pub struct RefreshTokenRequest {
    refresh_token: String,
    provider: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(health_check)
            .service(exchange_token)
            .service(refresh_token_handler)
            .service(get_accounts)
            .service(get_transactions)
            .service(get_connections)
            .service(delete_connection)
            .service(get_institutions)
            .service(get_institution)
            .service(update_institution_usage)
            .service(get_rates),
    );
}
