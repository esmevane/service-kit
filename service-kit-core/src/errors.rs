#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Config error: {0}")]
    ConfigError(#[from] config::ConfigError),
    #[error("Clap error: {0}")]
    ClapError(#[from] clap::Error),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("SQLx error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("Storage error: {0}")]
    StorageError(#[from] crate::storage::StorageError),
    #[error("Storage not configured, unable to initialize storage collection")]
    StorageNotConfiguredError,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Unable to get next terminal event")]
    TerminalEventError,

    #[error("Error getting receiver lock")]
    ComponentReceiverLockError,
    #[error("Error getting component messages")]
    ComponentReceiverError,
    #[error("Unable to initialize tcp listener: {0}")]
    ListenerInitFailure(std::io::Error),

    #[error("Unable to parse selected option: {0}")]
    CliOptionSelectError(#[from] strum::ParseError),
}
