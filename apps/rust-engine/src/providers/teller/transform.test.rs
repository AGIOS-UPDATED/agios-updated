#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_transform_account() {
        let teller_account = TellerAccount {
            id: "acc123".to_string(),
            currency: "USD".to_string(),
            enrollment_id: "enr123".to_string(),
            institution: TellerInstitution {
                id: "inst123".to_string(),
                name: "Test Bank".to_string(),
                capabilities: vec!["accounts".to_string()],
            },
            last_four: "1234".to_string(),
            links: TellerLinks {
                balances: "https://api.teller.io/accounts/acc123/balances".to_string(),
                transactions: "https://api.teller.io/accounts/acc123/transactions".to_string(),
            },
            name: "Checking Account".to_string(),
            r#type: "depository".to_string(),
            subtype: Some("checking".to_string()),
            status: "open".to_string(),
        };

        let account = transform_account(&teller_account);
        assert_eq!(account.id, "acc123");
        assert_eq!(account.name, "Checking Account");
        assert_eq!(account.account_type, "checking");
        assert_eq!(account.currency, "USD");
    }

    #[test]
    fn test_transform_transaction() {
        let teller_transaction = TellerTransaction {
            id: "tx123".to_string(),
            account_id: "acc123".to_string(),
            amount: "50.00".to_string(),
            date: Utc.ymd(2023, 1, 1).and_hms(0, 0, 0),
            description: "Coffee Shop".to_string(),
            details: TellerTransactionDetails {
                category: Some("food_and_drink".to_string()),
                counterparty: None,
                processing_status: "complete".to_string(),
            },
            running_balance: Some("1000.00".to_string()),
            status: "posted".to_string(),
            r#type: "pos".to_string(),
        };

        let transaction = transform_transaction(&teller_transaction);
        assert_eq!(transaction.id, "tx123");
        assert_eq!(transaction.account_id, "acc123");
        assert_eq!(transaction.amount, 50.0);
        assert_eq!(transaction.currency, "USD");
        assert_eq!(transaction.description, "Coffee Shop");
        assert_eq!(transaction.category, Some("food_and_drink".to_string()));
    }
}
