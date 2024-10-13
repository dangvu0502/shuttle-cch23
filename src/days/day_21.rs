use std::{num::NonZeroU32, sync::Arc};

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use governor::{
    clock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
};
use s2::{cellid::CellID, latlng::LatLng};

struct Day21State {
    client: reqwest::Client,
    rate_limiter:
        governor::RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
}

pub(super) fn route() -> Router {
    Router::new()
        .route("/coords/:binary", get(coords))
        .route("/country/:binary", get(country))
        .with_state(Arc::new(Day21State {
            client: reqwest::Client::new(),
            rate_limiter: governor::RateLimiter::direct(governor::Quota::per_second(
                NonZeroU32::new(1).unwrap(),
            )),
        }))
}

async fn coords(Path(binary): Path<String>) -> String {
    let lat_long = u64::from_str_radix(&binary, 2)
        .map(CellID)
        .map(LatLng::from)
        .unwrap();

    let lat = lat_long.lat.deg();
    let lng = lat_long.lng.deg();

    let latitude_indicator = if lat >= 0.0 { 'N' } else { 'S' };
    let longitude_indicator = if lng >= 0.0 { 'E' } else { 'W' };

    format!(
        "{}{} {}{}",
        degrees_to_dms(lat),
        latitude_indicator,
        degrees_to_dms(lng),
        longitude_indicator
    )
}

#[derive(serde::Deserialize)]
struct Record {
    address: Address,
}

#[derive(serde::Deserialize)]
struct Address {
    country: String,
}

async fn country(State(s): State<Arc<Day21State>>, Path(binary): Path<String>) -> String {
    let lat_long = u64::from_str_radix(&binary, 2)
        .map(CellID)
        .map(LatLng::from)
        .unwrap();

    let lat = lat_long.lat.deg();
    let lng = lat_long.lng.deg();

    tokio::time::interval(tokio::time::Duration::from_secs(2)).tick().await;
    s.rate_limiter.until_ready().await;
    s.client
        .get(format!(
            "https://geocode.maps.co/reverse?lat={lat}&lon={lng}&api_key=???"
        ))
        .send()
        .await
        .unwrap()
        .json::<Record>()
        .await
        .unwrap()
        .address
        .country
}

fn degrees_to_dms(degrees: f64) -> String {
    // 83°39'54.324''N
    // Convert decimal degrees to degrees, minutes, seconds
    let m = (degrees.fract() * 60.0).trunc();
    let s = (degrees.fract() * 3600.0 - m * 60.0).abs();
    format!("{}°{}'{:.03}''", degrees.trunc().abs(), m.abs(), s.abs())
}

#[cfg(test)]
mod test {
    use crate::days::routes_test;

    #[tokio::test]
    async fn task1() {
        let server = routes_test().await;

        server
            .get("/21/coords/0100111110010011000110011001010101011111000010100011110001011011")
            .await
            .assert_text("83°39'54.324''N 30°37'40.584''W");

        server
            .get("/21/coords/0010000111110000011111100000111010111100000100111101111011000101")
            .await
            .assert_text("18°54'55.944''S 47°31'17.976''E");
    }

    // #[tokio::test]
    async fn task2() {
        let server = routes_test().await;
        server
            .get("/21/country/0010000111110000011111100000111010111100000100111101111011000101")
            .await
            .assert_text("Madagascar");
        server
            .get("/21/country/0011001000100010100010110001110100000111000010111000100000010101")
            .await
            .assert_text("Brunei");
    }
}
