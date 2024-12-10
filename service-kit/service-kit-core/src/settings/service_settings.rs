use clap::Parser;
use service_manager::ServiceManagerKind;

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
