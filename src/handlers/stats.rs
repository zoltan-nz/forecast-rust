use crate::repositories::CityRepository;
use askama_axum::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use hyper::StatusCode;
use sea_orm::DatabaseConnection;

#[derive(Template)]
#[template(path = "stats.html")]
struct StatsTemplate {
    searches: Vec<SearchRecord>,
}

#[derive(Debug)]
struct SearchRecord {
    name: String,
    created_at: String,
    lat: f32,
    long: f32,
}

pub async fn show(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    let repository = CityRepository::new(db);
    match repository.get_recent_searches(10).await {
        Ok(models) => {
            let searches = models
                .into_iter()
                .map(|model| SearchRecord {
                    name: model.name,
                    created_at: model.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                    lat: model.lat,
                    long: model.long,
                })
                .collect();

            let template = StatsTemplate { searches };
            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to render template",
                )
                    .into_response(),
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch searches",
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use crate::handlers;
    use axum::{routing::get, Router};
    use axum_test::TestServer;
    use sea_orm::{Database, DatabaseConnection};
    use sea_orm_migration::MigratorTrait;

    async fn setup_test_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("Failed to create test database");

        migration::Migrator::up(&db, None)
            .await
            .expect("Failed to run migrations");

        db
    }

    #[tokio::test]
    async fn test_stats_page_auth() {
        let db = setup_test_db().await;
        let app = Router::new()
            .route("/stats", get(handlers::stats::show))
            .with_state(db);
        let server = TestServer::new(app.into_make_service()).unwrap();

        let response = server.get("/stats").await;

        assert_eq!(response.status_code(), 200);
        let html = response.text();
        assert!(html.contains("Recent Searches"));
    }
}
