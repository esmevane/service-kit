mod assets;

use axum::Router;
use std::net::SocketAddr;

pub async fn router(context: crate::WebContext) -> Router<crate::WebContext> {
    Router::new().merge(assets::router(context.clone()).await)
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
