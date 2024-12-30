use crate::{
    config::Config,
    providers::ProviderFactory,
    utils::{ApiError, ApiResult},
};

pub async fn download_teller_data(
    config: &Config,
    provider_factory: &ProviderFactory,
    connection_id: &str,
    account_id: &str,
) -> ApiResult<()> {
    let provider = provider_factory
        .get_provider("teller")
        .ok_or_else(|| ApiError::BadRequest("Teller provider not configured".to_string()))?;

    // Download accounts
    let accounts = provider.get_accounts(connection_id).await?;
    
    // Download transactions
    let transactions = provider.get_transactions(
        connection_id,
        account_id,
        None,
        None,
    ).await?;

    // Store in database
    // TODO: Implement database storage

    Ok(())
}
