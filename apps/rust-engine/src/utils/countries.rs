use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref COUNTRY_CODES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("US", "United States");
        m.insert("GB", "United Kingdom");
        m.insert("CA", "Canada");
        m.insert("AU", "Australia");
        m.insert("NZ", "New Zealand");
        m.insert("IE", "Ireland");
        m.insert("FR", "France");
        m.insert("DE", "Germany");
        m.insert("ES", "Spain");
        m.insert("IT", "Italy");
        m
    };
}

pub fn get_country_name(country_code: &str) -> Option<&'static str> {
    COUNTRY_CODES.get(country_code).copied()
}

pub fn is_supported_country(country_code: &str) -> bool {
    COUNTRY_CODES.contains_key(country_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_country_name() {
        assert_eq!(get_country_name("US"), Some("United States"));
        assert_eq!(get_country_name("GB"), Some("United Kingdom"));
        assert_eq!(get_country_name("XX"), None);
    }

    #[test]
    fn test_is_supported_country() {
        assert!(is_supported_country("US"));
        assert!(is_supported_country("GB"));
        assert!(!is_supported_country("XX"));
    }
}
