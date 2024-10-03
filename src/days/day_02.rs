use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub(super) fn route() -> Router {
    Router::new().route("/", get(not_found))
}

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        "I can't found it on https://console.shuttle.rs/cch",
    )
}
