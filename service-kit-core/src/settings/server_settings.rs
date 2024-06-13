use clap::Parser;
use std::str::FromStr;
use strum::{EnumString, VariantNames};

#[derive(Clone, Debug, Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Server {
    /// The mode to run the server in.
    #[clap(subcommand)]
    pub mode: Option<ServerMode>,
    /// The settings for the server.
    #[clap(flatten)]
    pub settings: crate::settings::network_settings::NetworkSettings,
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

    pub async fn exec(&self, config: crate::WebContext) -> crate::Result<()> {
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
