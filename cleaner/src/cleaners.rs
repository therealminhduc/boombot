use std::collections::HashSet;
use crate::traits::UrlCleaner;

pub struct DomainCleaner {
    /// Query parameters that should be removed if they exactly match a key in this set
    keys: HashSet<String>,

    /// Query parameters that should be removed if they start with a key in this set
    starts_with: HashSet<String>,
}

impl DomainCleaner {

    /// Create a new domain-specific cleaner
    pub fn new(keys: &[&str], starts_with: &[&str]) -> Self {
        Self {
            keys: keys.iter().map(|k| k.to_string()).collect(),
            starts_with: starts_with.iter().map(|k| k.to_string()).collect(),
        }
    }
}

impl UrlCleaner for DomainCleaner {
    fn should_remove(&self, key: &str) -> bool {
        self.keys.contains(key) || self.starts_with.iter().any(|prefix| key.starts_with(prefix))
    }
}

/* =============================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_exact_keys() {
        let cleaner = DomainCleaner::new(&["fbclid", "igsh"], &[]);
        
        assert!(cleaner.should_remove("fbclid"));
        assert!(cleaner.should_remove("igsh"));

        assert!(!cleaner.should_remove("keep"));
    }

    #[test]
    fn removes_prefix_keys() {
        let cleaner = DomainCleaner::new(&[], &["utm_", "track_"]);
        
        assert!(cleaner.should_remove("utm_source"));
        assert!(cleaner.should_remove("track_user"));

        assert!(!cleaner.should_remove("keep"));
    }

    #[test]
    fn combines_exact_and_prefix() {
        let cleaner = DomainCleaner::new(&["exact"], &["prefix_"]);
        
        assert!(cleaner.should_remove("exact"));
        assert!(cleaner.should_remove("prefix_anything"));

        assert!(!cleaner.should_remove("keep"));
    }
}