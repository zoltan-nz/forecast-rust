use axum::routing::get;
use axum::Router;

async fn index() -> &'static str {
    "Index"
}

async fn weather() -> &'static str {
    "Weather"
}

async fn stats() -> &'static str {
    "Stats"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(index))
        .route("/weather", get(weather))
        .route("/stats", get(stats));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}