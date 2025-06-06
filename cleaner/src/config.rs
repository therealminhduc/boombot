use std::collections::HashMap;
use serde::Deserialize;

use std::path::Path;
use std::error::Error;
use std::fs;

use crate::cleaners::DomainCleaner;


/// Configuration for a domain's URL cleaning rules
#[derive(Debug, Deserialize, Clone)]
pub struct RuleConfig {
    #[serde(default)]
    pub keys: Vec<String>,

    #[serde(default)]
    pub starts_with: Option<Vec<String>>,
}

/// Top level configuration structure with defaults and domain-specific rules
#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub defaults: RuleConfig,
    pub domains: HashMap<String, RuleConfig>,
}

pub fn load_registry_from_file(path: &Path) -> Result<HashMap<String, DomainCleaner>, Box<dyn Error>> {
    let yaml = fs::read_to_string(path)?;
    let config: ConfigFile = serde_yaml::from_str(&yaml)?;

    let mut registry = HashMap::new();

    for (domain, rule) in config.domains {
        // Merge with defaults: use domain-specific values if provided, otherwise use defaults
        let merged_keys = if rule.keys.is_empty() {
            config.defaults.keys.clone()
        } else {
            rule.keys
        };
        
        let merged_starts_with = rule.starts_with
            .or_else(|| config.defaults.starts_with.clone())
            .unwrap_or_else(|| vec!["utm_".to_string()]);

        let cleaner = DomainCleaner::new(
            &merged_keys.iter().map(String::as_str).collect::<Vec<_>>(),
            &merged_starts_with.iter().map(String::as_str).collect::<Vec<_>>(),
        );

        registry.insert(domain, cleaner);
    }

    Ok(registry)
}

