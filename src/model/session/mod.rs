use deadpool_redis::{Config, Pool, Runtime};

use crate::{
    config::config,
    error::{Error, Result},
};

pub type SessionDb = Pool;

pub mod crud;

// TODO: find a way to manage multiple user sessions

pub async fn new_session_db_conn() -> Result<SessionDb> {
    let cfg = Config::from_url(config().SESSION_DB_URL.as_str());

    cfg.create_pool(Some(Runtime::Tokio1))
        .map_err(Error::RedisCreatePool)
}
