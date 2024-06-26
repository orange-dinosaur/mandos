use std::vec;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    utils,
};

use super::iterable::{Iterable, IterableType};

pub mod model_controller;

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
        if self.password.is_empty() {
            return Err(Error::PasswordNotSet);
        }

        let hashed_password = utils::hash_password(self.password)?;

        Ok(Self {
            password: hashed_password,
            ..self
        })
    }
}

// endregion: UserAuthForCreate

// region: UserAuthForUpdate

#[derive(Deserialize)]
pub struct UserAuthForUpdate {
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub needs_verify: Option<bool>,
    pub is_blocked: Option<bool>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl Default for UserAuthForUpdate {
    fn default() -> Self {
        Self {
            updated_at: chrono::Utc::now(),
            last_login: None,
            needs_verify: None,
            is_blocked: None,
            username: None,
            email: None,
            password: None,
        }
    }
}

impl UserAuthForUpdate {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn hash_password(self) -> Result<Self> {
        let pwd = self.password.ok_or(Error::PasswordNotSet)?;

        if pwd.is_empty() {
            return Err(Error::PasswordNotSet);
        }

        let hashed_password = utils::hash_password(pwd)?;

        Ok(Self {
            password: Some(hashed_password),
            ..self
        })
    }
}

impl Iterable for UserAuthForUpdate {
    fn get_fields(&self) -> (Vec<String>, Vec<IterableType>) {
        let mut fields_names = Vec::new();
        let mut fields_values = Vec::new();

        fields_names.push("updated_at".to_string());
        fields_values.push(IterableType::DateTime(self.updated_at));

        if self.last_login.is_some() {
            fields_names.push("last_login".to_string());
            fields_values.push(IterableType::DateTime(self.last_login.unwrap()));
        }

        if self.needs_verify.is_some() {
            fields_names.push("needs_verify".to_string());
            fields_values.push(IterableType::Bool(self.needs_verify.unwrap()));
        }

        if self.is_blocked.is_some() {
            fields_names.push("is_blocked".to_string());
            fields_values.push(IterableType::Bool(self.is_blocked.unwrap()));
        }

        if self.username.is_some() {
            fields_names.push("username".to_string());
            fields_values.push(IterableType::String(self.username.clone().unwrap()));
        }

        if self.email.is_some() {
            fields_names.push("email".to_string());
            fields_values.push(IterableType::String(self.email.clone().unwrap()));
        }

        if self.password.is_some() {
            fields_names.push("password".to_string());
            fields_values.push(IterableType::String(self.password.clone().unwrap()));
        }

        (fields_names, fields_values)
    }
}

// endregion: UserAuthForUpdate
