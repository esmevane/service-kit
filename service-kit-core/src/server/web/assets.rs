use axum::{http::Uri, response::IntoResponse, routing::get, Router};
use rust_embed::RustEmbed;

use crate::settings::NetworkSettings;

pub async fn router(_config: NetworkSettings) -> Router {
    Router::new()
        .route("/*file", get(embed_handler))
        .route("/", get(index))
        .fallback_service(get(index))
}

#[tracing::instrument(level = "debug")]
async fn index() -> impl IntoResponse {
    tracing::debug!("index");
    embed_handler("/index.html".parse::<Uri>().unwrap()).await
}

#[tracing::instrument(level = "debug")]
async fn embed_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("dist/") {
        path = path.replace("dist/", "");
    }

    StaticFile(path)
}

#[derive(RustEmbed)]
#[folder = "dist/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    #[tracing::instrument(level = "debug", skip(self))]
    fn into_response(self) -> axum::response::Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                tracing::debug!("found asset: {}", path);
                let mime = mime_guess::from_path(path).first_or_octet_stream();

                (
                    [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
                    content.data,
                )
                    .into_response()
            }
            None => {
                tracing::debug!("no asset found for: {}", path);
                (axum::http::StatusCode::NOT_FOUND, "404 Not Found").into_response()
            }
        }
    }
}
