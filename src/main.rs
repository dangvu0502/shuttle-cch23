use axum::{
    extract::{self, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

#[derive(serde::Deserialize)]
struct Pagination {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

async fn names(
    pagination: Query<Pagination>,
    extract::Json(payload): extract::Json<Vec<String>>,
) -> impl IntoResponse {
    let payload_len = payload.len();

    let offset = pagination.offset.unwrap_or(0).min(payload_len);
    let limit = pagination
        .limit
        .unwrap_or(payload_len - offset)
        .min(payload_len - offset);
    let split = pagination.split.unwrap_or(0);

    let result: Vec<String> = payload.into_iter().skip(offset).take(limit).collect();
    if split == 0 {
        return extract::Json(result).into_response();
    }

    let chunks: Vec<Vec<String>> = result
        .chunks(split)
        .map(|chunk| chunk.to_vec()) // Convert each chunk to a Vec<String>
        .collect();

    extract::Json(chunks).into_response()
}

async fn server_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let nest_router = Router::new().route("/", post(names));

    let router = Router::new().nest("/5", nest_router).fallback(server_error);

    Ok(router.into())
}
