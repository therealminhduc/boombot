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
    pub contributors: Vec<String>,
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

/// Initialize the domain rules schema
pub fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute(
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

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_domain_rules_status ON domain_rules(status)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_domain_rules_domain ON domain_rules(domain)",
        [],
    )?;
    
    Ok(())
}

/// Insert a new rule into the database
pub fn insert_rule(conn: &Connection, rule: &DomainRule) -> Result<i64> {
    let keys_json = serde_json::to_string(&rule.keys)?;
    let starts_with_json = serde_json::to_string(&rule.starts_with)?;
    let contributors_json = serde_json::to_string(&rule.contributors)?;

    conn.execute(
        "INSERT INTO domain_rules (domain, keys, starts_with, contributor, status) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            rule.domain,
            keys_json,
            starts_with_json,
            contributors_json,
            rule.status,
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

/// Upsert a rule (insert or update existing)
pub fn upsert_rule(conn: &Connection, rule: &DomainRule) -> Result<i64> {
    let mut stmt = conn.prepare(
        "SELECT id, keys, starts_with, contributor FROM domain_rules WHERE domain = ? AND status = ?",
    )?;

    let mut rows = stmt.query(params![rule.domain, rule.status])?;

    if let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        let mut keys: Vec<String> = serde_json::from_str(&row.get::<_, String>(1)?)?;
        let mut starts_with: Vec<String> = serde_json::from_str(&row.get::<_, String>(2)?)?;
        let mut contributors: Vec<String> = serde_json::from_str(&row.get::<_, String>(3)?)?;

        // Merge and deduplicate
        keys.extend(rule.keys.clone());
        keys.sort();
        keys.dedup();

        starts_with.extend(rule.starts_with.clone());
        starts_with.sort();
        starts_with.dedup();

        contributors.extend(rule.contributors.clone());
        contributors.sort();
        contributors.dedup();

        // Update the existing row
        conn.execute(
            "UPDATE domain_rules SET keys = ?, starts_with = ?, contributor = ? WHERE id = ?",
            params![
                serde_json::to_string(&keys)?,
                serde_json::to_string(&starts_with)?,
                serde_json::to_string(&contributors)?,
                id
            ],
        )?;

        Ok(id)
    } else {
        insert_rule(conn, rule)
    }
}

/// Get approved rules as a HashMap of DomainCleaner objects
pub fn get_approved_rules(conn: &Connection) -> Result<HashMap<String, DomainCleaner>> {
    let mut stmt = conn.prepare(
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
pub fn get_all_rules(conn: &Connection) -> Result<Vec<DomainRule>> {
    let mut stmt = conn.prepare(
        "SELECT id, domain, keys, starts_with, contributor, status FROM domain_rules ORDER BY id DESC"
    )?;

    let rules = stmt.query_map([], |row| {
        let id: i64 = row.get(0)?;
        let domain: String = row.get(1)?;
        let keys_json: String = row.get(2)?;
        let starts_with_json: String = row.get(3)?;
        let contributors_json: String = row.get(4)?;
        let status: String = row.get(5)?;

        let keys: Vec<String> = serde_json::from_str(&keys_json)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        let starts_with: Vec<String> = serde_json::from_str(&starts_with_json)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        let contributors: Vec<String> = serde_json::from_str(&contributors_json)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        // Create a DomainRule object and return it
        Ok(DomainRule {
            id: Some(id as i32),
            domain,
            keys,
            starts_with,
            contributors,
            status,
        })
    })?.collect::<SqliteResult<Vec<_>>>()?;

    Ok(rules)
}

/// Get approved rules as DomainRule objects for the API
pub fn get_approved_rules_for_api(conn: &Connection) -> Result<Vec<DomainRule>> {
    let mut stmt = conn.prepare(
        "SELECT id, domain, keys, starts_with, contributor, status FROM domain_rules WHERE status = 'approved'"
    )?;

    let rules = stmt.query_map([], |row| {
        let id: i64 = row.get(0)?;
        let domain: String = row.get(1)?;
        let keys_json: String = row.get(2)?;
        let starts_with_json: String = row.get(3)?;
        let contributor: String = row.get(4)?;
        let status: String = row.get(5)?;

        let keys: Vec<String> = serde_json::from_str(&keys_json)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        let starts_with: Vec<String> = serde_json::from_str(&starts_with_json)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        let contributors: Vec<String> = serde_json::from_str(&contributor)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        // Create a DomainRule object and return it
        Ok(DomainRule {
            id: Some(id as i32),
            domain,
            keys,
            starts_with,
            contributors,
            status,
        })
    })?.collect::<SqliteResult<Vec<_>>>()?;

    Ok(rules)
}

/// Get pending rules for the API
pub fn get_pending_rules_for_api(conn: &Connection) -> Result<Vec<DomainRule>> {
    let mut stmt = conn.prepare(
        "SELECT id, domain, keys, starts_with, contributor, status FROM domain_rules WHERE status = 'pending'"
    )?;
    
    let rules = stmt.query_map([], |row| {
        let id: i64 = row.get(0)?;
        let domain: String = row.get(1)?;
        let keys_json: String = row.get(2)?;
        let starts_with_json: String = row.get(3)?;
        let contributor: String = row.get(4)?;
        let status: String = row.get(5)?;

        let keys: Vec<String> = serde_json::from_str(&keys_json)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        let starts_with: Vec<String> = serde_json::from_str(&starts_with_json)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        let contributors: Vec<String> = serde_json::from_str(&contributor)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        // Create a DomainRule object and return it
        Ok(DomainRule {
            id: Some(id as i32),
            domain,
            keys,
            starts_with,
            contributors,
            status,
        })
    })?.collect::<SqliteResult<Vec<_>>>()?;

    Ok(rules)
}

/// Update rule status
pub fn update_rule_status(conn: &Connection, id: i64, status: &str) -> Result<()> {
    conn.execute(
        "UPDATE domain_rules SET status = ? WHERE id = ?",
        params![status, id],
    )?;
    Ok(())
}

/// Migrate rules from YAML to database
pub fn migrate_from_yaml(conn: &Connection) -> Result<()> {
    // Check if database is empty
    let count: i64 = conn.query_row(
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
                contributors: vec!["system".to_string()],
                status: "approved".to_string(),
            };
            insert_rule(conn, &rule)?;
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
            contributors: vec!["system".to_string()],
            status: "approved".to_string(),
        };

        insert_rule(conn, &rule)?;
        migrated_count += 1;
    }

    tracing::info!("Successfully migrated {} rules from YAML to database", migrated_count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_database_operations() {
        // Clean up any existing test database
        let _ = fs::remove_file("test.db");
        
        // Create database connection
        let conn = Connection::open("test.db").expect("Should create database");
        init_schema(&conn).expect("Should initialize schema");
        
        // Create a test rule
        let test_rule = DomainRule {
            id: None,
            domain: "test.com".to_string(),
            keys: vec!["test_key".to_string()],
            starts_with: vec!["test_".to_string()],
            contributors: vec!["test@example.com".to_string()],
            status: "approved".to_string(),
        };
        
        // Insert rule
        let id = insert_rule(&conn, &test_rule).expect("Should insert rule");
        assert!(id > 0);
        
        // Get approved rules
        let rules = get_approved_rules(&conn).expect("Should get approved rules");
        assert!(rules.contains_key("test.com"));
        
        // Clean up
        let _ = fs::remove_file("test.db");
    }
}


