use super::types::*;

pub fn get_account_type(account: &GoCardlessAccount) -> String {
    match account.product.as_deref() {
        Some("checking") => "checking".to_string(),
        Some("savings") => "savings".to_string(),
        Some("credit_card") => "credit".to_string(),
        Some("loan") => "loan".to_string(),
        _ => "other".to_string(),
    }
}

pub fn get_transaction_type(transaction: &GoCardlessTransaction) -> String {
    match transaction.transaction_type.as_deref() {
        Some("SEPA_CREDIT_TRANSFER") => "transfer".to_string(),
        Some("SEPA_DIRECT_DEBIT") => "payment".to_string(),
        _ => "other".to_string(),
    }
}
