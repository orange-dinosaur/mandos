use tonic::{Response, Status};
use tracing::debug;
use uuid::Uuid;

use crate::{
    mandos_auth::{
        LoginRequest, LoginResponse, LogoutRequest, LogoutResponse, RegisterRequest,
        RegisterResponse, UpdatePasswordRequest, UpdatePasswordResponse, ValidateRequest,
        ValidateResponse,
    },
    model::{
        user_auth::{self, UserAuthBmc, UserAuthForUpdate},
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

    // check if the user is blocked or if it stills needs verification
    if db_res.is_blocked || db_res.needs_verify {
        return Err(Status::unauthenticated(
            "user is blocked or needs verification".to_string(),
        ));
    }

    // check that the password is correct
    utils::verify_password(login_request.password, db_res.password)
        .map_err(|e| Status::unauthenticated(e.to_string()))?;

    // generate the struct to update the user (last_login)
    let mut user_auth_for_update = UserAuthForUpdate::new();
    user_auth_for_update.last_login = Some(chrono::Utc::now());

    // update user's last_login in db
    UserAuthBmc::update(&model_maanger, user_auth_for_update, db_res.id)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

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

pub async fn logout(
    logout_request: LogoutRequest,
    model_maanger: ModelManager,
) -> Result<Response<LogoutResponse>, Status> {
    debug!("FN: logout - Service to logout user");

    // check that the fields are not empty
    if logout_request.session_id.is_empty() || logout_request.user_id.is_empty() {
        return Err(Status::invalid_argument("one ore more fields are empty"));
    }

    // get session from db
    let (session_id, user_id) = UserAuthBmc::get_session(&model_maanger, logout_request.session_id)
        .await
        .map_err(|e| Status::unauthenticated(e.to_string()))?;

    // check that the user_id matches
    if user_id != logout_request.user_id {
        return Err(Status::invalid_argument(
            "user_id does not match".to_string(),
        ));
    }

    // delete session from db
    UserAuthBmc::delete_session(&model_maanger, session_id)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

    let res = LogoutResponse { success: true };
    Ok(Response::new(res))
}

pub async fn register(
    register_request: RegisterRequest,
    model_maanger: ModelManager,
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
    let db_res = user_auth::UserAuthBmc::create(&model_maanger, user_auth_for_create).await;
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

pub async fn validate_session(
    validate_request: ValidateRequest,
    model_maanger: ModelManager,
) -> Result<Response<ValidateResponse>, Status> {
    debug!("FN: validate_session - Service to verify if user session is valid");

    // check that the fields are not empty
    if validate_request.session_id.is_empty() || validate_request.user_id.is_empty() {
        return Err(Status::invalid_argument("one ore more fields are empty"));
    }

    // get session from db
    let (_, user_id) = UserAuthBmc::get_session(&model_maanger, validate_request.session_id)
        .await
        .map_err(|e| Status::unauthenticated(e.to_string()))?;

    // check that the user_id matches
    if user_id != validate_request.user_id {
        return Err(Status::invalid_argument(
            "user_id does not match".to_string(),
        ));
    }

    let res = ValidateResponse { success: true };
    Ok(Response::new(res))
}

pub async fn update_password(
    update_password_request: UpdatePasswordRequest,
    model_maanger: ModelManager,
) -> Result<Response<UpdatePasswordResponse>, Status> {
    debug!("FN: update_password - Service to update the password of a logged user");

    // check that the fields are not empty
    if update_password_request.session_id.is_empty()
        || update_password_request.user_id.is_empty()
        || update_password_request.old_password.is_empty()
        || update_password_request.new_password.is_empty()
    {
        return Err(Status::invalid_argument("one ore more fields are empty"));
    }

    // get session from db
    let (_, user_id) = UserAuthBmc::get_session(&model_maanger, update_password_request.session_id)
        .await
        .map_err(|e| Status::unauthenticated(e.to_string()))?;

    // check that the user_id matches
    if user_id != update_password_request.user_id {
        return Err(Status::invalid_argument(
            "user_id does not match".to_string(),
        ));
    }

    // get user from db
    let user_uuid = Uuid::parse_str(update_password_request.user_id.as_str())
        .map_err(|e| Status::invalid_argument(e.to_string()))?;
    let db_res = user_auth::UserAuthBmc::get(&model_maanger, user_uuid)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

    // check if the user is blocked or if it stills needs verification
    if db_res.is_blocked || db_res.needs_verify {
        return Err(Status::unauthenticated(
            "user is blocked or needs verification".to_string(),
        ));
    }

    // check that the old password is correct
    utils::verify_password(update_password_request.old_password, db_res.password)
        .map_err(|e| Status::unauthenticated(e.to_string()))?;

    // generate the struct to update the user
    let mut user_auth_for_update = UserAuthForUpdate::new();
    user_auth_for_update.password = Some(update_password_request.new_password);

    // hash the new password
    let ua_fu = user_auth_for_update
        .hash_password()
        .map_err(|e| Status::internal(e.to_string()))?;

    // update password in db
    UserAuthBmc::update(&model_maanger, ua_fu, user_uuid)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

    let res = UpdatePasswordResponse { success: true };
    Ok(Response::new(res))
}
