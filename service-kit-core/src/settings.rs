mod cli;
mod client;
mod configuration;
mod environment;
mod network_settings;
mod server;
mod service;
mod service_settings;

use clap::Parser;
use config::Config;
use std::path::PathBuf;

use client::ClientResource;
use configuration::Configuration;
use server::ServerMode;
use service::ServiceOperation;

pub use cli::{Cli, Command};
pub use client::Client;
pub use environment::Environment;
pub use network_settings::NetworkSettings;
pub use server::Server;
pub use service::Service;
pub use service_settings::ServiceSettings;

#[derive(Clone, Debug)]
pub struct Settings {
    pub cli: Cli,
    pub config: Configuration,
}

impl Settings {
    pub fn parse() -> Result<Self, crate::errors::Error> {
        tracing::info!("Parsing CLI arguments");
        let cli = Cli::parse();

        tracing::info!("Getting configuration");
        let config_builder = Config::builder()
            .add_source(config::File::with_name(&cli.home_config()).required(false))
            .add_source(config::File::with_name(&cli.root_config()).required(false))
            .add_source(config::File::with_name(&cli.env_config()).required(false))
            .add_source(
                config::Environment::with_prefix(&cli.global.app_name.to_uppercase())
                    .separator("_"),
            )
            .build()?;

        let config: Configuration = config_builder.try_deserialize()?;

        Ok(Self { cli, config })
    }

    pub async fn exec(&self) -> crate::Result<()> {
        let cli = self.cli.clone();

        match cli.command {
            Command::Debug => {
                tracing::info!("Debugging");

                println!("{:#?}", self);
            }
            Command::Tui => {
                tracing::info!("Starting TUI");

                crate::tui::init().await?;
            }
            Command::Server(server_details) => {
                tracing::info!("Server command: {:?}", server_details);

                let context =
                    crate::context::WebContext::new(server_details.settings, self.clone()).await?;

                match server_details.mode {
                    Some(mode) => mode.exec(context).await?,
                    None => {
                        tracing::info!("No server mode specified, prompting");

                        ServerMode::select()?.exec(context).await?;
                    }
                }
            }
            Command::Client(client_details) => {
                tracing::info!("Client command");

                let response = match client_details.resource {
                    Some(resource) => resource.exec(client_details.settings).await?,
                    None => {
                        tracing::info!("No client resource specified, prompting");

                        ClientResource::select()?
                            .exec(client_details.settings)
                            .await?
                    }
                };

                tracing::info!("{}", response);
            }
            Command::Service(service_details) => {
                tracing::info!("Service command: {:?}", service_details);

                match service_details.operation {
                    Some(operation) => {
                        operation
                            .exec(self.cli.clone(), service_details.settings)
                            .await?
                    }
                    None => {
                        tracing::info!("No service operation specified, prompting");

                        ServiceOperation::select()?
                            .exec(self.cli.clone(), service_details.settings)
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub fn storage_path(&self) -> PathBuf {
        tracing::info!("Getting storage path");
        let path = match &self.config.storage {
            Some(storage) => storage.path.clone(),
            None => {
                // use directories to get a default data directory in user's config path
                match dirs::config_local_dir() {
                    Some(mut path) => {
                        path.push(self.cli.global.app_name.to_lowercase());
                        path.push("storage.db");
                        path
                    }
                    None => {
                        // otherwise we start in a temp directory
                        std::env::temp_dir().join("storage.db")
                    }
                }
            }
        };

        tracing::info!("Using storage path: {}", path.display());

        // ensure the file and path exist
        if let Some(parent) = path.parent() {
            tracing::info!("Ensuring storage path exists: {:?}", parent);
            std::fs::create_dir_all(parent).expect("Unable to create storage directory");
        }

        if std::fs::metadata(&path).is_err() {
            tracing::info!("Ensuring storage file exists: {:?}", path);
            std::fs::File::create(&path).expect("Unable to create storage file");
        }

        path
    }
}
