use axum::{
    extract,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

#[derive(serde::Deserialize, Debug)]
struct Reindeer {
    name: String,
    strength: u32,
    speed: Option<f64>,
    height: Option<u32>,
    antler_width: Option<u32>,
    snow_magic_power: Option<u32>,
    favorite_food: Option<String>,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: Option<u32>,
}

async fn strength(extract::Json(payload): extract::Json<Vec<Reindeer>>) -> impl IntoResponse {
    let total_strength: u32 = payload.iter().map(|r| r.strength).sum();
    (StatusCode::OK, total_strength.to_string()).into_response()
}

#[derive(serde::Serialize)]
struct ContestResults {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn contest(extract::Json(payload): extract::Json<Vec<Reindeer>>) -> impl IntoResponse {
    let mut fastest: Option<(f64, &Reindeer)> = None;
    let mut tallest: Option<(u32, &Reindeer)> = None;
    let mut magician: Option<(u32, &Reindeer)> = None;
    let mut consumer: Option<(u32, &Reindeer)> = None;

    for reindeer in &payload {
        if let Some(speed) = reindeer.speed {
            if fastest.is_none() || speed > fastest.unwrap().0 {
                fastest = Some((speed, reindeer));
            }
        }

        if let Some(height) = reindeer.height {
            if tallest.is_none() || height > tallest.unwrap().0 {
                tallest = Some((height, reindeer));
            }
        }

        if let Some(snow_magic_power) = reindeer.snow_magic_power {
            if magician.is_none() || snow_magic_power > magician.unwrap().0 {
                magician = Some((snow_magic_power, reindeer));
            }
        }

        if let Some(candies_eaten_yesterday) = reindeer.candies_eaten_yesterday {
            if consumer.is_none() || candies_eaten_yesterday > consumer.unwrap().0 {
                consumer = Some((candies_eaten_yesterday, reindeer));
            }
        }
    }

    let result: ContestResults = ContestResults {
        fastest: fastest
            .map(|(_, r)| {
                format!(
                    "Speeding past the finish line with a strength of {} is {}",
                    r.strength, r.name
                )
            })
            .unwrap_or_else(|| "No reindeer with valid speed found.".to_string()),
        tallest: tallest
            .map(|(_, r)| {
                format!(
                    "{} is standing tall with his {} cm wide antlers",
                    r.name,
                    r.antler_width.unwrap_or(0)
                )
            })
            .unwrap_or_else(|| "No reindeer with valid height found.".to_string()),
        magician: magician
            .map(|(_, r)| {
                format!(
                    "{} could blast you away with a snow magic power of {}",
                    r.name,
                    r.snow_magic_power.unwrap_or(0)
                )
            })
            .unwrap_or_else(|| "No reindeer with valid snow magic power found.".to_string()),
        consumer: consumer
            .map(|(_, r)| {
                format!(
                    "{} ate lots of candies, but also some {}",
                    r.name,
                    r.favorite_food
                        .clone()
                        .unwrap_or_else(|| "unknown food".to_string())
                )
            })
            .unwrap_or_else(|| "No reindeer with valid candy consumption found.".to_string()),
    };

    extract::Json(result)
}

async fn server_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let nest_router = Router::new()
        .route("/strength", post(strength))
        .route("/contest", post(contest));

    let router = Router::new().nest("/4", nest_router).fallback(server_error);

    Ok(router.into())
}
