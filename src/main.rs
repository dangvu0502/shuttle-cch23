use std::{num::ParseIntError, result};

use axum::{extract::{OriginalUri, Path}, http::StatusCode, response::IntoResponse, routing::get, Router};

async fn cube_the_bits(uri: OriginalUri) -> impl IntoResponse {
    let path = uri.0.path().trim_start_matches("/");
    let segments: Vec<&str> = path.split("/").collect();
    let parsed_segments = segments
        .iter()
        .map(|s| s.parse::<i128>())
        .collect::<Result<Vec<i128>, ParseIntError>>();

    match parsed_segments {
        Ok(numbers) => {
            let result = numbers.iter().fold(1, |acc,  &x| acc ^ x);
            format!("[{}] -> {}", path, result);
            (StatusCode::OK, result.pow(3).to_string()).into_response()
        }
        Err(_) => {
            format!("Invalid path segments. All segments must be valid integers. {}", path).into_response()
        }
    }
    
}

async fn server_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/*path", get(cube_the_bits))
        .fallback(server_error);

    Ok(router.into())
}
