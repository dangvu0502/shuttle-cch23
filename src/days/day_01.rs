use axum::{extract::Path, routing::get, Json, Router};

pub(super) fn route() -> Router {
    Router::new().route("/*numbers", get(numbers))
}

async fn numbers(Path(numbers): Path<String>) -> Json<i64> {
    numbers
        .trim_end_matches('/')
        .split('/')
        .map(|x| x.parse::<i64>().unwrap())
        .fold(0, std::ops::BitXor::bitxor)
        .pow(3)
        .into()
}