use clap::Parser;

use crate::Error;

#[derive(Clone, Debug, Default, Parser)]
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

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn listener(&self) -> crate::Result<tokio::net::TcpListener> {
        tokio::net::TcpListener::bind(self.address())
            .await
            .map_err(Error::ListenerInitFailure)
    }
}
