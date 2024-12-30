use super::types::*;

pub fn get_account_type(account: &PlaidAccount) -> String {
    match account.r#type.as_str() {
        "depository" => "checking".to_string(),
        "credit" => "credit".to_string(),
        "loan" => "loan".to_string(),
        "investment" => "investment".to_string(),
        _ => "other".to_string(),
    }
}

pub fn get_transaction_type(transaction: &PlaidTransaction) -> String {
    if let Some(ref category) = transaction.category {
        if category.contains(&"Payment".to_string()) {
            return "payment".to_string();
        }
        if category.contains(&"Transfer".to_string()) {
            return "transfer".to_string();
        }
    }
    "other".to_string()
}
