use axum::{routing::get, Router};
use std::net::SocketAddr;

pub async fn router(_context: crate::WebContext) -> Router<crate::WebContext> {
    Router::new().route("/health", get(|| async { "OK" }))
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
