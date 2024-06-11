mod client;
mod context;
mod errors;
mod server;
mod service;
mod settings;
mod storage;
mod telemetry;
mod tui;

pub use client::WebClient;
pub use context::WebContext;
pub use errors::Error;

pub type Result<T> = color_eyre::eyre::Result<T, Error>;

include!(concat!("../protocol/output", "/protocol.rs"));

pub async fn run() -> Result<()> {
    telemetry::init();

    // let check = protocol::services::HealthCheck { ping: true };

    // protocol::;
    // protocol::services::WebService::serve().await?;
    // protocol::
    tracing::info!("Starting up");

    settings::Settings::parse()?.exec().await?;

    Ok(())
}
