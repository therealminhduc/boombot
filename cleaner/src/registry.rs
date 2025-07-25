use std::collections::HashMap;
use crate::cleaners::DomainCleaner;

#[cfg(test)]
use crate::traits::UrlCleaner;

/// Retrieves the appropriate URL cleaner for a given host from a String-keyed registry
pub fn get_cleaner_for_host_string<'a>(
    host: &str,
    registry: &'a HashMap<String, DomainCleaner>,
) -> &'a DomainCleaner {
    for (domain, cleaner) in registry {
        if host.contains(domain) {
            return cleaner;
        }
    }

    registry.get("default").expect("default cleaner must exist")
}

/* =============================== */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_registry_with_fallback;

    #[test]
    fn loads_registry_from_database_with_fallback() {
        let registry = load_registry_with_fallback("test_rules.db")
            .expect("Should load registry");
        
        // Test that we can load the registry (either from DB or YAML fallback)
        assert!(!registry.is_empty());
    }

    #[test]
    fn gets_specific_cleaner_for_domain() {
        let registry = load_registry_with_fallback("test_rules.db")
            .expect("Should load registry");
        
        let instagram_cleaner = get_cleaner_for_host_string("www.instagram.com", &registry);
        assert!(instagram_cleaner.should_remove("igsh"));
        
        let linkedin_cleaner = get_cleaner_for_host_string("https://www.linkedin.com", &registry);
        assert!(linkedin_cleaner.should_remove("rcm"));
    }

    #[test]
    fn falls_back_to_default_cleaner() {
        let registry = load_registry_with_fallback("test_rules.db")
            .expect("Should load registry");
        
        let default_cleaner = get_cleaner_for_host_string("unknown.com", &registry);
        assert!(default_cleaner.should_remove("utm_source"));
    }
}