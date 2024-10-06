use axum::{
    extract::Path,
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};

pub(super) fn route() -> Router {
    Router::new()
        .route("/weight/:id", get(get_weight))
        .route("/drop/:id", get(get_momentum))
}

const API: &'static str = "https://pokeapi.co/api/v2/pokemon";
const G: f64 = 9.825;

async fn get_pokemon_stat(id: u32) -> Value {
    let url = format!("{}/{}", API, id);

    match reqwest::get(&url).await {
        Ok(response) if response.status().is_success() => {
            response.json::<Value>().await.unwrap_or_else(|_| json!({}))
        }
        _ => json!({}),
    }
}
async fn get_weight(Path(id): Path<u32>) -> Json<f64> {
    get_pokemon_stat(id)
        .await
        .get("weight")
        .map(|weight| Json(weight.as_f64().unwrap_or(0.0) / 10.0))
        .unwrap()
}

async fn get_momentum(Path(id): Path<u32>) -> Json<f64> {
    get_pokemon_stat(id)
        .await
        .get("weight")
        .map(|weight| Json(weight.as_f64().unwrap_or(0.0) / 10.0 * (2.0 * G * 10.0).sqrt()))
        .unwrap()
}
