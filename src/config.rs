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

    pub SERVER_ADDR: String,

    // Tracing
    pub TRACING_MAX_LEVEL: tracing::Level,

    // gRPC server auth credentials
    pub GRPC_AUTH_KEY: String,

    // Database
    pub DB_URL: String,
    pub DB_MAX_CONNECTIONS: u32,
}

fn default_environment() -> Environment {
    Environment::Development
}

fn default_port() -> String {
    "50051".to_string()
}

fn default_server_url() -> String {
    "127.0.0.1".to_string()
}

fn default_grpc_auth_key() -> String {
    "halls_of_mandos".to_string()
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

        let server_addr = get_server_addr(&environment)?;

        let tracing_max_level = get_tracing_max_level(&environment)?;

        let grpc_auth_key = get_grpc_auth_key(&environment)?;

        let db_url = get_db_url(&environment)?;
        let db_max_connections = get_env("DB_MAX_CONNECTIONS").map_or_else(
            |_| default_db_max_connections(),
            |p| p.parse::<u32>().unwrap(),
        );

        Ok(Config {
            ENVIRONMENT: environment,

            SERVER_ADDR: server_addr,

            TRACING_MAX_LEVEL: tracing_max_level,

            GRPC_AUTH_KEY: grpc_auth_key,

            DB_URL: db_url,
            DB_MAX_CONNECTIONS: db_max_connections,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_server_addr(env: &Environment) -> Result<String> {
    let port = get_env("PORT").map_or_else(
        |_| default_port(),
        |p| p.parse::<String>().unwrap_or(default_port()),
    );

    match env {
        Environment::Test => {
            let server_url = get_env("SERVICE_SERVER_URL_TEST").map_or_else(
                |_| default_server_url(),
                |p| p.parse::<String>().unwrap_or(default_server_url()),
            );

            Ok(server_url + ":" + &port)
        }
        Environment::Development => {
            let server_url = get_env("SERVICE_SERVER_URL_DEV").map_or_else(
                |_| default_server_url(),
                |p| p.parse::<String>().unwrap_or(default_server_url()),
            );

            Ok(server_url + ":" + &port)
        }
        Environment::Production => {
            // in production the server url and the port have to be set
            let port_prod = get_env("PORT_prod")?;
            let server_url = get_env("SERVER_URL")?;

            Ok(server_url + ":" + &port_prod)
        }
    }
}

fn get_tracing_max_level(env: &Environment) -> Result<Level> {
    match env {
        Environment::Test => Ok(tracing::Level::DEBUG),
        Environment::Development => Ok(tracing::Level::DEBUG),
        Environment::Production => Ok(tracing::Level::INFO),
    }
}

fn get_grpc_auth_key(env: &Environment) -> Result<String> {
    match env {
        Environment::Test => {
            let grpc_auth_key = get_env("SERVICE_GRPC_AUTH_KEY_TEST").map_or_else(
                |_| default_grpc_auth_key(),
                |p| p.parse::<String>().unwrap_or(default_grpc_auth_key()),
            );

            Ok(grpc_auth_key)
        }
        Environment::Development => {
            let grpc_auth_key = get_env("SERVICE_GRPC_AUTH_KEY_DEV").map_or_else(
                |_| default_grpc_auth_key(),
                |p| p.parse::<String>().unwrap_or(default_grpc_auth_key()),
            );

            Ok(grpc_auth_key)
        }
        Environment::Production => {
            // in production the server auth key and the value have to be set
            let grpc_auth_key = get_env("SERVICE_GRPC_AUTH_KEY_TEST")?;

            Ok(grpc_auth_key)
        }
    }
}

fn get_db_url(env: &Environment) -> Result<String> {
    match env {
        Environment::Test => {
            let db_user = get_env("SERVICE_DB_USER_TEST")?;
            let db_password = get_env("SERVICE_DB_PASSWORD_TEST")?;
            let db_host = get_env("SERVICE_DB_HOST_TEST")?;
            let db_port = get_env("SERVICE_DB_PORT_TEST")?;
            let db_name = get_env("SERVICE_DB_NAME_TEST")?;

            Ok(format!(
                "postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}"
            ))
        }
        Environment::Development => {
            let db_user = get_env("SERVICE_DB_USER_DEV")?;
            let db_password = get_env("SERVICE_DB_PASSWORD_DEV")?;
            let db_host = get_env("SERVICE_DB_HOST_DEV")?;
            let db_port = get_env("SERVICE_DB_PORT_DEV")?;
            let db_name = get_env("SERVICE_DB_NAME_DEV")?;

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
