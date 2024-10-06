use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use reqwest::StatusCode;
use sqlx::SqlitePool;

pub(super) fn route(pool: SqlitePool) -> Router {
    Router::new()
        .route("/sql", get(sql))
        .route("/reset", post(reset))
        .route("/orders", post(insert_orders))
        .route("/orders/total", get(total))
        .route("/orders/popular", get(popular))
        .with_state(pool)
}

async fn sql(State(pool): State<SqlitePool>) -> Json<i32> {
    Json(
        sqlx::query_scalar("SELECT 20231213")
            .fetch_one(&pool)
            .await
            .unwrap(),
    )
}

async fn reset(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let result = sqlx::query(
        "
        DROP TABLE IF EXISTS orders;
        CREATE TABLE orders (
          id INT PRIMARY KEY,
          region_id INT,
          gift_name VARCHAR(50),
          quantity INT
        )
    ",
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[derive(serde::Deserialize)]
struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

async fn insert_orders(
    State(pool): State<SqlitePool>,
    Json(payload): Json<Vec<Order>>,
) -> impl IntoResponse {
    for order in payload {
        sqlx::query("INSERT INTO orders (id, region_id, gift_name, quantity) VALUES (?, ?, ?, ?)")
            .bind(order.id)
            .bind(order.region_id)
            .bind(order.gift_name)
            .bind(order.quantity)
            .execute(&pool)
            .await
            .unwrap();
    }

    StatusCode::OK
}

#[derive(serde::Serialize)]
struct Total {
    total: i32,
}
async fn total(State(pool): State<SqlitePool>) -> Json<Total> {
    let total: i32 = sqlx::query_scalar("SELECT SUM(quantity) FROM orders")
        .fetch_one(&pool)
        .await
        .unwrap();

    Json(Total { total })
}

#[derive(serde::Serialize)]
struct PopularGift {
    popular: Option<String>,
}

async fn popular(State(pool): State<SqlitePool>) -> Json<PopularGift> {
    Json(PopularGift {
        popular: sqlx::query_scalar(
            "SELECT gift_name FROM orders GROUP BY gift_name ORDER BY SUM(quantity) DESC LIMIT 1",
        )
        .fetch_optional(&pool)
        .await
        .unwrap(),
    })
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::days::routes_test;

    #[tokio::test]
    async fn task1() {
        routes_test()
            .await
            .get("/13/sql")
            .await
            .assert_json(&20231213)
    }

    #[tokio::test]
    async fn task2() {
        let server = routes_test().await;
        server.post("/13/reset").await.assert_status_ok();
        server
            .post("/13/orders")
            .json(&json!([
              {"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
              {"id":2,"region_id":2,"gift_name":"Doll","quantity":8},
              {"id":3,"region_id":3,"gift_name":"Action Figure","quantity":12},
              {"id":4,"region_id":4,"gift_name":"Board Game","quantity":10},
              {"id":5,"region_id":2,"gift_name":"Teddy Bear","quantity":6},
              {"id":6,"region_id":3,"gift_name":"Toy Train","quantity":3}
            ]))
            .await
            .assert_status_ok();
        server
            .get("/13/orders/total")
            .await
            .assert_json(&json!({"total": 44}));
    }

    #[tokio::test]
    async fn task3() {
        let server = routes_test().await;
        server.post("/13/reset").await.assert_status_ok();
        server
            .get("/13/orders/popular")
            .await
            .assert_json(&json!({"popular": null}));
        server
            .post("/13/orders")
            .json(&json!([
              {"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
              {"id":2,"region_id":2,"gift_name":"Doll","quantity":8},
              {"id":3,"region_id":3,"gift_name":"Toy Train","quantity":4}
            ]))
            .await
            .assert_status_ok();

        server
            .get("/13/orders/popular")
            .await
            .assert_json(&json!({"popular": "Toy Train"}));
    }
}
