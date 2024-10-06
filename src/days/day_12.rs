use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{Datelike, TimeZone, Utc, Weekday};
use reqwest::StatusCode;
use ulid::Ulid;
use uuid::Uuid;

type SharedState = Arc<Mutex<HashMap<String, Instant>>>;

pub(super) fn route() -> Router {
    let shared_state = Arc::new(Mutex::new(HashMap::<String, Instant>::new()));

    Router::new()
        .route("/save/:string", post(save_string))
        .route("/load/:string", get(get_string))
        .route("/ulids", post(convert_ulids_to_uuids))
        .route("/ulids/:weekday", post(process_ulids))
        .with_state(shared_state)
}

async fn save_string(
    Path(string): Path<String>,
    State(app_state): State<SharedState>,
) -> impl IntoResponse {
    app_state.lock().unwrap().insert(string, Instant::now());
    StatusCode::OK
}

async fn get_string(Path(string): Path<String>, State(app_state): State<SharedState>) -> Json<u64> {
    app_state
        .lock()
        .unwrap()
        .get(&string)
        .map_or(Json(0), |value| Json(value.elapsed().as_secs()))
}

async fn convert_ulids_to_uuids(Json(payload): Json<Vec<String>>) -> Json<Vec<String>> {
    Json(
        payload
            .iter()
            .rev()
            .map(|ulid| Uuid::from_u128(Ulid::from_string(&ulid).unwrap().0).to_string())
            .collect::<Vec<String>>(),
    )
}

#[derive(serde::Serialize, Default)]
struct Task3Output {
    #[serde(rename = "christmas eve")]
    christmas_eve: u64,
    weekday: u64,
    #[serde(rename = "in the future")]
    in_the_future: u64,
    #[serde(rename = "LSB is 1")]
    lsb_is_1: u64,
}
async fn process_ulids(
    Path(weekday): Path<u8>,
    Json(payload): Json<Vec<String>>,
) -> Json<Task3Output> {
    let now = Utc::now();
    let weekday = Weekday::try_from(weekday).unwrap();
    Json(
        payload
            .iter()
            .fold(Task3Output::default(), |mut acc, ulid_str| {
                let ulid = ulid_str.parse::<Ulid>().unwrap();
                let time_stamp = ulid.timestamp_ms() as i64;
                let date_time = Utc.timestamp_millis_opt(time_stamp).unwrap();

                if date_time > now {
                    acc.in_the_future += 1;
                }
                if date_time.weekday() == weekday {
                    acc.weekday += 1;
                }
                if date_time.month() == 12 && date_time.day() == 24 {
                    acc.christmas_eve += 1;
                }
                if ulid.0 & 1 == 1 {
                    acc.lsb_is_1 += 1;
                }

                acc
            }),
    )
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use std::time::Duration;
    use tokio::time::sleep;

    use super::super::routes_test;

    #[tokio::test]
    async fn task1() {
        let server = routes_test().await;
        server
            .post("/12/save/packet20231212")
            .await
            .assert_status_ok();
        sleep(Duration::new(2, 0)).await;
        server
            .get("/12/load/packet20231212")
            .await
            .assert_json(&2u64);
        sleep(Duration::new(2, 0)).await;
        server
            .get("/12/load/packet20231212")
            .await
            .assert_json(&4u64);
        server
            .post("/12/save/packet20231212")
            .await
            .assert_status_ok();
        server
            .get("/12/load/packet20231212")
            .await
            .assert_json(&0u64);
    }

    #[tokio::test]
    async fn task2() {
        routes_test()
            .await
            .post("/12/ulids")
            .json(&json!([
                "01BJQ0E1C3Z56ABCD0E11HYX4M",
                "01BJQ0E1C3Z56ABCD0E11HYX5N",
                "01BJQ0E1C3Z56ABCD0E11HYX6Q",
                "01BJQ0E1C3Z56ABCD0E11HYX7R",
                "01BJQ0E1C3Z56ABCD0E11HYX8P"
            ]))
            .await
            .assert_json(&json!([
                "015cae07-0583-f94c-a5b1-a070431f7516",
                "015cae07-0583-f94c-a5b1-a070431f74f8",
                "015cae07-0583-f94c-a5b1-a070431f74d7",
                "015cae07-0583-f94c-a5b1-a070431f74b5",
                "015cae07-0583-f94c-a5b1-a070431f7494"
            ]));
    }

    #[tokio::test]
    async fn task3() {
        routes_test()
            .await
            .post("/12/ulids/5")
            .json(&json!([
                "00WEGGF0G0J5HEYXS3D7RWZGV8",
                "76EP4G39R8JD1N8AQNYDVJBRCF",
                "018CJ7KMG0051CDCS3B7BFJ3AK",
                "00Y986KPG0AMGB78RD45E9109K",
                "010451HTG0NYWMPWCEXG6AJ8F2",
                "01HH9SJEG0KY16H81S3N1BMXM4",
                "01HH9SJEG0P9M22Z9VGHH9C8CX",
                "017F8YY0G0NQA16HHC2QT5JD6X",
                "03QCPC7P003V1NND3B3QJW72QJ"
            ]))
            .await
            .assert_json(&json!({
              "christmas eve": 3,
              "weekday": 1,
              "in the future": 2,
              "LSB is 1": 5
            }));
    }
}
