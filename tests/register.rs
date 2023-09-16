use mandos::{
    error::{Error, Result},
    mandos_auth::RegisterRequest,
    model::user_auth::UserAuth,
    utils, utils_tests,
};
use sqlx::FromRow;
use uuid::Uuid;

/// Test that the register grpc method works
/// Steps:
/// 1. Setup test environment (Env variables, run server in the backgroung, get client)
/// 2. Clean all databases
/// 3. Define user to be registered
/// 4. Call the register grpc method
/// 5. Check that the user has been created
/// 6. Check that the user password is correct
/// 7. Clean all databases
#[tokio::test]
async fn register_works() -> Result<()> {
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

    let username = "username".to_string();
    let email = "email@email.com".to_string();
    let password = "secret".to_string();

    // region: call grpc method

    let request = tonic::Request::new(RegisterRequest {
        username: username.clone(),
        email: email.clone(),
        password: password.clone(),
    });

    client
        .register(request)
        .await
        .map_err(|s| Error::Test(s.to_string()))?;

    // endregion: call grpc method

    // get the user from the database
    let row = sqlx::query("select * from users_auth where username = $1 and email = $2")
        .bind(username.clone())
        .bind(email.clone())
        .fetch_one(model_manager.db())
        .await?;

    let user_auth = UserAuth::from_row(&row)?;

    // region: tests

    assert!(
        user_auth.id != Uuid::nil() && user_auth.username == username && user_auth.email == email
    );

    // check that the passwords match
    utils::verify_password(password, user_auth.password)?;

    // endregion: tests

    // clean all databases after running the test
    utils_tests::clean_all_dbs(model_manager.clone()).await?;

    Ok(())
}
