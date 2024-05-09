mod errors;
mod settings;
mod telemetry;
mod tui;

use color_eyre::eyre::Result;

pub use errors::Error;

pub async fn run() -> Result<(), Error> {
    telemetry::init();

    tracing::info!("Starting up");

    let settings = settings::Settings::parse()?;

    match settings.cli.command {
        settings::Command::Debug => {
            tracing::info!("Debugging");

            println!("{:#?}", settings);
        }
        settings::Command::Tui => {
            tracing::info!("Starting TUI");

            tui::init().await.unwrap();
        }
    }

    Ok(())
}
