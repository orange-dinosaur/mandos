use error::Result as CustomResult;
use tonic::{transport::Server, Request, Response, Status};
use tracing::{debug, info};

use crate::{
    config, error,
    mandos_auth::{
        mandos_auth_server::{MandosAuth, MandosAuthServer},
        DeleteAccountRequest, DeleteAccountResponse, HealthCheckRequest, HealthCheckResponse,
        LoginRequest, LoginResponse, LogoutRequest, LogoutResponse, RegisterRequest,
        RegisterResponse, UpdatePasswordRequest, UpdatePasswordResponse, ValidateRequest,
        ValidateResponse,
    },
    mandos_auth_proto,
    model::{self, ModelManager},
    server::middleware::check_auth,
};

mod middleware;
mod routes;

pub struct ServiceMandosAuth {
    model_manager: model::ModelManager,
}

impl ServiceMandosAuth {
    pub fn new(model_manager: model::ModelManager) -> Self {
        Self { model_manager }
    }
}

#[tonic::async_trait]
impl MandosAuth for ServiceMandosAuth {
    async fn health_check(
        &self,
        _: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        debug!("FN: health_check - Service to check if server is up");

        let res = HealthCheckResponse { success: true };
        Ok(Response::new(res))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        routes::auth::login(request.into_inner(), self.model_manager.clone()).await
    }

    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        routes::auth::logout(request.into_inner(), self.model_manager.clone()).await
    }

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        routes::auth::register(request.into_inner(), self.model_manager.clone()).await
    }

    async fn validate_session(
        &self,
        request: Request<ValidateRequest>,
    ) -> Result<Response<ValidateResponse>, Status> {
        routes::auth::validate_session(request.into_inner(), self.model_manager.clone()).await
    }

    async fn update_password(
        &self,
        request: Request<UpdatePasswordRequest>,
    ) -> Result<Response<UpdatePasswordResponse>, Status> {
        routes::auth::update_password(request.into_inner(), self.model_manager.clone()).await
    }

    async fn delete_account(
        &self,
        request: Request<DeleteAccountRequest>,
    ) -> Result<Response<DeleteAccountResponse>, Status> {
        routes::auth::delete_account(request.into_inner(), self.model_manager.clone()).await
    }
}

pub async fn start(model_manager: ModelManager) -> CustomResult<()> {
    let addr = config().SERVER_ADDR.parse()?;
    let mandos_auth = ServiceMandosAuth::new(model_manager.clone());

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
