use std::time::Duration;

use crate::{
    config::config,
    error::Result,
    mandos_auth::{mandos_auth_client::MandosAuthClient, mandos_auth_server::MandosAuthServer},
    model::{session, ModelManager},
    server::{middleware::check_auth, ServiceMandosAuth},
};
use tonic::{
    metadata::MetadataValue,
    service::interceptor::InterceptedService,
    transport::{Channel, Server},
    Request, Status,
};

pub async fn clean_all_dbs(model_manager: ModelManager) -> Result<()> {
    sqlx::query("delete from users_auth")
        .execute(model_manager.db())
        .await?;
    session::crud::flush_db(model_manager.session_db().clone()).await?;

    Ok(())
}

pub async fn setup_test_environment() -> Result<(
    ModelManager,
    MandosAuthClient<
        InterceptedService<
            tonic::transport::Channel,
            impl Fn(Request<()>) -> core::result::Result<Request<()>, Status>,
        >,
    >,
)> {
    // Initialize env variables
    dotenvy::from_filename_override(".env.test").expect("Failed to load .env.test file");

    let addr = "0.0.0.0:50051".to_string();
    let client_addr = "http://0.0.0.0:50051";

    // Run the server in the background
    let model_manager = start_background_grpc_server(addr).await?;

    // get the grpc client
    let client = get_grpc_client(client_addr).await?;

    Ok((model_manager, client))
}

async fn start_background_grpc_server(addr: String) -> Result<ModelManager> {
    // Initialize ModelManager
    let model_manager = ModelManager::new().await?;

    let addr = addr.parse()?;
    let mandos_auth = ServiceMandosAuth::new(model_manager.clone());

    tokio::spawn(async move {
        let server = Server::builder()
            .add_service(MandosAuthServer::with_interceptor(mandos_auth, check_auth))
            .serve(addr)
            .await;
        if let Err(e) = server {
            e.to_string();
        }
    });

    // Wait for the server to be ready (optional)
    tokio::time::sleep(Duration::from_secs(2)).await;

    Ok(model_manager)
}

async fn get_grpc_client(
    client_addr: &'static str,
) -> Result<
    MandosAuthClient<
        InterceptedService<
            tonic::transport::Channel,
            impl Fn(Request<()>) -> core::result::Result<Request<()>, Status>,
        >,
    >,
> {
    // connect to the server and run the test
    let channel = Channel::from_static(client_addr).connect().await?;

    let grpc_auth_key = config().GRPC_AUTH_KEY.as_str();
    let grpc_auth_value: MetadataValue<_> = config().GRPC_AUTH_VALUE.as_str().parse().unwrap();

    let client = MandosAuthClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut()
            .insert(grpc_auth_key, grpc_auth_value.clone());
        Ok(req)
    });

    Ok(client)
}
