mod assets;

use axum::Router;
use std::net::SocketAddr;

use crate::settings::NetworkSettings;

pub async fn app(config: NetworkSettings) -> Router {
    Router::new().merge(assets::router(config.clone()).await)
}

pub async fn init(config: NetworkSettings) -> crate::Result<()> {
    let app = app(config.clone()).await;
    let listener = config.listener().await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
