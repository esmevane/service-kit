use wasm_bindgen::prelude::*;

/// Make a network request with a `NetworkSettings` configuration against the /health endpoint.
///
pub async fn health(
    config: crate::settings::network_settings::NetworkSettings,
) -> crate::Result<crate::protocol::services::HealthCheckResponse> {
    let uri = format!("http://{}/health", config.address());
    let response = reqwest::get(&uri).await?;
    let body = response
        .json::<crate::protocol::services::HealthCheckResponse>()
        .await?;

    Ok(body)
}

#[wasm_bindgen]
pub struct WebClient {
    config: crate::settings::network_settings::NetworkSettings,
}

impl WebClient {
    pub fn new(host: impl AsRef<str>, port: Option<u16>) -> Self {
        let config = crate::settings::network_settings::NetworkSettings {
            host: host.as_ref().to_string(),
            port: port.unwrap_or(8080),
        };

        Self { config }
    }

    pub async fn health(&self) -> crate::Result<JsValue> {
        println!("Hello from the client!");
        let uri = format!("http://{}/healthyo", self.config.address());
        let response = reqwest::get(&uri).await?;

        let text = response.text().await?;
        let body: serde_json::Value = serde_json::from_str(&text).unwrap();

        // Ok(JsValue::from_serde(&branch_info).unwrap())
        // let result = health(config).await?;

        let response: JsValue =
            serde_wasm_bindgen::to_value(&body).expect("Failed to import response");

        // println!("Hay there pardner - {:?}", response);

        Ok(response)
    }
}
