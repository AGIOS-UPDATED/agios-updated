use super::types::*;

pub fn get_account_type(account: &TellerAccount) -> String {
    match account.r#type.as_str() {
        "depository" => "checking".to_string(),
        "credit" => "credit".to_string(),
        _ => "other".to_string(),
    }
}

pub fn get_transaction_type(transaction: &TellerTransaction) -> String {
    match transaction.r#type.as_str() {
        "ach" => "transfer".to_string(),
        "wire" => "transfer".to_string(),
        "payment" => "payment".to_string(),
        _ => "other".to_string(),
    }
}
