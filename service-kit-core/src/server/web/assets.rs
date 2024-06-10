use axum::{extract::State, http::Uri, response::IntoResponse, routing::get, Router};
use rust_embed::RustEmbed;

pub async fn router(_context: crate::WebContext) -> Router<crate::WebContext> {
    Router::new()
        .route("/*file", get(embed_handler))
        .route("/", get(index))
        .fallback(get(index))
}

#[tracing::instrument(level = "debug")]
async fn index(State(app_context): State<crate::WebContext>) -> impl IntoResponse {
    tracing::debug!("index");
    embed_handler(State(app_context), "/index.html".parse::<Uri>().unwrap()).await
}

#[tracing::instrument(level = "debug", skip(app_context))]
async fn embed_handler(
    State(app_context): State<crate::WebContext>,
    uri: Uri,
) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("dist/") {
        path = path.replace("dist/", "");
    }

    match crate::storage::StorageFile::get(&app_context, &uri.to_string()).await {
        Ok(Some(file)) => {
            tracing::debug!("found storage file: {}", uri);
            let mime = mime_guess::from_path(uri.to_string()).first_or_octet_stream();

            (
                [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
                file.contents,
            )
                .into_response()
        }
        _ => {
            tracing::debug!("no storage file found: {}", path);
            StaticFile(path).into_response()
        }
    }
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
