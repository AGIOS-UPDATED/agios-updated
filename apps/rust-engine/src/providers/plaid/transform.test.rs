#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_transform_account() {
        let plaid_account = PlaidAccount {
            account_id: "acc123".to_string(),
            balances: PlaidBalances {
                available: Some(1000.0),
                current: 1000.0,
                limit: None,
                iso_currency_code: Some("USD".to_string()),
                unofficial_currency_code: None,
            },
            mask: Some("1234".to_string()),
            name: "Checking Account".to_string(),
            official_name: Some("Premium Checking".to_string()),
            r#type: "depository".to_string(),
            subtype: Some("checking".to_string()),
            verification_status: Some("verified".to_string()),
        };

        let account = transform_account(&plaid_account);
        assert_eq!(account.id, "acc123");
        assert_eq!(account.name, "Checking Account");
        assert_eq!(account.account_type, "checking");
        assert_eq!(account.balance, 1000.0);
        assert_eq!(account.currency, "USD");
    }

    #[test]
    fn test_transform_transaction() {
        let plaid_transaction = PlaidTransaction {
            transaction_id: "tx123".to_string(),
            account_id: "acc123".to_string(),
            amount: 50.0,
            iso_currency_code: Some("USD".to_string()),
            unofficial_currency_code: None,
            date: Utc.ymd(2023, 1, 1).and_hms(0, 0, 0),
            name: "Coffee Shop".to_string(),
            merchant_name: Some("Starbucks".to_string()),
            payment_channel: "in_store".to_string(),
            pending: false,
            transaction_type: Some("place".to_string()),
            category: Some(vec!["Food and Drink".to_string()]),
            location: None,
        };

        let transaction = transform_transaction(&plaid_transaction);
        assert_eq!(transaction.id, "tx123");
        assert_eq!(transaction.account_id, "acc123");
        assert_eq!(transaction.amount, 50.0);
        assert_eq!(transaction.currency, "USD");
        assert_eq!(transaction.description, "Coffee Shop");
        assert_eq!(transaction.merchant, Some("Starbucks".to_string()));
        assert_eq!(transaction.category, Some("Food and Drink".to_string()));
    }
}
