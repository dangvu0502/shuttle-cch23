use axum::{extract, response::IntoResponse, routing::post, Router};


pub(super) fn route() -> Router {
    Router::new().route("/", post(elf_count))
}

#[derive(serde::Serialize)]
struct Elf {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

async fn elf_count(payload: String) -> impl IntoResponse {
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


