use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use reqwest::StatusCode;
use serde_json::json;
use sqlx::{Row, SqlitePool};

pub(super) fn route(pool: SqlitePool) -> Router {
    Router::new()
        .route("/reset", post(reset))
        .route("/orders", post(orders))
        .route("/regions", post(regions))
        .route("/regions/total", get(regions_total))
        .route("/regions/top_list/:number", get(top_list))
        .with_state(pool)
}

async fn reset(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let result = sqlx::query(
        "
    DROP TABLE IF EXISTS regions;
    DROP TABLE IF EXISTS orders;
    
    CREATE TABLE regions (
      id INT PRIMARY KEY,
      name VARCHAR(50)
    );
    
    CREATE TABLE orders (
      id INT PRIMARY KEY,
      region_id INT,
      gift_name VARCHAR(50),
      quantity INT
    );
    ",
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_ok) => StatusCode::OK,
        Err(_err) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[derive(serde::Deserialize)]
struct Region {
    id: i32,
    name: String,
}

#[derive(serde::Deserialize)]
struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

async fn orders(
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

async fn regions(
    State(pool): State<SqlitePool>,
    Json(payload): Json<Vec<Region>>,
) -> impl IntoResponse {
    for region in payload {
        sqlx::query("INSERT INTO regions (id, name) VALUES (?, ?)")
            .bind(region.id)
            .bind(region.name)
            .execute(&pool)
            .await
            .unwrap();
    }

    StatusCode::OK
}

#[derive(sqlx::FromRow, serde::Serialize)]
struct RegionTotal {
    region: String,
    total: i32,
}

async fn regions_total(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, RegionTotal>(
        "
    SELECT
      regions.name AS region,
      SUM(orders.quantity) AS total
    FROM
      regions
    JOIN
      orders ON regions.id = orders.region_id
    GROUP BY
      region 
    ORDER BY
      region ASC;
    ",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(result)
}

#[derive(serde::Serialize, Debug)]
struct RegionTopGifts {
    region: String,
    top_gifts: Vec<String>,
}

async fn top_list(Path(number): Path<i32>, State(pool): State<SqlitePool>) -> impl IntoResponse {
    if number < 1 {
        return Json(
            sqlx::query("SELECT name as region FROM regions")
                .map(|x: sqlx::sqlite::SqliteRow| {
                    let region: String = x.get("region");
                    RegionTopGifts {
                        region,
                        top_gifts: vec![],
                    }
                })
                .fetch_all(&pool)
                .await
                .unwrap(),
        );
    }

    sqlx::query(
        r"
SELECT region, group_concat(gift_name, ', ') as top_gifts FROM (
  SELECT regions.name as region, gift_name, row_number() OVER (PARTITION BY regions.name order by regions.name ASC, SUM(quantity) DESC, gift_name ASC) as row_num
  FROM orders LEFT FULL JOIN regions ON regions.id = orders.region_id
  GROUP BY regions.name, gift_name
  ORDER BY regions.name ASC, SUM(quantity) DESC, gift_name ASC
)
where row_num <= $1
group by region;",
    ).bind(number)
    .map(|row: sqlx::sqlite::SqliteRow| {
        let region: String = row.get("region");
        let top_gifts: Option<String> = row.get("top_gifts");
        tracing::info!(?region, ?top_gifts);
        RegionTopGifts {
            region,
            top_gifts: top_gifts.map(|x| x.split(", ").map(ToString::to_string).collect()).unwrap_or_default(),
        }
    })
    .fetch_all(&pool)
    .await.map(|x| Json(x.into_iter().filter(|y| !y.region.is_empty()).collect::<Vec<RegionTopGifts>>()))
    .unwrap()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::days::routes_test;

    #[tokio::test]
    async fn task1() {
        let server = routes_test().await;

        server.post("/18/reset").await.assert_status_ok();

        server
            .post("/18/regions")
            .json(&json!([
              {"id":1,"name":"North Pole"},
              {"id":2,"name":"Europe"},
              {"id":3,"name":"North America"},
              {"id":4,"name":"South America"},
              {"id":5,"name":"Africa"},
              {"id":6,"name":"Asia"},
              {"id":7,"name":"Oceania"}
            ]))
            .await
            .assert_status_ok();

        server
            .post("/18/orders")
            .json(&json!([
              {"id":1,"region_id":2,"gift_name":"Board Game","quantity":5},
              {"id":2,"region_id":2,"gift_name":"Origami Set","quantity":8},
              {"id":3,"region_id":3,"gift_name":"Action Figure","quantity":12},
              {"id":4,"region_id":4,"gift_name":"Teddy Bear","quantity":10},
              {"id":5,"region_id":2,"gift_name":"Yarn Ball","quantity":6},
              {"id":6,"region_id":3,"gift_name":"Art Set","quantity":3},
              {"id":7,"region_id":5,"gift_name":"Robot Lego Kit","quantity":5},
              {"id":8,"region_id":6,"gift_name":"Drone","quantity":9}
            ]))
            .await
            .assert_status_ok();

        server.get("/18/regions/total").await.assert_json(&json!(
        [
          {"region":"Africa","total":5},
          {"region":"Asia","total":9},
          {"region":"Europe","total":19},
          {"region":"North America","total":15},
          {"region":"South America","total":10}
        ]));
    }

    #[tokio::test]
    async fn task2() {
        let server = routes_test().await;

        server.post("/18/reset").await.assert_status_ok();

        server
            .post("/18/regions")
            .json(&json!([
              {"id":1,"name":"North Pole"},
              {"id":2,"name":"South Pole"},
              {"id":3,"name":"Kiribati"},
              {"id":4,"name":"Baker Island"}
            ]))
            .await
            .assert_status_ok();

        server
            .post("/18/orders")
            .json(&json!([
              {"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
              {"id":2,"region_id":2,"gift_name":"Toy Train","quantity":3},
              {"id":3,"region_id":2,"gift_name":"Doll","quantity":8},
              {"id":4,"region_id":3,"gift_name":"Toy Train","quantity":3},
              {"id":5,"region_id":2,"gift_name":"Teddy Bear","quantity":6},
              {"id":6,"region_id":3,"gift_name":"Action Figure","quantity":12},
              {"id":7,"region_id":4,"gift_name":"Board Game","quantity":10},
              {"id":8,"region_id":3,"gift_name":"Teddy Bear","quantity":1},
              {"id":9,"region_id":3,"gift_name":"Teddy Bear","quantity":2}
            ]))
            .await
            .assert_status_ok();

        let t = server.get("/18/regions/top_list/2").await;
        t.assert_status_ok();
        t.assert_json(&json!([
          {"region":"Baker Island","top_gifts":["Board Game"]},
          {"region":"Kiribati","top_gifts":["Action Figure","Teddy Bear"]},
          {"region":"North Pole","top_gifts":[]},
          {"region":"South Pole","top_gifts":["Doll","Toy Train"]}
        ]));
    }
}
