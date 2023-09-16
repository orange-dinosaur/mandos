use mandos::{
    error::{Error, Result},
    mandos_auth::UpdatePasswordRequest,
    model::{
        db, session,
        user_auth::{UserAuth, UserAuthForCreate},
    },
    utils, utils_tests,
};
use sqlx::FromRow;

/// Test that the update_password grpc method works
/// Steps:
/// 1. Setup test environment (Env variables, run server in the backgroung, get client)
/// 2. Clean all databases
/// 3. Create a user in the database
/// 4. Create a session for the user
/// 5. Call the update_password grpc method
/// 6. Check that the updated password is correct
/// 7. Check that the old password does not match the one in the database
/// 8. Clean all databases
#[tokio::test]
async fn update_password_works() -> Result<()> {
    // region: setup test environment

    // Initialize env variables
    dotenvy::from_filename_override(".env.test").expect("Failed to load .env.test file");

    let addr = "0.0.0.0:50051".to_string();
    let client_addr = "http://0.0.0.0:50051";

    // Run the server in the background
    let model_manager = utils_tests::start_background_grpc_server(addr).await?;

    // get the grpc client
    let mut client = utils_tests::get_grpc_client(client_addr).await?;

    // endregion: setup test environment

    // clean all databases before running the test
    utils_tests::clean_all_dbs(model_manager.clone()).await?;

    // create the user in the database for login
    let username = "username".to_string();
    let email = "email@email.com".to_string();
    let password = "secret".to_string();
    let new_password = "new_secret".to_string();
    let user_auth_for_create = UserAuthForCreate {
        username: username.clone(),
        email: email.clone(),
        password: password.clone(),
    };
    let user_auth = UserAuth::new(user_auth_for_create)?;
    let res = db::crud::create(model_manager.db().clone(), "users_auth", user_auth.clone()).await?;
    // newly created user
    let user_auth_db = UserAuth::from_row(&res)?;

    // create a session for the user
    let session_id = session::crud::create(
        model_manager.session_db().clone(),
        user_auth_db.id.to_string().clone(),
        60,
    )
    .await?;

    // region: call grpc method

    let request = tonic::Request::new(UpdatePasswordRequest {
        session_id: session_id.clone(),
        user_id: user_auth_db.id.to_string().clone(),
        old_password: password.clone(),
        new_password: new_password.clone(),
    });

    client
        .update_password(request)
        .await
        .map_err(|s| Error::Test(s.to_string()))?
        .into_inner();

    // endregion: call grpc method

    // get the updated user from the database
    let res_upd =
        db::crud::get_one_by_id(model_manager.db().clone(), "users_auth", user_auth_db.id).await?;
    // newly created user
    let user_auth_db_updated = UserAuth::from_row(&res_upd)?;

    // region: tests

    // check that the new password matches the one in the database
    utils::verify_password(new_password, user_auth_db_updated.password.clone())?;

    // check that the old password does not match the one in the database
    let res_old_pwd = utils::verify_password(password, user_auth_db_updated.password).is_ok();
    assert!(!res_old_pwd);

    // endregion: tests

    // clean al databases after running the test
    utils_tests::clean_all_dbs(model_manager.clone()).await?;

    Ok(())
}
