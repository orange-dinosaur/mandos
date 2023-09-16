use mandos::{
    error::{Error, Result},
    mandos_auth::RegisterRequest,
    model::{session, user_auth::UserAuth},
    utils,
};
use sqlx::FromRow;
use uuid::Uuid;

use crate::test_utils::{get_grpc_client, start_background_grpc_server};

mod test_utils;

#[tokio::test]
async fn register_works() -> Result<()> {
    let addr = "0.0.0.0:50051".to_string();
    let client_addr = "http://0.0.0.0:50051";

    // Run the server in the background
    let model_manager = start_background_grpc_server(addr).await?;

    // get the grpc client
    let mut client = get_grpc_client(client_addr).await?;

    // clean all databases before running the test
    sqlx::query("delete from users_auth")
        .execute(model_manager.db())
        .await?;
    session::crud::flush_db(model_manager.session_db().clone()).await?;

    let username = "username".to_string();
    let email = "email@email.com".to_string();
    let password = "secret".to_string();

    // region: call register grpc method
    let request = tonic::Request::new(RegisterRequest {
        username: username.clone(),
        email: email.clone(),
        password: password.clone(),
    });

    client
        .register(request)
        .await
        .map_err(|s| Error::Test(s.to_string()))?;
    // endregion: call register grpc method

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
    sqlx::query("delete from users_auth")
        .execute(model_manager.db())
        .await?;
    session::crud::flush_db(model_manager.session_db().clone()).await?;

    Ok(())
}
