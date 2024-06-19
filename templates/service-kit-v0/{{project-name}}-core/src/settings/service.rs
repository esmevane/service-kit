use clap::Parser;
use std::str::FromStr;
use strum::{EnumString, VariantNames};

use super::{Cli, ServiceSettings};

#[derive(Clone, Debug, Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Service {
    /// Control the service itself.
    #[clap(subcommand)]
    pub operation: Option<ServiceOperation>,
    #[clap(flatten)]
    pub settings: ServiceSettings,
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
