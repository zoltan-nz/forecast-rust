mod services;

use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize login
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create SQLite database connection with SeaOrm
    let db_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://weather.db".to_string());
    let db = sea_orm::Database::connect(&db_url)
        .await
        .expect("Database connection failed");

    // Build the app
    let app = create_router(db);

    // Run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn create_router(db: DatabaseConnection) -> Router {
    Router::new().route("/", get(health_check)).with_state(db)
}

async fn health_check() -> &'static str {
    "I'm alive!"
}
