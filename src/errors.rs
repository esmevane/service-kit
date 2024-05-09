#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Config error: {0}")]
    ConfigError(#[from] config::ConfigError),
    #[error("Clap error: {0}")]
    ClapError(#[from] clap::Error),
    #[error("SQLx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}
