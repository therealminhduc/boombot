use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::env;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminUser {
    pub id: Option<i32>,
    pub username: String,
    pub password: String, // This should be hashed
}

#[derive(Debug, Error)]
pub enum AdminError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Admin not found")]
    NotFound,
    #[error("Username already exists")]
    UsernameExists,
}

pub type Result<T> = std::result::Result<T, AdminError>;

/// Initialize the admins table schema
pub fn init_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS admins (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

/// Insert a new admin user
pub fn insert_admin(conn: &Connection, username: &str, password_hash: &str) -> Result<i64> {
    match conn.execute(
        "INSERT INTO admins (username, password) VALUES (?1, ?2)",
        params![username, password_hash],
    ) {
        Ok(_) => Ok(conn.last_insert_rowid()),
        Err(rusqlite::Error::SqliteFailure(err, _)) if err.code == rusqlite::ErrorCode::ConstraintViolation => {
            Err(AdminError::UsernameExists)
        }
        Err(e) => Err(AdminError::Sqlite(e)),
    }
}

/// Get admin by username
pub fn get_admin_by_username(conn: &Connection, username: &str) -> Result<AdminUser> {
    let mut stmt = conn.prepare("SELECT id, username, password FROM admins WHERE username = ?1")?;
    
    let admin = stmt.query_row(params![username], |row| {
        Ok(AdminUser {
            id: Some(row.get::<_, i64>(0)? as i32),
            username: row.get(1)?,
            password: row.get(2)?,
        })
    });

    match admin {
        Ok(admin) => Ok(admin),
        Err(rusqlite::Error::QueryReturnedNoRows) => Err(AdminError::NotFound),
        Err(e) => Err(AdminError::Sqlite(e)),
    }
}

/// Check if any admin exists
pub fn has_admin(conn: &Connection) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM admins",
        [],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

/// Get all admins (for management purposes)
pub fn get_all_admins(conn: &Connection) -> Result<Vec<AdminUser>> {
    let mut stmt = conn.prepare("SELECT id, username, password FROM admins")?;
    
    let admins = stmt.query_map([], |row| {
        Ok(AdminUser {
            id: Some(row.get::<_, i64>(0)? as i32),
            username: row.get(1)?,
            password: row.get(2)?,
        })
    })?;

    let mut result = Vec::new();
    for admin in admins {
        result.push(admin?);
    }
    Ok(result)
}

/// Delete admin by username
pub fn delete_admin(conn: &Connection, username: &str) -> Result<()> {
    let rows_affected = conn.execute(
        "DELETE FROM admins WHERE username = ?1",
        params![username],
    )?;

    if rows_affected == 0 {
        Err(AdminError::NotFound)
    } else {
        Ok(())
    }
}

/// Create the first admin from environment variables if none exists
pub fn create_first_admin_if_needed(conn: &rusqlite::Connection) {
    match has_admin(conn) {
        Ok(false) => {
            let username = env::var("ADMIN_USERNAME").ok();
            let password = env::var("ADMIN_PASSWORD").ok();
            if let (Some(username), Some(password)) = (username, password) {
                match hash(&password, DEFAULT_COST) {
                    Ok(password_hash) => {
                        match insert_admin(conn, &username, &password_hash) {
                            Ok(_) => {
                                println!("First admin created from environment variables");
                            },
                            Err(e) => eprintln!("Failed to insert first admin: {e}"),
                        }
                    },
                    Err(e) => eprintln!("Failed to hash admin password: {e}"),
                }
            } else {
                eprintln!("No admin exists and ADMIN_USERNAME or ADMIN_PASSWORD not set. No admin created.");
            }
        },
        Ok(true) => {}, // At least one admin exists
        Err(e) => eprintln!("Failed to check for existing admin: {e}"),
    }
}
