use crate::providers::types::Transaction;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnrichedTransaction {
    pub transaction: Transaction,
    pub enriched_description: Option<String>,
    pub enriched_category: Option<String>,
    pub enriched_merchant: Option<String>,
    pub logo_url: Option<String>,
}

lazy_static! {
    static ref CATEGORY_PATTERNS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("groceries", vec![
            r"(?i)TRADER.*JOE",
            r"(?i)WHOLE.*FOODS",
            r"(?i)SAFEWAY",
            r"(?i)KROGER",
            r"(?i)ALBERTSONS",
        ]);
        m.insert("transportation", vec![
            r"(?i)UBER",
            r"(?i)LYFT",
            r"(?i)TAXI",
            r"(?i)TRANSIT",
            r"(?i)METRO",
        ]);
        m.insert("dining", vec![
            r"(?i)RESTAURANT",
            r"(?i)CAFE",
            r"(?i)COFFEE",
            r"(?i)STARBUCKS",
            r"(?i)MCDONALD",
        ]);
        m
    };

    static ref MERCHANT_PATTERNS: Vec<(&'static str, Regex)> = {
        vec![
            ("Trader Joe's", Regex::new(r"(?i)TRADER.*JOE").unwrap()),
            ("Whole Foods", Regex::new(r"(?i)WHOLE.*FOODS").unwrap()),
            ("Uber", Regex::new(r"(?i)UBER").unwrap()),
            ("Lyft", Regex::new(r"(?i)LYFT").unwrap()),
            ("Starbucks", Regex::new(r"(?i)STARBUCKS").unwrap()),
        ]
    };
}

pub fn enrich_transaction(transaction: Transaction) -> EnrichedTransaction {
    let enriched_description = Some(
        transaction
            .description
            .as_str()
            .trim()
            .to_string()
    );

    let enriched_category = transaction.category.clone();
    let enriched_merchant = transaction.merchant.clone();
    let logo_url = enriched_merchant.as_ref().map(|m| get_logo_url(m));

    EnrichedTransaction {
        transaction,
        enriched_description,
        enriched_category,
        enriched_merchant,
        logo_url,
    }
}

fn detect_category(description: &str) -> Option<String> {
    for (category, patterns) in CATEGORY_PATTERNS.iter() {
        for pattern in patterns {
            if Regex::new(pattern).unwrap().is_match(description) {
                return Some(category.to_string());
            }
        }
    }
    None
}

fn detect_merchant(description: &str) -> Option<String> {
    for (merchant, pattern) in MERCHANT_PATTERNS.iter() {
        if pattern.is_match(description) {
            return Some(merchant.to_string());
        }
    }
    None
}

fn get_logo_url(merchant: &str) -> String {
    // In a real implementation, this would fetch from a logo service or CDN
    format!("https://api.example.com/logos/{}", merchant.to_lowercase().replace(' ', "-"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use chrono::Utc;

    fn create_test_transaction(description: &str) -> Transaction {
        Transaction {
            id: Uuid::new_v4().to_string(),
            description: Some(description.to_string()),
            amount: 100.0,
            date: Utc::now(),
            currency: "USD".to_string(),
            account_id: "acc_123".to_string(),
            category: None,
            merchant: None,
            pending: false,
        }
    }

    #[test]
    fn test_enrich_transaction() {
        let transaction = create_test_transaction("TRADER JOE'S #123");
        let enriched = enrich_transaction(transaction);

        assert_eq!(enriched.enriched_category, None);
        assert_eq!(enriched.enriched_merchant, None);
        assert!(enriched.logo_url.is_none());
    }
}
