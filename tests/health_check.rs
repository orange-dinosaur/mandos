use mandos::{
    error::{Error, Result},
    mandos_auth::HealthCheckRequest,
};

#[path = "tests_utils.rs"]
mod tests_utils;

#[tokio::test]
async fn health_check_works() -> Result<()> {
    let addr = "0.0.0.0:50051".to_string();
    let client_addr: &'static str = "http://0.0.0.0:50051";

    // Run the server in the background
    tests_utils::start_background_grpc_server(addr).await?;

    // get the grpc client
    let mut client = tests_utils::get_grpc_client(client_addr).await?;

    let request = tonic::Request::new(HealthCheckRequest {});

    let response = client.health_check(request).await;
    match response {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Service(e.to_string())),
    }
}
