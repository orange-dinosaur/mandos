use mandos::{
    error::{Error, Result},
    mandos_auth::ValidateRequest,
    model::{
        db, session,
        user_auth::{UserAuth, UserAuthForCreate},
    },
    utils_tests,
};
use sqlx::FromRow;

#[tokio::test]
async fn validate_session_works() -> Result<()> {
    // Initialize env variables
    dotenvy::from_filename_override(".env.test").expect("Failed to load .env.test file");

    let addr = "0.0.0.0:50051".to_string();
    let client_addr = "http://0.0.0.0:50051";

    // Run the server in the background
    let model_manager = utils_tests::start_background_grpc_server(addr).await?;

    // get the grpc client
    let mut client = utils_tests::get_grpc_client(client_addr).await?;

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

    // region: call logout grpc method
    let request = tonic::Request::new(ValidateRequest {
        session_id: session_id.clone(),
        user_id: user_auth_db.id.to_string().clone(),
    });

    client
        .validate_session(request)
        .await
        .map_err(|s| Error::Test(s.to_string()))?;
    // endregion: call logout grpc method

    // region: tests

    // check that the session was effectively validated
    let (_, session_res_user_id) =
        session::crud::get(model_manager.session_db().clone(), session_id).await?;
    assert!(session_res_user_id == user_auth_db.id.to_string());

    // endregion: tests

    // clean al databases after running the test
    utils_tests::clean_all_dbs(model_manager.clone()).await?;

    Ok(())
}
