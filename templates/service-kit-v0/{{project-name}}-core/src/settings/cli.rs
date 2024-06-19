use clap::Parser;
use std::path::PathBuf;

use super::{Client, Environment, Server, Service};

/// A CLI application that helps do non-standard AzerothCore db tasks
#[derive(Clone, Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,

    #[clap(flatten)]
    pub global: GlobalOpts,
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
