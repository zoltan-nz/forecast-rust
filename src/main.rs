mod api;
mod errors;
mod handlers;
mod services;

use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "forecast_rust=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(true)
                .with_level(true)
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    let db_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://weather.db".to_string());

    info!("Connecting to database at {}", db_url);
    let db = sea_orm::Database::connect(&db_url)
        .await
        .expect("Database connection failed");
    info!("Database connection established");

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
    let api_router = Router::new().route("/weather", get(api::weather::get));

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
