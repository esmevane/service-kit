use clap::Parser;
use serde_json::Value;
use std::str::FromStr;
use strum::{EnumString, VariantNames};

use super::NetworkSettings;

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

    pub async fn exec(&self, config: NetworkSettings) -> crate::Result<Value> {
        Ok(serde_json::to_value(&match self {
            ClientResource::Health => crate::client::health(config).await?,
        })?)
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
