use crate::services::weather_service::{ServiceError, WeatherService};
use askama_axum::Template;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
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

pub async fn fetch(Query(query): Query<QueryParams>) -> impl IntoResponse {
    match generate_weather_response(&query.city).await {
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

pub async fn generate_weather_response(city: &str) -> Result<Html<String>, ServiceError> {
    let service = WeatherService::new();

    let coords = service.fetch_coordinates(city).await?;
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

// Helper function to format the time string
fn format_time(time: &str) -> String {
    // Parse the ISO time string and format it more nicely
    // Example: "2024-02-23T12:00" -> "Feb 23, 12:00"
    time.split('T')
        .nth(1)
        .unwrap_or(time)
        .trim_end_matches(":00")
        .to_string()
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

        let html = response.text();
        assert!(html.contains("Weather for London"));
        assert!(html.contains("°C"));
        assert!(html.contains("Hourly Forecast"));
    }

    #[tokio::test]
    async fn test_get_weather_not_found() {
        let server = setup();

        let response = server
            .get("/weather")
            .add_query_param("city", "ThisCityDoesNotExist123")
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        assert_eq!(
            response.text(),
            "No coordinates found for ThisCityDoesNotExist123"
        );
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

    #[test]
    fn test_format_time() {
        assert_eq!(format_time("2024-02-23T12:00"), "12");
        assert_eq!(format_time("invalid"), "invalid");
        assert_eq!(format_time("2024-02-23T15:30"), "15:30");
    }
}
