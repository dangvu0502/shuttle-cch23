use std::io::Cursor;

use axum::{body::Bytes, response::IntoResponse, routing::post, Json, Router};
use tar::Archive;

pub(super) fn route() -> Router {
    Router::new()
    .route("/archive_files", post(archive_files))
    .route("/archive_files_size", post(archive_files_size))
    .route("/cookie", post(cookie))
}

async fn archive_files(body: Bytes) -> Json<usize> {
    let cursor = Cursor::new(body);
    let mut archive = Archive::new(cursor);
    match archive.entries() {
        Ok(entries) => Json(entries.count()),
        Err(_) => Json(0)
    }
}

async fn archive_files_size(body: Bytes) -> Json<usize> {
    let cursor = Cursor::new(body);
    let mut archive = Archive::new(cursor);

    match archive.entries() {
        Ok(entries) => {
            let mut total_size = 0;
            for entry in entries {
                if let Ok(file) = entry {
                    total_size += file.size() as usize;
                };
            }

            Json(total_size)
        }
        Err(_) => Json(0),
    }
}

async fn cookie(body: Bytes) -> impl IntoResponse {
    todo!()
}


#[cfg(test)]
mod test {
    use crate::days::routes_test;
    use axum::body::Bytes;

    #[tokio::test]
    async fn task1() {
        let north_pole = Bytes::from_static(include_bytes!("../../assets/northpole20231220.tar"));
        let server = routes_test().await;
        server
            .post("/20/archive_files")
            .bytes(north_pole.clone())
            .await
            .assert_text("6");
        server
            .post("/20/archive_files_size")
            .bytes(north_pole)
            .await
            .assert_text("1196282");
    }

    // #[tokio::test]
    async fn task2() {
        let c = Bytes::from_static(include_bytes!("../../assets/cookiejar.tar"));
        routes_test()
            .await
            .post("/20/cookie")
            .bytes(c)
            .await
            .assert_text("Grinch 71dfab551a1958b35b7436c54b7455dcec99a12c");
    }
}