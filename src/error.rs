use std::net::AddrParseError;

use argon2::password_hash;
use deadpool_redis::{CreatePoolError, PoolError};
use redis::RedisError;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use sqlx::migrate::MigrateError;
use strum_macros::AsRefStr;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // Tonic errors
    TonicTransport(#[serde_as(as = "DisplayFromStr")] tonic::transport::Error),

    // Config errors
    ConfigMissingEnv(&'static str),
    ConfigInvalidEnvironment(String),

    // SQLx errors
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
    SqlxMigrate(#[serde_as(as = "DisplayFromStr")] MigrateError),
    SqlxEntityNotFound { entity: &'static str, id: String },

    // Redis errors
    Redis(#[serde_as(as = "DisplayFromStr")] RedisError),
    RedisCreatePool(#[serde_as(as = "DisplayFromStr")] CreatePoolError),
    RedisPool(#[serde_as(as = "DisplayFromStr")] PoolError),

    // Argon2 errors
    Argon2Error(#[serde_as(as = "DisplayFromStr")] argon2::Error),
    Argon2ErrorPasswordHash(#[serde_as(as = "DisplayFromStr")] password_hash::Error),

    // UserAuth errors
    UsernameNotSet,
    EmailNotSet,
    PasswordNotSet,

    // Generic errors
    Service(String),
}

// region: impl From

impl From<AddrParseError> for Error {
    fn from(e: AddrParseError) -> Self {
        Self::Service(e.to_string())
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(e: tonic::transport::Error) -> Self {
        Self::TonicTransport(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(e)
    }
}

impl From<MigrateError> for Error {
    fn from(e: MigrateError) -> Self {
        Self::SqlxMigrate(e)
    }
}

impl From<RedisError> for Error {
    fn from(e: RedisError) -> Self {
        Self::Redis(e)
    }
}

impl From<CreatePoolError> for Error {
    fn from(e: CreatePoolError) -> Self {
        Self::RedisCreatePool(e)
    }
}

impl From<PoolError> for Error {
    fn from(e: PoolError) -> Self {
        Self::RedisPool(e)
    }
}

impl From<argon2::Error> for Error {
    fn from(e: argon2::Error) -> Self {
        Self::Argon2Error(e)
    }
}

impl From<password_hash::Error> for Error {
    fn from(e: password_hash::Error) -> Self {
        Self::Argon2ErrorPasswordHash(e)
    }
}

// endregion: impl From

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
