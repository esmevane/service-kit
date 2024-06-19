use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

static DEFAULT_ENV_FILTER: &str =
    "info,{{crate_name}}=debug,tower_http=debug,axum::rejection=trace";

pub fn init() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| DEFAULT_ENV_FILTER.into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
