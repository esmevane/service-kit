mod client;
mod errors;
mod server;
mod service;
mod settings;
mod telemetry;
mod tui;

pub use errors::Error;

pub type Result<T> = color_eyre::eyre::Result<T, Error>;

pub async fn run() -> Result<()> {
    telemetry::init();

    tracing::info!("Starting up");

    settings::Settings::parse()?.exec().await?;

    Ok(())
}
