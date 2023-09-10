use std::env;

use tonic::{Request, Status};
use tracing::debug;

use crate::config;

pub fn check_auth(request: Request<()>) -> std::result::Result<Request<()>, Status> {
    debug!("FN: check_auth - Verifying auth token");

    let request_grpc_auth_value = match request.metadata().get(&config().GRPC_AUTH_KEY) {
        Some(v) => {
            // if the value canno be converted to a string, set it to an empty string
            v.to_str().unwrap_or("").to_string()
        }
        None => {
            return Err(Status::unauthenticated("No valid auth token"));
        }
    };

    // get the auth value to validate request
    let grpc_auth_value = match config().ENVIRONMENT {
        config::Environment::Test | config::Environment::Development => env::var("GRPC_AUTH_VALUE")
            .map_err(|_| Status::unauthenticated("No valid auth token"))?,
        config::Environment::Production => env::var("GRPC_AUTH_VALUE")
            .map_err(|_| Status::unauthenticated("No valid auth token"))?,
    };

    // check that that the auth value is correct
    if request_grpc_auth_value != grpc_auth_value {
        return Err(Status::unauthenticated("No valid auth token"));
    }

    Ok(request)
}
