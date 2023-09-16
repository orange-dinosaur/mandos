use mandos::{
    error::{Error, Result},
    mandos_auth::LoginRequest,
    model::{
        db, session,
        user_auth::{UserAuth, UserAuthForCreate},
    },
    utils_tests,
};
use sqlx::FromRow;
use uuid::Uuid;

/// Test that the login grpc method works
/// Steps:
/// 1. Setup test environment (Env variables, run server in the backgroung, get client)
/// 2. Clean all databases
/// 3. Create a user in the database
/// 4. Call the login grpc method
/// 5. Check that the last login field has been updated
/// 6. Check that the session has been created
/// 7. Clean all databases
#[tokio::test]
async fn login_works() -> Result<()> {
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

    // region: call grpc method

    let request = tonic::Request::new(LoginRequest {
        username: "".to_string(),
        email: email.clone(),
        password: password.clone(),
    });

    let login_res = client
        .login(request)
        .await
        .map_err(|s| Error::Test(s.to_string()))?
        .into_inner();

    // endregion: call grpc method

    // region: tests

    // check that the last_login field was updated
    assert!(user_auth_db.last_login != user_auth.last_login);

    // check that the session_id exists in the database and matches the user_id
    let (_, session_user_id) =
        session::crud::get(model_manager.session_db().clone(), login_res.session_id).await?;
    let session_user_uuid =
        Uuid::parse_str(&session_user_id).map_err(|s| Error::Test(s.to_string()))?;
    assert!(session_user_uuid == user_auth_db.id);

    // endregion: tests

    // clean al databases after running the test
    utils_tests::clean_all_dbs(model_manager.clone()).await?;

    Ok(())
}
