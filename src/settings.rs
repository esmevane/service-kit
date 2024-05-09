use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use config::{Config, ValueKind};
use serde::Deserialize;

/// A CLI application that helps do non-standard AzerothCore db tasks
#[derive(Clone, Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,

    #[clap(flatten)]
    global: GlobalOpts,
}

#[derive(Clone, Debug, Parser)]
#[clap(rename_all = "kebab-case")]
pub enum Command {
    Debug,
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

#[derive(Clone, Debug, Parser)]
struct GlobalOpts {
    /// If you want to override the program name.
    #[clap(env = "CARGO_PKG_NAME", short, long)]
    app_name: String,

    /// The path to the configuration root.
    #[clap(short, long)]
    config: Option<String>,

    /// What environment to run the program in.
    #[clap(short, long, default_value = "development")]
    environment: Environment,

    /// Enable verbose output.
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, strum::Display, ValueEnum)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
enum Environment {
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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub db: Option<Database>,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: Option<String>,
}

#[derive(Debug)]
pub struct Settings {
    pub cli: Cli,
    pub config: Configuration,
}

impl Settings {
    pub fn parse() -> Result<Self, crate::errors::Errors> {
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
