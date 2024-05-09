mod errors;
mod settings;
mod tui;

pub use errors::Error;

mod telemetry {
    use tracing_subscriber::EnvFilter;
    pub fn init() {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }
}

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
