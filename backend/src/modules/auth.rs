use argonautica::{Hasher, Verifier};
use uuid::Uuid;
use rusqlite::Connection;
use rusqlite::params;

const SECRET_KEY: &str = "please_change_this_secret_in_prod";

pub fn hash_password(password: &str) -> Result<String, String> {
    let mut hasher = Hasher::default();
    hasherwith(&mut hasher, SECRET_KEY);
    hasher.configure_secret_key(SECRET_KEY);
    hasher.with_password(password).map_err(|e| format!("hash err {:?}", e))
}

fn hasherwith(h: &mut Hasher, _s: &str) {
    // placeholder - argonautica Hasher is configured above
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let mut verifier = Verifier::default();
    verifier.configure_secret_key(SECRET_KEY);
    verifier.with_password(password).with_hash(hash).map_err(|e| format!("verify err {:?}", e))
}

pub fn create_token(conn: &Connection, username: &str) -> rusqlite::Result<String> {
    let token = Uuid::new_v4().to_string();
    conn.execute("UPDATE users SET token = ?1 WHERE username = ?2", params![token, username])?;
    Ok(token)
}

pub fn get_user_by_token(conn: &Connection, token: &str) -> rusqlite::Result<Option<(i64,String)>> {
    let mut stmt = conn.prepare("SELECT id, role FROM users WHERE token = ?1")?;
    let mut rows = stmt.query(params![token])?;
    if let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        let role: String = row.get(1)?;
        Ok(Some((id, role)))
    } else {
        Ok(None)
    }
}