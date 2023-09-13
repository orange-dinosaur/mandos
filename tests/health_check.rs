use mandos::{
    config::config,
    error::{Error, Result},
    mandos_auth::{mandos_auth_client::MandosAuthClient, HealthCheckRequest},
};

use tonic::{metadata::MetadataValue, transport::Channel, Request};

use crate::test_utils::start_background_grpc_server;

mod test_utils;

#[tokio::test]
async fn health_check_works() -> Result<()> {
    let addr = "0.0.0.0:50051".to_string();
    let client_addr = "http://0.0.0.0:50051";

    // Run the server in the background
    start_background_grpc_server(addr).await?;

    // connect to the server and run the test
    let channel = Channel::from_static(client_addr).connect().await?;

    let grpc_auth_key = config().GRPC_AUTH_KEY.as_str();
    let grpc_auth_value: MetadataValue<_> = config().GRPC_AUTH_VALUE.as_str().parse().unwrap();

    let mut client = MandosAuthClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut()
            .insert(grpc_auth_key, grpc_auth_value.clone());
        Ok(req)
    });

    let request = tonic::Request::new(HealthCheckRequest {});

    let response = client.health_check(request).await;
    match response {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Service(e.to_string())),
    }
}
