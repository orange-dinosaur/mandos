use tonic::{Response, Status};
use tracing::debug;

use crate::{
    mandos_auth::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse},
    model::{
        user_auth::{self, UserAuthBmc},
        ModelManager,
    },
    utils,
};

pub async fn login(
    login_request: LoginRequest,
    model_maanger: ModelManager,
) -> Result<Response<LoginResponse>, Status> {
    debug!("FN: login - Service to login user");

    // check that the fields are not empty
    if (login_request.username.is_empty() && login_request.email.is_empty())
        || login_request.password.is_empty()
    {
        return Err(Status::invalid_argument("one ore more fields are empty"));
    }

    // get user from db
    // if email is not empty, search by email otherwise search by username
    let db_res = if !login_request.email.is_empty() {
        user_auth::UserAuthBmc::get_from_email(&model_maanger, login_request.email)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
    } else {
        user_auth::UserAuthBmc::get_from_username(&model_maanger, login_request.username)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
    };

    // check that the password is correct
    utils::verify_password(login_request.password, db_res.password)
        .map_err(|e| Status::unauthenticated(e.to_string()))?;

    // create session in the db
    let session_id = UserAuthBmc::create_session(
        &model_maanger,
        db_res.id.to_string(),
        (60 * 60 * 24 * 30) as u64,
    )
    .await
    .map_err(|e| Status::internal(e.to_string()))?;

    let res = LoginResponse { session_id };
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

    // TODO: send email to user to confirm email

    let res = RegisterResponse { success: true };
    Ok(Response::new(res))
}
