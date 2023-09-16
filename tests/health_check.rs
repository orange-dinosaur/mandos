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
    // setup test environment
    let (_, mut client) = utils_tests::setup_test_environment().await?;

    let request = tonic::Request::new(HealthCheckRequest {});

    let response = client.health_check(request).await;
    match response {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Service(e.to_string())),
    }
}
