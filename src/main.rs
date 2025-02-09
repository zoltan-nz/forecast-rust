mod api;
mod entities;
mod errors;
mod handlers;
mod repositories;
mod services;

use axum::{http::Request, routing::get, Router};
use bytes::Bytes;
use env_logger::{Builder, WriteStyle};
use log::{debug, info, LevelFilter};
use sea_orm::DatabaseConnection;
use std::{net::SocketAddr, time::Duration};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;

#[tokio::main]
async fn main() {
    // Initialize custom logger
    Builder::new()
        .format_timestamp(Some(env_logger::fmt::TimestampPrecision::Seconds))
        .format_module_path(true)
        .format_target(true)
        .filter(None, LevelFilter::Info)
        .filter(Some("sqlx"), LevelFilter::Warn) // Reduce SQL noise
        .filter(Some("forecast_rust"), LevelFilter::Debug)
        .write_style(WriteStyle::Auto) // This will auto-detect if colors should be used
        .init();

    let db_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://weather.db".to_string());

    info!("Connecting to database at {}", db_url);

    // Enable logging for database connections
    let db = sea_orm::Database::connect(
        sea_orm::ConnectOptions::new(db_url)
            .max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Warn)
            .to_owned(),
    )
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
    // Create a trace layer with custom configuration
    let trace_layer = TraceLayer::new_for_http()
        .on_request(|request: &Request<_>, _span: &Span| {
            info!("Request: {} {}", request.method(), request.uri());
        })
        .on_response(
            |response: &axum::http::Response<_>, latency: Duration, _span: &Span| {
                info!(
                    "Response: {} completed in {}ms",
                    response.status(),
                    latency.as_millis()
                );
            },
        )
        .on_body_chunk(|chunk: &Bytes, _latency: Duration, _span: &Span| {
            debug!("Response chunk size: {} bytes", chunk.len());
        })
        .on_failure(
            |error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                log::error!(
                    "Request failed after {}ms: {:?}",
                    latency.as_millis(),
                    error
                );
            },
        );
    // API routes
    let api_router = Router::new().route("/weather", get(api::weather::get));

    // Page routes
    let page_router = Router::new()
        .route("/", get(handlers::pages::index))
        .route("/weather", get(handlers::weather::show))
        .route("/stats", get(handlers::stats::show));

    // Combine them
    Router::new()
        .nest("/api", api_router) // All API routes under /api
        .merge(page_router) // HTML pages at root level
        .with_state(db)
        .layer(trace_layer)
}
