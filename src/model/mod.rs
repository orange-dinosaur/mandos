use tracing::info;

use crate::error::Result;
use crate::model::db::new_db_pool;

use self::db::Db;
use self::session::SessionDb;

pub mod db;
mod iterable;
pub mod session;
pub mod user_auth;

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
    session_db: SessionDb,
}

impl ModelManager {
    /// Constructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        sqlx::migrate!("./migrations").run(&db).await?;
        info!("Connected to DB");

        // Connect to the Session DB
        let session_db = session::new_session_db_conn().await?;
        info!("Connected to Session DB");

        Ok(ModelManager { db, session_db })
    }

    /// Returns a reference to the database pool
    pub fn db(&self) -> &Db {
        &self.db
    }

    /// Returns a reference to the session database pool
    pub fn session_db(&self) -> &SessionDb {
        &self.session_db
    }
}
