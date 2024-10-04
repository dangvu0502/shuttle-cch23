use std::collections::HashMap;

use axum::{routing::get, Json, Router};
use axum_extra::extract::CookieJar;
use base64::prelude::*;

pub fn route() -> Router {
    Router::new()
        .route("/decode", get(decode))
        .route("/bake", get(get_cookie))
}

fn decode_cookie<T: serde::de::DeserializeOwned>(encrypt: &str) -> Option<T> {
    BASE64_STANDARD
        .decode(encrypt)
        .map(|x| serde_json::from_slice(&x).unwrap())
        .ok()
}

type Recipe = HashMap<String, usize>;

async fn decode(jar: CookieJar) -> Json<serde_json::Value> {
    decode_cookie(jar.get("recipe").unwrap().value())
        .map(Json)
        .unwrap()
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct BakingInput {
    recipe: Recipe,
    pantry: Recipe,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct BakingResult {
    cookies: usize,
    pantry: Recipe,
}

async fn get_cookie(jar: CookieJar) -> Json<BakingResult> {
    decode_cookie(jar.get("recipe").unwrap().value())
        .map(|x: BakingInput| {
            let mut cookies = usize::MAX;

            for (ingredient, recipe_amount) in x.recipe.iter() {
                if recipe_amount == &0 {
                    continue;
                }
                if let Some(pantry_amount) = x.pantry.get(ingredient) {
                    if pantry_amount < recipe_amount {
                        return Json(BakingResult {
                            cookies: 0,
                            pantry: x.pantry,
                        });
                    }

                    cookies = cookies.min(pantry_amount / recipe_amount);
                }
            }

            let mut updated_pantry = x.pantry;
            for (ingredient, recipe_amount) in x.recipe.iter() {
                if let Some(pantry_amount) = updated_pantry.get_mut(ingredient) {
                    *pantry_amount = pantry_amount.saturating_sub(cookies * recipe_amount);
                }
            }

            Json(BakingResult {
                cookies,
                pantry: updated_pantry,
            })
        })
        .unwrap()
}
