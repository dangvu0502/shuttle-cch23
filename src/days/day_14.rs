use axum::{
    response::{Html, IntoResponse},
    routing::post,
    Json, Router,
};

pub(super) fn route() -> Router {
    Router::new().route("/unsafe", post(unsafe_html))
    .route("/safe", post(safe_html))
}

#[derive(serde::Deserialize)]
struct Payload {
    content: String,
}

async fn unsafe_html(Json(payload): Json<Payload>) -> impl IntoResponse {
    Html(format!(
        r"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>",
        &payload.content
    ))
}
fn custom_escape(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#x27;")
}

async fn safe_html(Json(payload): Json<Payload>) -> impl IntoResponse {
    Html(format!(
        r"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>",
        custom_escape(&payload.content))
    )
}

#[cfg(test)]
mod test {
    use serde_json::json;

    #[tokio::test]
    async fn task1() {
        super::super::routes_test()
            .await
            .post("/14/unsafe")
            .json(&json!({"content": "<h1>Welcome to the North Pole!</h1>"}))
            .await
            .assert_text(
                r"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    <h1>Welcome to the North Pole!</h1>
  </body>
</html>",
            );
    }

    #[tokio::test]
    async fn task2() {
        super::super::routes_test()
            .await
            .post("/14/safe")
            .json(&json!({"content": "<script>alert(\"XSS Attack!\")</script>"}))
            .await
            .assert_text(
                r"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    &lt;script&gt;alert(&quot;XSS Attack!&quot;)&lt;/script&gt;
  </body>
</html>",
            );
    }
}