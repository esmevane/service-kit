use axum::{routing::get, Router};
use std::net::SocketAddr;

use crate::settings::NetworkSettings;

pub async fn router(_config: NetworkSettings) -> Router {
    Router::new().route("/health", get(|| async { "OK" }))
}

pub async fn init(config: NetworkSettings) -> crate::Result<()> {
    let app = router(config.clone()).await;
    let listener = config.listener().await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
