use crate::repositories::CityRepository;
use crate::services::weather_service::{ServiceError, WeatherService};
use askama_axum::Template;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use log::warn;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    city: String,
}

#[derive(Template)]
#[template(path = "weather.html")]
struct WeatherTemplate {
    city: String,
    min_temp: f64,
    max_temp: f64,
    hourly_forecasts: Vec<HourlyForecast>,
}

#[derive(Debug)]
struct HourlyForecast {
    time: String,
    temp: f64,
}

pub async fn show(
    State(db): State<DatabaseConnection>,
    Query(query): Query<QueryParams>,
) -> impl IntoResponse {
    let repository = CityRepository::new(db);

    match generate_weather_response(repository, &query.city).await {
        Ok(html) => (StatusCode::OK, html).into_response(),
        Err(err) => {
            let (status, message) = match err {
                ServiceError::CityNotFound(msg) => (StatusCode::NOT_FOUND, msg),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            };
            (status, Html(message)).into_response()
        }
    }
}

async fn generate_weather_response(
    repository: CityRepository,
    city: &str,
) -> Result<Html<String>, ServiceError> {
    let service = WeatherService::new();

    let coords = service.fetch_coordinates(city).await?;

    if let Err(err) = repository
        .save_search(city.to_string(), &coords, None)
        .await
    {
        warn!("Failed to save search history: {}", err);
    }

    let weather = service.fetch_weather(&coords).await?;

    let min_temp = weather
        .hourly
        .temperature_2m
        .iter()
        .fold(f64::INFINITY, |a, &b| a.min(b));
    let max_temp = weather
        .hourly
        .temperature_2m
        .iter()
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let hourly_forecasts: Vec<HourlyForecast> = weather
        .hourly
        .time
        .iter()
        .zip(weather.hourly.temperature_2m.iter())
        .map(|(time, temp)| HourlyForecast {
            time: format_time(time),
            temp: *temp,
        })
        .collect();

    let template = WeatherTemplate {
        city: city.to_string(),
        min_temp,
        max_temp,
        hourly_forecasts,
    };

    let html = template
        .render()
        .map_err(|e| ServiceError::WeatherError(format!("Failed to render template: {e}")))?;

    Ok(Html(html))
}

fn format_time(time: &str) -> String {
    time.split('T')
        .nth(1)
        .unwrap_or(time)
        .trim_end_matches(":00")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::get, Router};
    use axum_test::TestServer;
    use sea_orm::Database;
    use sea_orm_migration::MigratorTrait;
    use crate::handlers;

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
    async fn test_show_weather_page() {
        let db = setup_test_db().await;
        let app = Router::new().route("/weather", get(handlers::weather::show)).with_state(db);
        let server = TestServer::new(app.into_make_service()).unwrap();

        let response = server
            .get("/weather")
            .add_query_param("city", "London")
            .await;

        assert_eq!(response.status_code(), 200);
        let html = response.text();
        assert!(html.contains("Weather for London"));
        assert!(html.contains("°C"));
    }
}
