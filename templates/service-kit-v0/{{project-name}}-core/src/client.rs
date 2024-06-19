use crate::settings::NetworkSettings;
use {{crate_name}}_proto::prelude::*;

/// Make a network request with a `NetworkSettings` configuration against the /health endpoint.
///
pub async fn health(config: NetworkSettings) -> crate::Result<HealthCheckResponse> {
    let uri = format!("http://{}/health", config.address());
    let response = reqwest::get(&uri).await?;

    Ok(response.json::<HealthCheckResponse>().await?)
}

pub struct WebClient;

impl WebClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn health(&self) -> crate::Result<HealthCheckResponse> {
        health(NetworkSettings {
            host: "localhost".to_string(),
            port: 8080,
        })
        .await
    }
}
