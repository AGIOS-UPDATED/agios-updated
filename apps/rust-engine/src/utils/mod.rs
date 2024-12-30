pub mod account;
pub mod config;
pub mod countries;
pub mod enrich;
pub mod error;
pub mod logo;
pub mod paginate;
pub mod rates;
pub mod retry;
pub mod search;

// Re-export commonly used utilities
pub use account::{generate_account_id, normalize_account_type};
pub use countries::{get_country_name, is_supported_country};
pub use enrich::enrich_transaction;
pub use error::{ApiError, ApiResult};
pub use logo::get_institution_logo;
pub use paginate::{PaginatedResponse, PaginationParams};
pub use rates::RatesClient;
pub use retry::{retry, RetryConfig};
