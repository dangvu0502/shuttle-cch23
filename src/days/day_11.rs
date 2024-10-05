use axum::{extract::Multipart, routing::post, Json, Router};
use image::GenericImageView;
use tower_http::services::ServeDir;

pub(super) fn route() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/red_pixels", post(count_pixels))
}

async fn count_pixels(mut multipart: Multipart) -> Json<usize> {
    let Some(field) = multipart.next_field().await.unwrap() else {
        return axum::Json(0);
    };

    let img = image::load_from_memory(field.bytes().await.unwrap().as_ref()).unwrap();

    img.pixels()
        .filter(|x| {
            let [r, g, b, _] = x.2 .0;
            u16::from(r) > (u16::from(g) + u16::from(b))
        })
        .count()
        .into()
}
