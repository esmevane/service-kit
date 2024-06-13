pub mod network_settings;

mod client_settings;
mod server_settings;
mod service_settings;

use clap::{Parser, ValueEnum};
use config::{Config, ValueKind};
use serde::Deserialize;
use std::path::PathBuf;

/// A CLI application that helps do non-standard AzerothCore db tasks
#[derive(Clone, Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,

    #[clap(flatten)]
    pub global: GlobalOpts,
}

#[derive(Clone, Debug, Parser)]
pub struct GlobalOpts {
    /// If you want to override the program name.
    #[clap(env = "CARGO_PKG_NAME", short, long)]
    pub app_name: String,

    /// The path to the configuration root.
    #[clap(short, long)]
    pub config: Option<String>,

    /// What environment to run the program in.
    #[clap(short, long, default_value = "development")]
    pub environment: Environment,

    /// Enable verbose output.
    #[clap(short = 'v', long = "verbose")]
    pub verbose: bool,
}

#[derive(Clone, Debug, Parser)]
#[clap(rename_all = "kebab-case")]
pub enum Command {
    Debug,
    Tui,
    Server(server_settings::Server),
    Client(client_settings::Client),
    Service(service_settings::Service),
}

#[derive(Clone, Copy, Debug, Default, Deserialize, strum::Display, ValueEnum)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Environment {
    #[default]
    Development,
    Production,
}

impl Into<ValueKind> for Environment {
    fn into(self) -> ValueKind {
        self.to_string().into()
    }
}

impl Cli {
    fn base_config_path(&self) -> String {
        match self.global.config {
            Some(ref config) => config.clone(),
            None => String::new(),
        }
    }

    /// Build an OS agnostic path to the home configuration directory
    /// based on the given config.
    pub fn home_config(&self) -> String {
        let mut path = PathBuf::new();
        path.push(dirs::home_dir().unwrap_or_default());
        path.push(".config");
        path.push(self.global.app_name.to_lowercase());
        path.push("config");

        path.to_string_lossy().into()
    }

    /// Build an OS agnostic path to the root configuration directory
    /// based on the given config, app_name, and environment.
    pub fn env_config(&self) -> String {
        let mut path = PathBuf::new();
        path.push(self.base_config_path());
        path.push(format!(
            "{}.{}",
            self.global.app_name.to_lowercase(),
            self.global.environment.clone()
        ));

        path.to_string_lossy().into()
    }

    /// Build an OS agnostic path to the root configuration directory
    /// based on the given config, app_name.
    pub fn root_config(&self) -> String {
        let mut path = PathBuf::new();
        path.push(self.base_config_path());
        path.push(self.global.app_name.to_lowercase());

        path.to_string_lossy().into()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub db: Option<Database>,
    pub storage: Option<Storage>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Storage {
    pub path: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Database {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: Option<String>,
}

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

                #[cfg(not(target_arch = "wasm32"))]
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

                        server_settings::ServerMode::select()?.exec(context).await?;
                    }
                }
            }
            Command::Client(client_details) => {
                tracing::info!("Client command");

                match client_details.resource {
                    Some(resource) => resource.exec(client_details.settings).await?,
                    None => {
                        tracing::info!("No client resource specified, prompting");

                        client_settings::ClientResource::select()?
                            .exec(client_details.settings)
                            .await?;
                    }
                }
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

                        service_settings::ServiceOperation::select()?
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

impl std::fmt::Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mysql://{}:{}@{}:{}/{}",
            self.user,
            self.password,
            self.host,
            self.port,
            self.database.as_deref().unwrap_or("acore_world"),
        )
    }
}
