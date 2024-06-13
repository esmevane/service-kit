mod client;
#[cfg(not(target_arch = "wasm32"))]
mod context;
mod errors;
#[cfg(not(target_arch = "wasm32"))]
mod server;
#[cfg(not(target_arch = "wasm32"))]
mod service;
mod settings;
#[cfg(not(target_arch = "wasm32"))]
mod storage;
mod telemetry;

#[cfg(not(target_arch = "wasm32"))]
mod tui;

pub use client::WebClient;
#[cfg(not(target_arch = "wasm32"))]
pub use context::WebContext;
pub use errors::Error;

pub type Result<T> = color_eyre::eyre::Result<T, Error>;

include!(concat!("../protocol/output", "/protocol.rs"));

pub async fn run() -> Result<()> {
    telemetry::init();

    tracing::info!("Starting up");

    settings::Settings::parse()?.exec().await?;

    Ok(())
}
