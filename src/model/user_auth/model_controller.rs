use sqlx::FromRow;
use uuid::Uuid;

use crate::model::iterable::IterableType;
use crate::model::{session, ModelManager};
use crate::{error::Result, model::db};

use super::{UserAuth, UserAuthForCreate, UserAuthForUpdate};

const TABLE_NAME: &str = "users_auth";

pub struct UserAuthBmc;

impl UserAuthBmc {
    // region: Db CRUD operations

    pub async fn create(model_manager: &ModelManager, ua_fc: UserAuthForCreate) -> Result<Uuid> {
        let user_auth = UserAuth::new(ua_fc)?;

        let res = db::crud::create(model_manager.db().clone(), TABLE_NAME, user_auth).await?;

        let user_auth_created = UserAuth::from_row(&res)?;

        Ok(user_auth_created.id)
    }

    pub async fn get(model_manager: &ModelManager, id: Uuid) -> Result<UserAuth> {
        let res = db::crud::get_one_by_id(model_manager.db().clone(), TABLE_NAME, id).await?;

        let user_auth = UserAuth::from_row(&res)?;

        Ok(user_auth)
    }

    pub async fn get_from_username(
        model_manager: &ModelManager,
        username: String,
    ) -> Result<UserAuth> {
        let res = db::crud::get_one_by_field(
            model_manager.db().clone(),
            TABLE_NAME,
            "username",
            IterableType::String(username),
        )
        .await?;

        let user_auth = UserAuth::from_row(&res)?;

        Ok(user_auth)
    }

    pub async fn get_from_email(
        model_manager: &ModelManager,
        username: String,
    ) -> Result<UserAuth> {
        let res = db::crud::get_one_by_field(
            model_manager.db().clone(),
            TABLE_NAME,
            "email",
            IterableType::String(username),
        )
        .await?;

        let user_auth = UserAuth::from_row(&res)?;

        Ok(user_auth)
    }

    pub async fn get_all(model_manager: &ModelManager) -> Result<Vec<UserAuth>> {
        let res = db::crud::get_all(model_manager.db().clone(), TABLE_NAME).await?;

        let mut user_auths = Vec::new();
        for user_auth in res {
            let ua = UserAuth::from_row(&user_auth)?;
            user_auths.push(ua);
        }

        Ok(user_auths)
    }

    pub async fn update(
        model_manager: &ModelManager,
        ua_fu: UserAuthForUpdate,
        id: Uuid,
    ) -> Result<()> {
        db::crud::update_by_id(model_manager.db.clone(), TABLE_NAME, ua_fu, id).await?;

        Ok(())
    }

    pub async fn delete(model_manager: &ModelManager, id: Uuid) -> Result<()> {
        db::crud::delete_by_id(model_manager.db.clone(), TABLE_NAME, id).await?;

        Ok(())
    }

    // endregion: Db CRUD operations

    // region: Session Db CRUD operations

    pub async fn create_session(
        model_manager: &ModelManager,
        value: String,
        expiration: u64,
    ) -> Result<String> {
        let res =
            session::crud::create(model_manager.session_db().clone(), value, expiration).await?;

        Ok(res)
    }

    pub async fn get_session(
        model_manager: &ModelManager,
        session_id: String,
    ) -> Result<(String, String)> {
        let res = session::crud::get(model_manager.session_db().clone(), session_id).await?;

        Ok(res)
    }

    pub async fn delete_session(model_manager: &ModelManager, session_id: String) -> Result<()> {
        session::crud::delete(model_manager.session_db().clone(), session_id).await?;

        Ok(())
    }

    // endregion: Db CRUD operations
}
