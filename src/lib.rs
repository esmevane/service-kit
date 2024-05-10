mod client;
mod errors;
mod server;
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
        settings::Command::Server(server_details) => {
            tracing::info!("Server command: {:?}", server_details);

            match server_details.mode {
                settings::ServerMode::Full => server::init(server_details.settings).await?,
                settings::ServerMode::Web => server::web::init(server_details.settings).await?,
                settings::ServerMode::Api => server::api::init(server_details.settings).await?,
            }
        }
        settings::Command::Client(client_details) => {
            tracing::info!("Client command");

            match client_details.resource {
                settings::ClientResource::Health => {
                    client::health(client_details.settings).await?;
                }
            }
        }
        settings::Command::Service(service_details) => {
            tracing::info!("Service command: {:?}", service_details);

            // get the whereabouts of the current executable binary
            let program = std::env::current_exe()?;
            let args = vec![
                "-a".into(),
                settings.cli.global.app_name.clone().into(),
                "server".into(),
                "api".into(),
            ];
            let service = service::Service::init(
                service_details
                    .settings
                    .service_label
                    .unwrap_or_else(|| format!("local.service.{}", settings.cli.global.app_name))
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
