mod errors;
mod settings;
mod telemetry;
mod tui;

pub use errors::Error;

pub type Result<T> = color_eyre::eyre::Result<T, Error>;

pub async fn run() -> Result<()> {
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

            tui::init().await?;
        }
    }

    Ok(())
}
