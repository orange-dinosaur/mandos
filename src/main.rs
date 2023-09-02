use dotenv::dotenv;
use error::Result;
use mandos_auth::mandos_auth_server::MandosAuthServer;
use server::ServiceMandosAuth;
use tonic::transport::Server;
use tracing::info;

pub use config::config;

use crate::server::check_auth;

pub mod config;
pub mod error;
pub mod mandos_auth;
pub mod server;

mod mandos_auth_proto {
    #![allow(non_snake_case)]
    include!("mandos_auth.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("mandos_auth_descriptor");
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n##########################");
    println!("##        MANDOS        ##");
    println!("##########################\n");

    // Load environment variables
    dotenv().ok();

    // Setup tracing
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    /* tracing_subscriber::fmt()
    .with_target(false)
    .with_env_filter(EnvFilter::from_default_env())
    .init(); */

    // region: setup gRPC server
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
    // endregion: setup gRPC server

    Ok(())
}
