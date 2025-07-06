use rusqlite::{Connection, Result as SqliteResult, params};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::cleaners::DomainCleaner;
use thiserror::Error;
use tracing;

/// DomainRule struct for the database
#[derive(Debug, Serialize, Deserialize)]
pub struct DomainRule {
    pub id: Option<i32>,
    pub domain: String,
    pub keys: Vec<String>,
    pub starts_with: Vec<String>,
    pub contributor: Option<String>,
    pub status: String, // "pending", "approved", "rejected"
}

/// DatabaseError enum for the database
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result type for the database
pub type Result<T> = std::result::Result<T, DatabaseError>;

/// Initialize the database
pub struct Database {
    conn: Connection,
}

/// Database implementation
impl Database {

    /// Create a new database
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Database { conn };
        db.init_schema()?;

        Ok(db)
    }

    /// Initialize the schema
    fn init_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS domain_rules (
                id INTEGER PRIMARY KEY,
                domain TEXT NOT NULL,
                keys TEXT NOT NULL,
                starts_with TEXT NOT NULL,
                contributor TEXT,
                status TEXT DEFAULT 'pending'
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_domain_rules_status ON domain_rules(status)",
            [],
        )?;
        
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_domain_rules_domain ON domain_rules(domain)",
            [],
        )?;
        
        Ok(())
    }

    /// Insert a new rule into the database
    pub fn insert_rule(&self, rule: &DomainRule) -> Result<i64> {
        let keys_json = serde_json::to_string(&rule.keys)?;
        let starts_with_json = serde_json::to_string(&rule.starts_with)?;

        self.conn.execute(
            "INSERT INTO domain_rules (domain, keys, starts_with, contributor, status) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                rule.domain,
                keys_json,
                starts_with_json,
                rule.contributor,
                rule.status,
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Get approved rules as a HashMap of DomainCleaner objects
    pub fn get_approved_rules(&self) -> Result<HashMap<String, DomainCleaner>> {
        let mut stmt = self.conn.prepare(
            "SELECT domain, keys, starts_with FROM domain_rules WHERE status = 'approved'"
        )?;

        let rules = stmt.query_map([], |row| {
            let domain: String = row.get(0)?;
            let keys_json: String = row.get(1)?;
            let starts_with_json: String = row.get(2)?;

            let keys: Vec<String> = serde_json::from_str(&keys_json)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

            let starts_with: Vec<String> = serde_json::from_str(&starts_with_json)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

            // Create a DomainCleaner object for the domain and return it
            let cleaner = DomainCleaner::new(
                &keys.iter().map(String::as_str).collect::<Vec<_>>(),
                &starts_with.iter().map(String::as_str).collect::<Vec<_>>(),
            );

            Ok((domain, cleaner))
        })?.collect::<SqliteResult<Vec<_>>>()?;

        Ok(rules.into_iter().collect())
    }

    /// Get all rules for the web API
    pub fn get_all_rules(&self) -> Result<Vec<DomainRule>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, domain, keys, starts_with, contributor, status FROM domain_rules ORDER BY id DESC"
        )?;

        let rules = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let domain: String = row.get(1)?;
            let keys_json: String = row.get(2)?;
            let starts_with_json: String = row.get(3)?;
            let contributor: Option<String> = row.get(4)?;
            let status: String = row.get(5)?;

            let keys: Vec<String> = serde_json::from_str(&keys_json)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

            let starts_with: Vec<String> = serde_json::from_str(&starts_with_json)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

            // Create a DomainRule object and return it
            Ok(DomainRule {
                id: Some(id as i32),
                domain,
                keys,
                starts_with,
                contributor,
                status,
            })
        })?.collect::<SqliteResult<Vec<_>>>()?;

        Ok(rules)
    }

    // Get approved rules as DomainRule objects for the API
    pub fn get_approved_rules_for_api(&self) -> Result<Vec<DomainRule>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, domain, keys, starts_with, contributor, status FROM domain_rules WHERE status = 'approved'"
        )?;

        let rules = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let domain: String = row.get(1)?;
            let keys_json: String = row.get(2)?;
            let starts_with_json: String = row.get(3)?;
            let contributor: Option<String> = row.get(4)?;
            let status: String = row.get(5)?;

            let keys: Vec<String> = serde_json::from_str(&keys_json)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

            let starts_with: Vec<String> = serde_json::from_str(&starts_with_json)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

            // Create a DomainRule object and return it
            Ok(DomainRule {
                id: Some(id as i32),
                domain,
                keys,
                starts_with,
                contributor,
                status,
            })
        })?.collect::<SqliteResult<Vec<_>>>()?;

        Ok(rules)
    }

    pub fn get_pending_rules_for_api(&self) -> Result<Vec<DomainRule>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, domain, keys, starts_with, contributor, status FROM domain_rules WHERE status = 'pending'"
        )?;
        
        let rules = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let domain: String = row.get(1)?;
            let keys_json: String = row.get(2)?;
            let starts_with_json: String = row.get(3)?;
            let contributor: Option<String> = row.get(4)?;
            let status: String = row.get(5)?;

            let keys: Vec<String> = serde_json::from_str(&keys_json)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

            let starts_with: Vec<String> = serde_json::from_str(&starts_with_json)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

            // Create a DomainRule object and return it
            Ok(DomainRule {
                id: Some(id as i32),
                domain,
                keys,
                starts_with,
                contributor,
                status,
            })
        })?.collect::<SqliteResult<Vec<_>>>()?;

        Ok(rules)
    }

    /// Update rule status
    pub fn update_rule_status(&self, id: i64, status: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE domain_rules SET status = ? WHERE id = ?",
            params![status, id],
        )?;
        Ok(())
    }

    /// Migrate rules from YAML to database
    pub fn migrate_from_yaml(&self) -> Result<()> {
        // Check if database is empty
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM domain_rules",
            [],
            |row| row.get(0),
        )?;

        // If database is not empty, skip migration
        if count > 0 {
            tracing::info!("Database already has {} rules, skipping migration", count);
            return Ok(());
        }

        let yaml_content = crate::config::DOMAIN_RULES_YAML;
        
        // Parse the YAML content into a serde_yaml::Value
        let config: serde_yaml::Value = serde_yaml::from_str(yaml_content)
            .map_err(|e| DatabaseError::Sqlite(rusqlite::Error::InvalidParameterName(e.to_string())))?;

        let domains = config["domains"].as_mapping()
            .ok_or_else(|| DatabaseError::Sqlite(rusqlite::Error::InvalidParameterName("Invalid YAML structure".to_string())))?;

        let default_keys = config["defaults"]["keys"].as_sequence()
            .unwrap_or(&serde_yaml::Sequence::new())
            .iter()
            .filter_map(|v| v.as_str())
            .map(String::from)
            .collect::<Vec<String>>();

        let default_starts_with = config["defaults"]["starts_with"].as_sequence()
            .unwrap_or(&serde_yaml::Sequence::new())
            .iter()
            .filter_map(|v| v.as_str())
            .map(String::from)
            .collect::<Vec<String>>();

        let mut migrated_count = 0;

        for (domain, rules) in domains {
            let domain_str = domain.as_str().ok_or_else(|| DatabaseError::Sqlite(rusqlite::Error::InvalidParameterName("Invalid domain".to_string())))?;

            // Include the "default" domain as it's needed for fallback
            if domain_str == "default" {
                // Use default settings for the default domain
                let rule = DomainRule {
                    id: None,
                    domain: domain_str.to_string(),
                    keys: default_keys.clone(),
                    starts_with: default_starts_with.clone(),
                    contributor: Some("system".to_string()),
                    status: "approved".to_string(),
                };
                self.insert_rule(&rule)?;
                migrated_count += 1;
                continue;
            }

            let keys = rules["keys"].as_sequence()
                .unwrap_or(&serde_yaml::Sequence::new())
                .iter()
                .filter_map(|v| v.as_str())
                .map(String::from)
                .collect::<Vec<String>>();

            let starts_with = rules["starts_with"].as_sequence()
                .unwrap_or(&serde_yaml::Sequence::new())
                .iter()
                .filter_map(|v| v.as_str())
                .map(String::from)
                .collect::<Vec<String>>();

            // Use domain-specific rules or fall back to defaults
            let final_keys = if keys.is_empty() { &default_keys } else { &keys };
            let final_starts_with = if starts_with.is_empty() { &default_starts_with } else { &starts_with };

            let rule = DomainRule {
                id: None,
                domain: domain_str.to_string(),
                keys: final_keys.clone(),
                starts_with: final_starts_with.clone(),
                contributor: Some("system".to_string()),
                status: "approved".to_string(),
            };

            self.insert_rule(&rule)?;
            migrated_count += 1;
        }

        tracing::info!("Successfully migrated {} rules from YAML to database", migrated_count);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_database_operations() {
        // Clean up any existing test database
        let _ = fs::remove_file("test.db");
        
        // Create database
        let db = Database::new("test.db").expect("Should create database");
        
        // Create a test rule
        let test_rule = DomainRule {
            id: None,
            domain: "test.com".to_string(),
            keys: vec!["test_key".to_string()],
            starts_with: vec!["test_".to_string()],
            contributor: Some("test@example.com".to_string()),
            status: "approved".to_string(),
        };
        
        // Insert rule
        let id = db.insert_rule(&test_rule).expect("Should insert rule");
        assert!(id > 0);
        
        // Get approved rules
        let rules = db.get_approved_rules().expect("Should get approved rules");
        assert!(rules.contains_key("test.com"));
        
        // Clean up
        let _ = fs::remove_file("test.db");
    }
}
