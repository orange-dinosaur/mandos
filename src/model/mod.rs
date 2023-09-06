use tracing::info;

use crate::error::Result;
use crate::model::db::new_db_pool;

use self::db::Db;

pub mod db;
mod iterable;
pub mod user_auth;

#[derive(Clone, Debug)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    /// Constructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        sqlx::migrate!("./migrations").run(&db).await?;

        info!("Connected to DB");

        // Connect to the SessionDB

        Ok(ModelManager { db })
    }

    /// Returns a reference to the database pool
    /// (Only for the model layer)
    pub fn db(&self) -> &Db {
        &self.db
    }
}
