use std::path::PathBuf;
use std::str::FromStr;

use clap::{Parser, ValueEnum};
use config::{Config, ValueKind};
use serde::Deserialize;
use service_manager::ServiceManagerKind;
use strum::{EnumString, VariantNames};

use crate::Error;

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
    Server(Server),
    Client(Client),
    Service(Service),
}

#[derive(Clone, Debug, Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Server {
    /// The mode to run the server in.
    #[clap(subcommand)]
    pub mode: Option<ServerMode>,
    /// The settings for the server.
    #[clap(flatten)]
    pub settings: NetworkSettings,
}

#[derive(Clone, Debug, Parser, Default, EnumString, VariantNames)]
#[clap(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ServerMode {
    /// Run the server in full mode.
    #[default]
    Full,
    /// Run the server in a web mode.
    Web,
    /// Run the server in an api mode.
    Api,
}

impl ServerMode {
    pub fn options() -> &'static [&'static str] {
        Self::VARIANTS
    }

    pub async fn exec(&self, config: NetworkSettings) -> crate::Result<()> {
        Ok(match self {
            ServerMode::Full => crate::server::init(config).await?,
            ServerMode::Web => crate::server::web::init(config).await?,
            ServerMode::Api => crate::server::api::init(config).await?,
        })
    }

    pub fn select() -> crate::Result<Self> {
        let options = Self::options();
        let result = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Select a server mode")
            .default(0)
            .items(options)
            .interact()
            .expect("Unable to select server mode");

        Ok(Self::from_str(options[result])?)
    }
}

#[derive(Clone, Debug, Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Client {
    /// Tell the client what resource to connect to.
    #[clap(subcommand)]
    pub resource: Option<ClientResource>,
    /// The settings for the client.
    #[clap(flatten)]
    pub settings: NetworkSettings,
}

#[derive(Clone, Debug, Parser, EnumString, VariantNames)]
#[clap(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ClientResource {
    /// The health check api.
    Health,
}

impl ClientResource {
    pub fn options() -> &'static [&'static str] {
        Self::VARIANTS
    }

    pub async fn exec(&self, config: NetworkSettings) -> crate::Result<()> {
        Ok(match self {
            ClientResource::Health => crate::client::health(config).await?,
        })
    }

    pub fn select() -> crate::Result<Self> {
        let options = Self::options();
        let result = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Select a client resource")
            .default(0)
            .items(options)
            .interact()
            .expect("Unable to select client resource");

        Ok(Self::from_str(options[result])?)
    }
}

#[derive(Clone, Debug, Parser)]
pub struct NetworkSettings {
    /// The host to connect to.
    #[clap(long, default_value = "localhost")]
    pub host: String,

    /// The port to connect to.
    #[clap(long, default_value = "8080")]
    pub port: u16,
}

impl NetworkSettings {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub async fn listener(&self) -> crate::Result<tokio::net::TcpListener> {
        tokio::net::TcpListener::bind(self.address())
            .await
            .map_err(Error::ListenerInitFailure)
    }
}

#[derive(Clone, Debug, Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Service {
    /// Control the service itself.
    #[clap(subcommand)]
    pub operation: Option<ServiceOperation>,
    #[clap(flatten)]
    pub settings: ServiceSettings,
}

#[derive(Clone, Debug, Parser)]
pub struct ServiceSettings {
    /// The service label to use. Defaults to the app name.
    #[clap(long)]
    pub service_label: Option<String>,
    /// The kind of service manager to use. Defaults to system native.
    #[clap(long, value_enum)]
    pub service_manager: Option<ServiceManagerKind>,
    /// Install system-wide. If not set, attempts to install for the current user.
    #[clap(long)]
    pub system: bool,
}

#[derive(Clone, Debug, Parser, EnumString, VariantNames)]
#[clap(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ServiceOperation {
    Install,
    Uninstall,
    Start,
    Stop,
}

impl ServiceOperation {
    pub fn options() -> &'static [&'static str] {
        Self::VARIANTS
    }

    pub fn select() -> crate::Result<Self> {
        let options = Self::options();
        let result = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Select a service operation")
            .default(0)
            .items(options)
            .interact()
            .expect("Unable to select service operation");

        Ok(Self::from_str(options[result])?)
    }

    pub async fn exec(&self, cli: Cli, settings: ServiceSettings) -> crate::Result<()> {
        let program = std::env::current_exe()?;
        let args: Vec<std::ffi::OsString> = vec![
            "-a".into(),
            cli.global.app_name.clone().into(),
            "server".into(),
            "api".into(),
        ];
        let service = crate::service::Service::init(
            settings
                .service_label
                .clone()
                .unwrap_or_else(|| format!("local.service.{}", cli.global.app_name))
                .parse()?,
        )?;

        match self {
            ServiceOperation::Install => service.install(program, args)?,
            ServiceOperation::Start => service.start()?,
            ServiceOperation::Stop => service.stop()?,
            ServiceOperation::Uninstall => service.uninstall()?,
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum NumberOrString {
    String(String),
    Number(u32),
}

impl std::str::FromStr for NumberOrString {
    type Err = &'static str; // The actual type doesn't matter since we never error, but it must implement `Display`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<u32>()
            .map(NumberOrString::Number)
            .unwrap_or_else(|_| NumberOrString::String(s.to_string())))
    }
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

                crate::tui::init().await?;
            }
            Command::Server(server_details) => {
                tracing::info!("Server command: {:?}", server_details);

                match server_details.mode {
                    Some(mode) => mode.exec(server_details.settings).await?,
                    None => {
                        tracing::info!("No server mode specified, prompting");

                        ServerMode::select()?.exec(server_details.settings).await?;
                    }
                }
            }
            Command::Client(client_details) => {
                tracing::info!("Client command");

                match client_details.resource {
                    Some(resource) => resource.exec(client_details.settings).await?,
                    None => {
                        tracing::info!("No client resource specified, prompting");

                        ClientResource::select()?
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

                        ServiceOperation::select()?
                            .exec(self.cli.clone(), service_details.settings)
                            .await?;
                    }
                }
            }
        }

        Ok(())
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
