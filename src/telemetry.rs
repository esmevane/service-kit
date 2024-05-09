// use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

static DEFAULT_ENV_FILTER: &str = "info,emblem_web=debug,tower_http=debug,axum::rejection=trace";

pub(crate) fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| DEFAULT_ENV_FILTER.into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

// pub(crate) fn tracing_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
//     TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default())
// }
