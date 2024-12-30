use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOGO_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("chase", "https://logo.clearbit.com/chase.com");
        m.insert("bank_of_america", "https://logo.clearbit.com/bankofamerica.com");
        m.insert("wells_fargo", "https://logo.clearbit.com/wellsfargo.com");
        m.insert("citi", "https://logo.clearbit.com/citi.com");
        m.insert("capital_one", "https://logo.clearbit.com/capitalone.com");
        m
    };
}

pub fn get_institution_logo(institution_id: &str) -> Option<&'static str> {
    LOGO_MAP.get(institution_id).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_institution_logo() {
        assert_eq!(
            get_institution_logo("chase"),
            Some("https://logo.clearbit.com/chase.com")
        );
        assert_eq!(get_institution_logo("unknown"), None);
    }
}
