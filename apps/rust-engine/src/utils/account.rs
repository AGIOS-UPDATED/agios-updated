use crate::providers::types::{Account, AccountType};
use uuid::Uuid;

pub fn generate_account_id() -> String {
    format!("acc_{}", Uuid::new_v4())
}

pub fn normalize_account_type(account_type: &str) -> AccountType {
    match account_type.to_lowercase().as_str() {
        "checking" => AccountType::Checking,
        "savings" => AccountType::Savings,
        "credit" => AccountType::Credit,
        "loan" => AccountType::Loan,
        "investment" => AccountType::Investment,
        _ => AccountType::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_account_id() {
        let id = generate_account_id();
        assert!(id.starts_with("acc_"));
        assert_eq!(id.len(), 40); // "acc_" + 36 chars UUID
    }

    #[test]
    fn test_normalize_account_type() {
        assert_eq!(normalize_account_type("checking"), AccountType::Checking);
        assert_eq!(normalize_account_type("SAVINGS"), AccountType::Savings);
        assert_eq!(normalize_account_type("Credit"), AccountType::Credit);
        assert_eq!(normalize_account_type("unknown"), AccountType::Other);
    }
}
