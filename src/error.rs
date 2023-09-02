use std::net::AddrParseError;

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
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

    // Generic errors
    Service(String),
}

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

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
