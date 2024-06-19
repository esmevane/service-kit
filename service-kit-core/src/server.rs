use std::net::SocketAddr;

use axum::Router;

pub mod api;
pub mod full;
pub mod web;

pub mod protocol_service {
    pub struct ProtocolService;

    impl service_kit_proto::prelude::WebService for ProtocolService {
        fn health(
            _: service_kit_proto::prelude::HealthCheck,
        ) -> service_kit_proto::prelude::HealthCheckResponse {
            service_kit_proto::prelude::HealthCheckResponse {
                version: env!("CARGO_PKG_VERSION").to_string(),
            }
        }
    }
}

pub async fn init(context: crate::WebContext) -> crate::Result<()> {
    let app = Router::new()
        .merge(web::router(context.clone()).await)
        .merge(api::router(context.clone()).await)
        .with_state(context.clone());

    let listener = context.listener().await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
