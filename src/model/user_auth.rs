use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    utils,
};

use super::{
    db::crud,
    iterable::{Iterable, IterableType},
    ModelManager,
};

// region: UserAuth

#[derive(Clone, Debug, FromRow, Serialize)]
pub struct UserAuth {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub needs_verify: bool,
    pub is_blocked: bool,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl Default for UserAuth {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            last_login: None,
            needs_verify: false,
            is_blocked: false,
            username: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
        }
    }
}

impl UserAuth {
    pub fn new(ua_fc: UserAuthForCreate) -> Result<UserAuth> {
        UserAuthForCreate::validate(&ua_fc)?;

        // Hash password
        let ua_fc = ua_fc.hash_password()?;

        Ok(UserAuth {
            username: ua_fc.username,
            email: ua_fc.email,
            password: ua_fc.password,
            ..Default::default()
        })
    }
}

impl Iterable for UserAuth {
    fn get_fields(&self) -> (Vec<String>, Vec<IterableType>) {
        let fields_names = vec![
            "id".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
            "last_login".to_string(),
            "needs_verify".to_string(),
            "is_blocked".to_string(),
            "username".to_string(),
            "email".to_string(),
            "password".to_string(),
        ];
        let fields_values = vec![
            IterableType::Uuid(self.id),
            IterableType::DateTime(self.created_at),
            IterableType::DateTime(self.updated_at),
            IterableType::DateTime(self.last_login.unwrap_or_default()),
            IterableType::Bool(self.needs_verify),
            IterableType::Bool(self.is_blocked),
            IterableType::String(self.username.to_string()),
            IterableType::String(self.email.to_string()),
            IterableType::String(self.password.to_string()),
        ];

        (fields_names, fields_values)
    }
}

// endregion: UserAuth

// region: UserAuthForCreate

#[derive(Deserialize)]
pub struct UserAuthForCreate {
    pub username: String,
    pub email: String,
    pub password: String,
}

// TODO: Implement email validation
impl UserAuthForCreate {
    pub fn validate(ua_fc: &UserAuthForCreate) -> Result<()> {
        if ua_fc.username.is_empty() {
            return Err(Error::UsernameNotSet);
        }

        if ua_fc.email.is_empty() {
            return Err(Error::EmailNotSet);
        }

        if ua_fc.password.is_empty() {
            return Err(Error::PasswordNotSet);
        }

        Ok(())
    }

    pub fn hash_password(self) -> Result<Self> {
        let hashed_password = utils::hash_password(self.password)?;

        Ok(Self {
            password: hashed_password,
            ..self
        })
    }
}

// endregion: UserAuthForCreate

// region: UserAuth Backend ModelController

pub struct UserAuthBmc;

impl UserAuthBmc {
    pub async fn create(_model_maanger: &ModelManager, _ua_fc: UserAuthForCreate) -> Result<Uuid> {
        let user_auth = UserAuth::new(_ua_fc)?;

        let res_uuid = crud::create(_model_maanger.db().clone(), "users_auth", user_auth).await?;

        let user_auth_created = UserAuth::from_row(&res_uuid)?;

        Ok(user_auth_created.id)
    }

    pub async fn _get(_model_maanger: &ModelManager, _id: Uuid) -> Result<UserAuth> {
        todo!()
    }

    pub async fn _get_username(
        _model_maanger: &ModelManager,
        _username: String,
    ) -> Result<UserAuth> {
        todo!()
    }

    pub async fn _get_all(_model_maanger: &ModelManager) -> Result<Vec<UserAuth>> {
        todo!()
    }

    pub async fn _update(_model_maanger: &ModelManager, _id: Uuid) -> Result<()> {
        todo!()
    }

    pub async fn _delete(_model_maanger: &ModelManager, _id: Uuid) -> Result<()> {
        todo!()
    }
}

// endregion: UserAuth Backend ModelController
