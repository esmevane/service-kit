mod errors;
mod settings;
mod terminal;

use sqlx::Execute;
use tracing_subscriber::EnvFilter;

pub use errors::Errors;

pub async fn main() -> Result<(), Errors> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting up");

    let settings = settings::Settings::parse()?;

    match settings.cli.command {
        settings::Command::Debug => {
            tracing::info!("Debugging");

            println!("{:#?}", settings);
        }
    }

    Ok(())
}
