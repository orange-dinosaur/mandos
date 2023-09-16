use mandos::{
    error::{Error, Result},
    mandos_auth::LogoutRequest,
    model::{
        db, session,
        user_auth::{UserAuth, UserAuthForCreate},
    },
    utils_tests,
};
use sqlx::FromRow;

/// Test that the logout grpc method works
/// Steps:
/// 1. Setup test environment (Env variables, run server in the backgroung, get client)
/// 2. Clean all databases
/// 3. Create a user in the database
/// 4. Create a session for the user
/// 5. Call the logout grpc method
/// 6. Check that the session has been deleted
/// 7. Clean all databases
#[tokio::test]
async fn logout_works() -> Result<()> {
    // setup test environment
    let (model_manager, mut client) = utils_tests::setup_test_environment().await?;

    // clean all databases before running the test
    utils_tests::clean_all_dbs(model_manager.clone()).await?;

    // create the user in the database for login
    let username = "username".to_string();
    let email = "email@email.com".to_string();
    let password = "secret".to_string();
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

    let request = tonic::Request::new(LogoutRequest {
        session_id: session_id.clone(),
        user_id: user_auth_db.id.to_string().clone(),
    });

    client
        .logout(request)
        .await
        .map_err(|s| Error::Test(s.to_string()))?;

    // endregion: call grpc method

    // region: tests

    // check that the session has been deleted
    let session_still_exists = session::crud::get(model_manager.session_db().clone(), session_id)
        .await
        .is_ok();
    assert!(!session_still_exists);

    // endregion: tests

    // clean al databases after running the test
    utils_tests::clean_all_dbs(model_manager.clone()).await?;

    Ok(())
}
