use axum::{
    extract::{self, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

#[derive(serde::Serialize)]
struct Elf {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

async fn count(payload: String) -> impl IntoResponse {
    const SHELF: &str = "shelf";
    const ELF_ON_A_SHELF: &str = "elf on a ";

    let (mut elf_on_a_shelf, mut shelf_with_no_elf_on_it) = (0, 0);
    
    for i in 0..=payload.len() - SHELF.len() {
        if &payload[i..i + SHELF.len()] == SHELF {
            if i >= ELF_ON_A_SHELF.len() && &payload[(i-ELF_ON_A_SHELF.len())..i] == ELF_ON_A_SHELF {
                elf_on_a_shelf += 1;
            } else {
                shelf_with_no_elf_on_it += 1;
            }
        }
    }

    extract::Json(Elf {
        elf: payload.matches("elf").count(),
        elf_on_a_shelf,
        shelf_with_no_elf_on_it,
    })
}

async fn server_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let nest_router = Router::new().route("/", post(count));

    let router = Router::new().nest("/6", nest_router).fallback(server_error);

    Ok(router.into())
}