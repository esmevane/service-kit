use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn health(address: &str) -> Result<(), String> {
    let uri = format!("http://{}/health", address);
    let response = reqwest::get(&uri).await.map_err(|e| e.to_string())?;
    let body = response.text().await.map_err(|e| e.to_string())?;

    println!("{}", body);

    Ok(())
}
