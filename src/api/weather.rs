use crate::services::weather_service::{ServiceError, WeatherService};
use axum::extract::Query;
use axum::response::Html;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    city: String,
}

pub async fn fetch(Query(query): Query<QueryParams>) -> Result<Html<String>, ServiceError> {
    let service = WeatherService::new();

    let coords = service.fetch_coordinates(&query.city).await?;
    let weather = service.fetch_weather(&coords).await?;

    Ok(Html(format!(
        "Weather for {}: Temperature ranges from {:.1}°C to {:.1}°C",
        query.city,
        weather
            .hourly
            .temperature_2m
            .iter()
            .fold(f64::INFINITY, |a, &b| a.min(b)),
        weather
            .hourly
            .temperature_2m
            .iter()
            .fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::{routing::get, Router};
    use axum_test::TestServer;
    use pretty_assertions::assert_eq;

    fn setup() -> TestServer {
        let app = Router::new().route("/weather", get(fetch));

        TestServer::new(app.into_make_service()).unwrap()
    }

    #[tokio::test]
    async fn test_get_weather() {
        let server = setup();

        let response = server
            .get("/weather")
            .add_query_param("city", "London")
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(response.text().contains("Weather for London"));
        assert!(response.text().contains("°C"));
    }

    #[tokio::test]
    async fn test_get_weather_not_found() {
        let server = setup();

        let response = server
            .get("/weather")
            .add_query_param("city", "ThisCityDoesNotExist123")
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        assert_eq!(response.text(), "No coordinates found for ThisCityDoesNotExist123");
    }

    #[tokio::test]
    async fn test_get_weather_missing_param() {
        let server = setup();

        let response = server.get("/weather").await;

        assert_eq!(
            response.status_code(),
            StatusCode::BAD_REQUEST,
            "Missing required query parameter 'city' should return 400 Bad Request"
        );

        // Check if the error message indicates the missing parameter
        let error_text = response.text();
        assert!(
            error_text.contains("city"),
            "Error message should mention the missing 'city' parameter\nGot: {error_text}"
        );
    }

    #[tokio::test]
    async fn test_get_weather_empty_city() {
        let server = setup();

        let response = server.get("/weather").add_query_param("city", "").await;

        assert_eq!(
            response.status_code(),
            StatusCode::NOT_FOUND,
            "Empty city parameter should return 404 Not Found"
        );
    }
}
