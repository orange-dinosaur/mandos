use mandos::{
    error::{Error, Result},
    mandos_auth::HealthCheckRequest,
    utils_tests,
};

#[tokio::test]
async fn health_check_works() -> Result<()> {
    // Initialize env variables
    dotenvy::from_filename_override(".env.test").expect("Failed to load .env.test file");

    let addr = "0.0.0.0:50051".to_string();
    let client_addr: &'static str = "http://0.0.0.0:50051";

    // Run the server in the background
    utils_tests::start_background_grpc_server(addr).await?;

    // get the grpc client
    let mut client = utils_tests::get_grpc_client(client_addr).await?;

    let request = tonic::Request::new(HealthCheckRequest {});

    let response = client.health_check(request).await;
    match response {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Service(e.to_string())),
    }
}
