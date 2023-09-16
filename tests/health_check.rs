use mandos::{
    error::{Error, Result},
    mandos_auth::HealthCheckRequest,
    utils_tests,
};

/// Test that the health_check grpc method works
/// 1. Setup test environment (Env variables, run server in the backgroung, get client)
/// 4. Call the health_check grpc method
#[tokio::test]
async fn health_check_works() -> Result<()> {
    // region: setup test environment

    // Initialize env variables
    dotenvy::from_filename_override(".env.test").expect("Failed to load .env.test file");

    let addr = "0.0.0.0:50051".to_string();
    let client_addr = "http://0.0.0.0:50051";

    // Run the server in the background
    utils_tests::start_background_grpc_server(addr).await?;

    // get the grpc client
    let mut client = utils_tests::get_grpc_client(client_addr).await?;

    // endregion: setup test environment

    let request = tonic::Request::new(HealthCheckRequest {});

    let response = client.health_check(request).await;
    match response {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Service(e.to_string())),
    }
}
