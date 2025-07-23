use std::collections::HashMap;
use serde::Deserialize;
use std::error::Error;

use crate::cleaners::DomainCleaner;
use crate::database::Database;
use crate::database::rules::Result as DbResult;

pub const DOMAIN_RULES_YAML: &str = include_str!("config/domain_rules.yaml");


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

/// Load the registry from the embedded config file
pub fn load_registry_from_file() -> Result<HashMap<String, DomainCleaner>, Box<dyn Error>> {
    let config: ConfigFile = serde_yaml::from_str(DOMAIN_RULES_YAML)?;

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

/// Load registry from database
pub fn load_registry_from_database(_db_path: &str) -> DbResult<HashMap<String, DomainCleaner>> {
    let db = Database::new(_db_path)?;
    
    match db.migrate_from_yaml() {
        Ok(_) => {
            // Migration was successful (either performed or skipped)
            // The actual status is logged inside migrate_from_yaml()
        },
        Err(e) => {
            println!("Failed to migrate rules from YAML: {e}");
        }
    }
    
    db.get_approved_rules()
}

/// Fallback function to load registry from database or file
/// If database is not found, load from file
pub fn load_registry_with_fallback(_db_path: &str) -> Result<HashMap<String, DomainCleaner>, Box<dyn Error>> {
    match load_registry_from_database(_db_path) {
        Ok(registry) => {
            println!("Successfully loaded registry from database");
            Ok(registry)
        },
        Err(e) => {
            println!("Failed to load from database: {e}, falling back to YAML");
            load_registry_from_file()
        }
    }
}