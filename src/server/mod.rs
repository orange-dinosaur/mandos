use error::Result as CustomResult;
use tonic::{transport::Server, Request, Response, Status};
use tracing::info;

use crate::{
    config, error,
    mandos_auth::{
        mandos_auth_server::{MandosAuth, MandosAuthServer},
        LoginRequest, LoginResponse, RegisterRequest, RegisterResponse,
    },
    mandos_auth_proto,
    server::middleware::check_auth,
};

mod middleware;
mod routes;

#[derive(Debug, Default)]
pub struct ServiceMandosAuth {}

#[tonic::async_trait]
impl MandosAuth for ServiceMandosAuth {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        routes::auth::login(request.into_inner()).await
    }

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        routes::auth::register(request.into_inner()).await
    }
}

pub async fn start() -> CustomResult<()> {
    let addr = config().SERVER_ADDR.parse()?;
    let mandos_auth = ServiceMandosAuth::default();

    info!("Starting gRPC server on {}", addr);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(mandos_auth_proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(MandosAuthServer::with_interceptor(mandos_auth, check_auth))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
