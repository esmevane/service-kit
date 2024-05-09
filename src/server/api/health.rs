use axum::{routing::get, Router};

use crate::settings::NetworkSettings;

pub async fn router(_config: NetworkSettings) -> Router {
    Router::new().route("/health", get(|| async { "OK" }))
}
