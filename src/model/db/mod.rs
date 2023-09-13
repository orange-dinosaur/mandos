use std::time::Duration;

use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    Pool, Postgres,
};

use crate::{
    config::config,
    error::{Error, Result},
};

pub type Db = Pool<Postgres>;
pub type DbRow = PgRow;

pub mod crud;

pub async fn new_db_pool() -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(config().DB_MAX_CONNECTIONS)
        .acquire_timeout(Duration::from_millis(5000))
        .connect(&config().DB_URL)
        .await
        .map_err(Error::Sqlx)
}
