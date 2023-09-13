use std::time::Duration;

use mandos::{
    error::Result,
    mandos_auth::mandos_auth_server::MandosAuthServer,
    model::ModelManager,
    server::{middleware::check_auth, ServiceMandosAuth},
};
use tonic::transport::Server;

pub async fn start_background_grpc_server(addr: String) -> Result<()> {
    dotenvy::from_filename_override(".tests.env").expect("Failed to load .tests.env file");

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

    Ok(())
}
