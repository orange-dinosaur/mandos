use dotenvy::dotenv;
use error::Result;

pub use config::config;

use crate::model::ModelManager;

pub mod config;
pub mod error;
pub mod mandos_auth;
pub mod model;
pub mod server;
pub mod utils;

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
    tracing_subscriber::fmt()
        .with_max_level(config().TRACING_MAX_LEVEL)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .init();

    // Initialize ModelManager
    let model_manager = ModelManager::new().await?;

    // start gRPC server
    server::start(model_manager).await?;

    Ok(())
}
