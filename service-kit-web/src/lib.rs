use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn health(address: &str) -> Result<(), String> {
    let client = service_kit_core::WebClient::new(address, Some(8080));

    let result = client.health().await.expect("Failed to get health");

    // let uri = format!("http://{}/health", address);
    // let response = reqwest::get(&uri).await.map_err(|e| e.to_string())?;
    // let body = response.text().await.map_err(|e| e.to_string())?;

    println!("{:?}", result);

    Ok(())
}
