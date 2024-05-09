mod errors;
mod settings;
mod terminal;

pub use errors::Errors;

mod telemetry {
    use tracing_subscriber::EnvFilter;
    pub fn init() {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }
}

pub async fn run() -> Result<(), Errors> {
    telemetry::init();

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
