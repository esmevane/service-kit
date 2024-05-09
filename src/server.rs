use std::net::SocketAddr;

use axum::Router;

use crate::settings::NetworkSettings;

pub mod api;
pub mod full;
pub mod web;

pub async fn init(config: NetworkSettings) -> crate::Result<()> {
    let app = Router::new()
        .merge(api::app(config.clone()).await)
        .merge(web::app(config.clone()).await);

    let listener = config.listener().await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
