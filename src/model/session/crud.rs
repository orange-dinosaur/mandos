use redis::cmd;
use uuid::Uuid;

use crate::error::{Error, Result};

use super::SessionDb;

/// Create a new session in the session db
/// Returns the session id
/// # Arguments
/// * `session_db` - The session db connection pool
/// * `value` - The value to store in the session db
/// * `expiration` - The expiration time of the session in seconds
pub async fn create(session_db: SessionDb, value: String, expiration: u64) -> Result<String> {
    // get connection to session db
    let mut session_db_conn = session_db.get().await.map_err(Error::RedisPool)?;

    // generate random key
    let key = Uuid::new_v4().to_string();

    // save in the db
    cmd("SET")
        .arg(&[key.clone(), value, "EX".to_string(), expiration.to_string()])
        .query_async(&mut session_db_conn)
        .await
        .map_err(Error::Redis)?;

    Ok(key)
}

/// Get a session from the session db
/// Returns the value of the session
/// # Arguments
/// * `session_db` - The session db connection pool
/// * `key` - The key of the session
pub async fn get(session_db: SessionDb, key: String) -> Result<(String, String)> {
    // get connection to session db
    let mut session_db_conn = session_db.get().await.map_err(Error::RedisPool)?;

    // get the value from the db
    let value = cmd("GET")
        .arg(&[&key])
        .query_async(&mut session_db_conn)
        .await
        .map_err(Error::Redis)?;

    Ok((key, value))
}

/// Delete a session from the session db
/// # Arguments
/// * `session_db` - The session db connection pool
/// * `key` - The key of the session
pub async fn delete(session_db: SessionDb, key: String) -> Result<()> {
    // get connection to session db
    let mut session_db_conn = session_db.get().await.map_err(Error::RedisPool)?;

    // delete the value from the db
    cmd("DEL")
        .arg(&[key])
        .query_async(&mut session_db_conn)
        .await
        .map_err(Error::Redis)?;

    Ok(())
}
