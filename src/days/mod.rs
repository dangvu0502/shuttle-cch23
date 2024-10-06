use axum::Router;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;

pub fn routes() -> Router {
    Router::new()
        .nest("/1", day_01::route())
        .nest("/2", day_02::route())
        .nest("/3", day_03::route())
        .nest("/4", day_04::route())
        .nest("/5", day_05::route())
        .nest("/6", day_06::route())
        .nest("/7", day_07::route())
        .nest("/8", day_08::route())
        .nest("/9", day_09::route())
        .nest("/10", day_10::route())
        .nest("/11", day_11::route())
        .nest("/12", day_12::route())

}

#[cfg(test)]
pub(crate) async fn routes_test() -> axum_test::TestServer {
    // let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    // sqlx::migrate!().run(&pool).await.unwrap();
    // Force to init the tracing subscriber, first test call will succeed, rest will error out

    use axum_test::Transport;
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
    let app = routes().layer(tower_http::trace::TraceLayer::new_for_http());
    let config = axum_test::TestServerConfig {
        save_cookies: true,
        expect_success_by_default: true,
        transport: Some(Transport::MockHttp),
        ..Default::default()
    };

    axum_test::TestServer::new_with_config(app, config).unwrap()
}