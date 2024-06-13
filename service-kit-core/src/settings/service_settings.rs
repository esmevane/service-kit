use clap::Parser;
use service_manager::ServiceManagerKind;
use std::str::FromStr;
use strum::{EnumString, VariantNames};

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

    pub async fn exec(
        &self,
        cli: crate::settings::Cli,
        settings: ServiceSettings,
    ) -> crate::Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
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
