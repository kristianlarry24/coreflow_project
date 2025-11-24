use rusqlite::{Connection, params};
use std::path::Path;

pub fn init_db(db_path: &str) -> rusqlite::Result<Connection> {
    let new_db = !Path::new(db_path).exists();
    let conn = Connection::open(db_path)?;
    if new_db {
        conn.execute_batch(r#"
            PRAGMA foreign_keys = ON;
            CREATE TABLE users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                role TEXT NOT NULL DEFAULT 'user',
                token TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE audit_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER,
                action TEXT NOT NULL,
                details TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(user_id) REFERENCES users(id)
            );
        "#)?;
        // create default admin: username=admin, password=admin (please change)
        let hash = super::auth::hash_password("admin").unwrap_or_else(|_| "changeme".to_string());
        conn.execute("INSERT INTO users (username, password_hash, role) VALUES (?1, ?2, 'admin')",
            params!["admin", hash])?;
    }
    Ok(conn)
}