use std::collections::HashMap;
use crate::cleaners::DomainCleaner;

#[cfg(test)]
use crate::traits::UrlCleaner;

pub fn build_cleaner_registry() -> HashMap<&'static str, DomainCleaner> {
    let mut registry = HashMap::new();

    registry.insert(
        "instagram.com",
        DomainCleaner::new(&["igsh"], &["utm_"]),
    );

    registry.insert(
        "linkedin.com",
        DomainCleaner::new(&["rcm"], &["utm_"]),
    );

    registry.insert(
        "default",
        DomainCleaner::new(&[], &["utm_"]),
    );

    registry
}

/// Retrieves the appropriate URL cleaner for a given host
/// 
/// Note: ' is to introduce a lifetime annotation.
/// 
/// Since I still have confusion about lifetimes, here is the function's explanation in plain English:
/// This function receives a reference to a registry that lives at least 'a
/// and it will return a reference to one of the values inside that registry. That returned reference cannot outlive the registry reference.
pub fn get_cleaner_for_host<'a>(
    host: &str,
    registry: &'a HashMap<&'static str, DomainCleaner>,
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

    #[test]
    fn builds_registry_with_expected_domains() {
        let registry = build_cleaner_registry();
        
        assert!(registry.contains_key("instagram.com"));
        assert!(registry.contains_key("linkedin.com"));
        assert!(registry.contains_key("default"));
    }

    #[test]
    fn gets_specific_cleaner_for_domain() {
        let registry = build_cleaner_registry();
        
        let instagram_cleaner = get_cleaner_for_host("www.instagram.com", &registry);
        assert!(instagram_cleaner.should_remove("igsh"));
        
        let linkedin_cleaner = get_cleaner_for_host("https://www.linkedin.com", &registry);
        assert!(linkedin_cleaner.should_remove("rcm"));
    }

    #[test]
    fn falls_back_to_default_cleaner() {
        let registry = build_cleaner_registry();
        
        let default_cleaner = get_cleaner_for_host("unknown.com", &registry);
        assert!(default_cleaner.should_remove("utm_source"));
    }
}