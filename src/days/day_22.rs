use std::collections::HashMap;

use axum::{body, response::IntoResponse, routing::post, Router};

pub(super) fn route() -> Router {
    Router::new()
    .route("/integers", post(integers))
    .route("/rocket", post(rocket))
}

async fn integers(body: String) -> impl IntoResponse {

    let numbers = body.lines().filter_map(|line| line.trim().parse::<u64>().ok()).collect::<Vec<u64>>();
    let mut result = 0;
    for num in numbers {
        result ^= num;
    }

    "🎁".repeat(result as usize)
}

async fn rocket(body: String) -> impl IntoResponse {
    let mut lines = body.lines();
    let number_of_stars = lines.next().map(|x| x.parse().unwrap()).unwrap();
    let stars: Vec<Vec<i32>> = lines
        .by_ref()
        .take(number_of_stars)
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let number_of_portals = lines.next().unwrap().parse().unwrap();
    let mut portal_paths = HashMap::new();
    for p in lines.take(number_of_portals).map(|x| {
        x.split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
    }) {
        portal_paths.entry(p[0]).or_insert(Vec::new()).push(p);
    }

    todo!()
}


#[cfg(test)]
mod test {
    use crate::days::routes_test;

    #[tokio::test]
    async fn task1() {
        let server = routes_test().await;

        server.post("/22/integers")
        .text("888
        77
        888
        22
        77").await.assert_text("🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁");
    }
}