use actix_web::{get, web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::utils::RatesClient; // or however you bring RatesClient into scope

#[derive(Debug, Deserialize)]
pub struct RatesQuery {
    pub base: String,
    pub symbols: Option<String>,
    pub date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct RatesResponse {
    pub base: String,
    pub date: DateTime<Utc>,
    pub rates: HashMap<String, f64>,
}

// If you want to store RatesClient in Actix's application data:
#[get("/api/v1/rates")]
pub async fn get_rates(
    query: web::Query<RatesQuery>,
    rates_client: web::Data<RatesClient>,  // <-- important
) -> Result<HttpResponse, actix_web::Error> {
    let query = query.into_inner();

    let symbols = query
        .symbols
        .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_else(|| vec![]);

    // Use your new method
    let rates_map = rates_client
        .get_exchange_rates(&query.base, &symbols, query.date)
        .await
        .map_err(|err| {
            // Convert your String error into an Actix error
            actix_web::error::ErrorBadRequest(err)
        })?;

    Ok(HttpResponse::Ok().json(RatesResponse {
        base: query.base,
        date: query.date.unwrap_or_else(Utc::now),
        rates: rates_map,
    }))
}
