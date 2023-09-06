use tonic::{Response, Status};
use tracing::debug;

use crate::{
    mandos_auth::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse},
    model::{user_auth, ModelManager},
};

pub async fn login(
    login_request: LoginRequest,
    _model_maanger: ModelManager,
) -> Result<Response<LoginResponse>, Status> {
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
    _model_maanger: ModelManager,
) -> Result<Response<RegisterResponse>, Status> {
    debug!("FN: register - Service to register user");

    // check that the fields are not empty
    if register_request.username.is_empty()
        || register_request.email.is_empty()
        || register_request.password.is_empty()
    {
        return Err(Status::invalid_argument("one ore more fields are empty"));
    }

    let user_auth_for_create = user_auth::UserAuthForCreate {
        username: register_request.username,
        email: register_request.email,
        password: register_request.password,
    };

    // create user in the db
    let db_res = user_auth::UserAuthBmc::create(&_model_maanger, user_auth_for_create).await;
    match db_res {
        Ok(id) => {
            debug!("User created with id: {}", id);
        }
        Err(e) => {
            return Err(Status::internal(e.to_string()));
        }
    }

    let res = RegisterResponse { success: true };
    Ok(Response::new(res))
}
