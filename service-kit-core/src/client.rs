use crate::settings::NetworkSettings;
use wasm_bindgen::prelude::*;

/// Make a network request with a `NetworkSettings` configuration against the /health endpoint.
///
pub async fn health(config: NetworkSettings) -> crate::Result<()> {
    let uri = format!("http://{}/health", config.address());
    let response = reqwest::get(&uri).await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

#[wasm_bindgen]
pub struct WebClient;

impl WebClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn health(&self) -> crate::Result<()> {
        health(NetworkSettings {
            host: "localhost".to_string(),
            port: 8080,
        })
        .await
    }
}
