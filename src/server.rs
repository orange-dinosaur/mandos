use std::env;

use tonic::{Request, Response, Status};
use tracing::debug;

use crate::{
    config,
    mandos_auth::{
        mandos_auth_server::MandosAuth, LoginRequest, LoginResponse, RegisterRequest,
        RegisterResponse,
    },
};

#[derive(Debug, Default)]
pub struct ServiceMandosAuth {}

#[tonic::async_trait]
impl MandosAuth for ServiceMandosAuth {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        debug!("FN: login - Service to login user");

        let login_request = request.into_inner();

        // check that the fields are not empty
        if login_request.username.is_empty() || login_request.password.is_empty() {
            return Err(Status::invalid_argument("one ore more fields are empty"));
        }

        // check that the user exists
        if login_request.username != "giulio" {
            return Err(Status::not_found("user not found"));
        }

        // check that the password is correct
        if login_request.password != "secret" {
            return Err(Status::unauthenticated("wrong password"));
        }

        let res = LoginResponse {
            session_id: "session_id".to_string(),
        };
        Ok(Response::new(res))
    }

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        debug!("FN: register - Service to register user");

        let register_request = request.into_inner();

        // check that the fields are not empty
        if register_request.username.is_empty()
            || register_request.email.is_empty()
            || register_request.password.is_empty()
        {
            return Err(Status::invalid_argument("one ore more fields are empty"));
        }

        let res = RegisterResponse { success: true };
        Ok(Response::new(res))
    }
}

pub fn check_auth(request: Request<()>) -> std::result::Result<Request<()>, Status> {
    debug!("FN: check_auth - Verifying auth token");

    let request_grpc_auth_value: String;
    match request.metadata().get(&config().GRPC_AUTH_KEY) {
        Some(v) => {
            // if the value canno be converted to a string, set it to an empty string
            request_grpc_auth_value = v.to_str().unwrap_or_else(|_| "").to_string();
        }
        None => {
            return Err(Status::unauthenticated("No valid auth token"));
        }
    }

    // get the auth value to validate request
    let grpc_auth_value: String;
    match config().ENVIRONMENT {
        config::Environment::Test | config::Environment::Development => {
            grpc_auth_value = env::var("SERVICE_GRPC_AUTH_VALUE")
                .map_err(|_| Status::unauthenticated("No valid auth token"))?;
        }
        config::Environment::Production => {
            grpc_auth_value = env::var("SERVICE_GRPC_AUTH_VALUE")
                .map_err(|_| Status::unauthenticated("No valid auth token"))?;
        }
    }

    // check that that the auth value is correct
    if request_grpc_auth_value != grpc_auth_value {
        return Err(Status::unauthenticated("No valid auth token"));
    }

    Ok(request)
}
