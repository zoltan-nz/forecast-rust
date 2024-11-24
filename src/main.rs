mod api;
mod errors;
mod services;
mod handlers;

use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://weather.db".to_string());
    let db = sea_orm::Database::connect(&db_url)
        .await
        .expect("Database connection failed");

    let app = create_router(db);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn create_router(db: DatabaseConnection) -> Router {
    // API routes
    let api_router = Router::new().route("/weather", get(api::weather::get_weather));

    // Page routes
    let page_router = Router::new()
        .route("/", get(handlers::pages::index))
        .route("/weather", get(handlers::weather::show));

    // Combine them
    Router::new()
        .nest("/api", api_router) // All API routes under /api
        .merge(page_router) // HTML pages at root level
        .with_state(db)
}
