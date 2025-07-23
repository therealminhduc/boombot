use rusqlite::Connection;
use thiserror::Error;

pub mod rules;
pub mod admin;

// Re-export commonly used types
pub use rules::{DomainRule, DatabaseError, Result};

#[derive(Debug, Error)]
pub enum InitError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Rules error: {0}")]
    Rules(#[from] rules::DatabaseError),
}

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create a new database
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path).map_err(DatabaseError::Sqlite)?;
        let db = Database { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Initialize the schema for all modules
    fn init_schema(&self) -> Result<()> {
        rules::init_schema(&self.conn)?;
        admin::init_schema(&self.conn).map_err(DatabaseError::Sqlite)?;
        Ok(())
    }

    /// Get a reference to the connection for submodules
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    // Delegate domain rule methods to rules module
    pub fn insert_rule(&self, rule: &DomainRule) -> Result<i64> {
        rules::insert_rule(&self.conn, rule)
    }

    pub fn upsert_rule(&self, rule: &DomainRule) -> Result<i64> {
        rules::upsert_rule(&self.conn, rule)
    }

    pub fn get_approved_rules(&self) -> Result<std::collections::HashMap<String, crate::cleaners::DomainCleaner>> {
        rules::get_approved_rules(&self.conn)
    }

    pub fn get_all_rules(&self) -> Result<Vec<DomainRule>> {
        rules::get_all_rules(&self.conn)
    }

    pub fn get_approved_rules_for_api(&self) -> Result<Vec<DomainRule>> {
        rules::get_approved_rules_for_api(&self.conn)
    }

    pub fn get_pending_rules_for_api(&self) -> Result<Vec<DomainRule>> {
        rules::get_pending_rules_for_api(&self.conn)
    }

    pub fn update_rule_status(&self, id: i64, status: &str) -> Result<()> {
        rules::update_rule_status(&self.conn, id, status)
    }

    pub fn migrate_from_yaml(&self) -> Result<()> {
        rules::migrate_from_yaml(&self.conn)
    }
}