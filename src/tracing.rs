use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::config;

pub fn initialize() {
    let env_filter = EnvFilter::try_new("mandos=trace").unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = FmtSubscriber::builder()
        .with_max_level(config().TRACING_MAX_LEVEL)
        .with_env_filter(env_filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Tracing initialized");
}
