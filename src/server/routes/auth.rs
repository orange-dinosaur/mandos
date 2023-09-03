use tonic::{Response, Status};
use tracing::debug;

use crate::mandos_auth::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};

pub async fn login(login_request: LoginRequest) -> Result<Response<LoginResponse>, Status> {
    debug!("FN: login - Service to login user");

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

pub async fn register(
    register_request: RegisterRequest,
) -> Result<Response<RegisterResponse>, Status> {
    debug!("FN: register - Service to register user");

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
