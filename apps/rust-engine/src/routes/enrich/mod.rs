use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::utils::{enrich::*, ApiResult};

#[derive(Debug, Deserialize)]
pub struct EnrichRequest {
    pub text: String,
    pub categories: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct EnrichResponse {
    pub category: Option<String>,
    pub merchant: Option<String>,
}

/// Enrich transaction data
///
/// Enrich transaction description with category and merchant information
#[utoipa::path(
    post,
    path = "/api/v1/enrich",
    request_body = EnrichRequest,
    responses(
        (status = 200, description = "Enriched data", body = EnrichResponse),
        (status = 400, description = "Invalid request parameters"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("api_key" = [])
    )
)]
#[post("/api/v1/enrich")]
pub async fn enrich_transaction(
    request: web::Json<EnrichRequest>,
) -> ApiResult<HttpResponse> {
    let request = request.into_inner();
    
    let enriched = enrich_transaction_text(
        &request.text,
        request.categories.as_deref(),
    );

    Ok(HttpResponse::Ok().json(EnrichResponse {
        category: enriched.category,
        merchant: enriched.merchant,
    }))
}
