use {{crate_name}}_proto::protocol::services::*;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct Json<T>(pub T);

impl<T> From<Json<T>> for JsValue
where
    T: serde::Serialize,
{
    fn from(json: Json<T>) -> JsValue {
        serde_wasm_bindgen::to_value(&json.0).unwrap()
    }
}

#[wasm_bindgen]
pub async fn health(address: &str) -> Result<JsValue, JsValue> {
    let uri = format!("http://{}/health", address);
    let response = reqwest::get(&uri).await.map_err(|e| e.to_string())?;

    Ok(Json::<HealthCheckResponse>(
        response
            .json()
            .await
            .expect("Failed to parse response body"),
    )
    .into())
}
