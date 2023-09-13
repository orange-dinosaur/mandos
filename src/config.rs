use serde::Deserialize;
use tracing::Level;

use crate::error::{Error, Result};
use std::{env, str::FromStr, sync::OnceLock};

// region: Environment

#[derive(Deserialize, Debug, PartialEq)]
pub enum Environment {
    Production,
    Development,
    Test,
}

impl FromStr for Environment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "production" => Ok(Environment::Production),
            "development" => Ok(Environment::Development),
            "test" => Ok(Environment::Test),
            _ => Err(Error::ConfigInvalidEnvironment(s.to_string())),
        }
    }
}

// endregion: Environment

// The config instance is initialized only once
pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("Failed to load config from environment variables: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub ENVIRONMENT: Environment,

    // Tracing
    pub TRACING_MAX_LEVEL: tracing::Level,

    // gRPC server auth credentials
    pub GRPC_AUTH_KEY: String,
    pub GRPC_AUTH_VALUE: String,

    // Database
    pub DB_URL: String,
    pub DB_MAX_CONNECTIONS: u32,

    // Session Database
    pub SESSION_DB_URL: String,
}

fn default_environment() -> Environment {
    Environment::Development
}

fn default_db_max_connections() -> u32 {
    5
}

impl Config {
    fn load_from_env() -> Result<Config> {
        let environment = get_env("ENVIRONMENT").map_or_else(
            |_| default_environment(),
            |e| e.parse::<Environment>().unwrap_or(default_environment()),
        );

        let tracing_max_level = get_tracing_max_level(&environment)?;

        let grpc_auth_key = get_env("GRPC_AUTH_KEY")?;
        let grpc_auth_value = get_env("GRPC_AUTH_VALUE")?;

        let db_url = get_db_url(&environment)?;
        let db_max_connections = get_env("DB_MAX_CONNECTIONS").map_or_else(
            |_| default_db_max_connections(),
            |p| p.parse::<u32>().unwrap(),
        );

        let session_db_url = get_session_db_url(&environment)?;

        Ok(Config {
            ENVIRONMENT: environment,

            TRACING_MAX_LEVEL: tracing_max_level,

            GRPC_AUTH_KEY: grpc_auth_key,
            GRPC_AUTH_VALUE: grpc_auth_value,

            DB_URL: db_url,
            DB_MAX_CONNECTIONS: db_max_connections,

            SESSION_DB_URL: session_db_url,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_tracing_max_level(env: &Environment) -> Result<Level> {
    match env {
        Environment::Test => Ok(tracing::Level::TRACE),
        Environment::Development => Ok(tracing::Level::TRACE),
        Environment::Production => Ok(tracing::Level::INFO),
    }
}

fn get_db_url(env: &Environment) -> Result<String> {
    match env {
        Environment::Test => {
            let db_user = get_env("DB_USER_TEST")?;
            let db_password = get_env("DB_PASSWORD_TEST")?;
            let db_host = get_env("DB_HOST_TEST")?;
            let db_port = get_env("DB_PORT_TEST")?;
            let db_name = get_env("DB_NAME_TEST")?;

            Ok(format!(
                "postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}"
            ))
        }
        Environment::Development => {
            let db_user = get_env("DB_USER_DEV")?;
            let db_password = get_env("DB_PASSWORD_DEV")?;
            let db_host = get_env("DB_HOST_DEV")?;
            let db_port = get_env("DB_PORT_DEV")?;
            let db_name = get_env("DB_NAME_DEV")?;

            Ok(format!(
                "postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}"
            ))
        }
        Environment::Production => {
            let db_user = get_env("DB_USER")?;
            let db_password = get_env("DB_PASSWORD")?;
            let db_host = get_env("DB_HOST")?;
            let db_port = get_env("DB_PORT")?;
            let db_name = get_env("DB_NAME")?;

            Ok(format!(
                "postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}"
            ))
        }
    }
}

fn get_session_db_url(env: &Environment) -> Result<String> {
    match env {
        Environment::Test => {
            let session_db_user = get_env("SESSION_DB_USER_TEST")?;
            let session_db_password = get_env("SESSION_DB_PASSWORD_TEST")?;
            let session_db_host = get_env("SESSION_DB_HOST_TEST")?;
            let session_db_port = get_env("SESSION_DB_PORT_TEST")?;

            Ok(format!(
                "redis://{session_db_user}:{session_db_password}@{session_db_host}:{session_db_port}"
            ))
        }
        Environment::Development => {
            let session_db_user = get_env("SESSION_DB_USER_DEV")?;
            let session_db_password = get_env("SESSION_DB_PASSWORD_DEV")?;
            let session_db_host = get_env("SESSION_DB_HOST_DEV")?;
            let session_db_port = get_env("SESSION_DB_PORT_DEV")?;

            Ok(format!(
                "redis://{session_db_user}:{session_db_password}@{session_db_host}:{session_db_port}"
            ))
        }
        Environment::Production => {
            let session_db_user = get_env("SESSION_DB_USER")?;
            let session_db_password = get_env("SESSION_DB_PASSWORD")?;
            let session_db_host = get_env("SESSION_DB_HOST")?;
            let session_db_port = get_env("SESSION_DB_PORT")?;

            Ok(format!(
                "redis://{session_db_user}:{session_db_password}@{session_db_host}:{session_db_port}"
            ))
        }
    }
}
