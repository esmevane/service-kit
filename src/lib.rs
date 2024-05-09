mod errors;
mod service;
mod settings;
mod telemetry;
mod tui;

pub use errors::Error;

use crate::settings::ServiceOperation;

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
        settings::Command::Service(service_details) => {
            tracing::info!("Service command");

            // get the whereabouts of the current executable binary
            let program = std::env::current_exe()?;
            let args = std::env::args_os().skip(1).collect();
            let service = service::Service::init(
                service_details
                    .settings
                    .service_label
                    .unwrap_or_else(|| format!("com.{}.service", settings.cli.global.app_name))
                    .parse()?,
            )?;

            match service_details.operation {
                ServiceOperation::Install => service.install(program, args)?,
                ServiceOperation::Start => service.start()?,
                ServiceOperation::Stop => service.stop()?,
                ServiceOperation::Uninstall => service.uninstall()?,
            }
        }
    }

    Ok(())
}
