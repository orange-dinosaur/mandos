use error::Result;

pub use config::config;

use crate::model::ModelManager;

pub mod config;
pub mod error;
pub mod mandos_auth;
pub mod model;
pub mod server;
pub mod tracing;
pub mod utils;

mod mandos_auth_proto {
    #![allow(non_snake_case)]
    include!("mandos_auth.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("mandos_auth_descriptor");
}

#[tokio::main]
async fn main() -> Result<()> {
    utils::print_app_name("Mandos", 30, 2);

    // Initialize tracing
    tracing::initialize();

    // Initialize ModelManager
    let model_manager = ModelManager::new().await?;

    // start gRPC server
    server::start(model_manager).await?;

    Ok(())
}
