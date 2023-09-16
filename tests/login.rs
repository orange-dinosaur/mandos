use mandos::{
    error::{Error, Result},
    mandos_auth::LoginRequest,
    model::{
        db, session,
        user_auth::{UserAuth, UserAuthForCreate},
    },
};
use sqlx::FromRow;
use uuid::Uuid;

use crate::test_utils::{get_grpc_client, start_background_grpc_server};

mod test_utils;

#[tokio::test]
async fn login_works() -> Result<()> {
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

    // region: call register grpc method
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
    // endregion: call register grpc method

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
    sqlx::query("delete from users_auth")
        .execute(model_manager.db())
        .await?;
    session::crud::flush_db(model_manager.session_db().clone()).await?;

    Ok(())
}
