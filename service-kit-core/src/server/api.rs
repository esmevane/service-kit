use axum::{routing::get, Json, Router};
use std::net::SocketAddr;

use crate::protocol::services::WebService;

pub async fn router(_context: crate::WebContext) -> Router<crate::WebContext> {
    Router::new().route(
        "/health",
        get(|| {
            let response = super::protocol_service::ProtocolService::health(
                crate::protocol::services::HealthCheck { ping: true },
            );

            async move { Json(response) }
        }),
    )
}

pub async fn init(context: crate::WebContext) -> crate::Result<()> {
    let app = router(context.clone()).await.with_state(context.clone());
    let listener = context.listener().await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
