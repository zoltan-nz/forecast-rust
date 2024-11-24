use crate::services::weather_service::{ServiceError, WeatherService};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct WeatherQuery {
    city: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherResponse {
    city: String,
    temperature: WeatherTemperature,
    hourly_forecast: Vec<HourlyForecast>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherTemperature {
    min: f64,
    max: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyForecast {
    time: String,
    temperature: f64,
}

pub async fn get_weather(Query(query): Query<WeatherQuery>) -> impl IntoResponse {
    match fetch_weather_data(&query.city).await {
        Ok(weather) => (StatusCode::OK, Json(weather)).into_response(),
        Err(err) => {
            let (status, message) = match err {
                ServiceError::CityNotFound(msg) => (StatusCode::NOT_FOUND, msg),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            };
            (status, Json(json!({ "error": message }))).into_response()
        }
    }
}

async fn fetch_weather_data(city: &str) -> Result<WeatherResponse, ServiceError> {
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

    let hourly_forecast = weather
        .hourly
        .time
        .iter()
        .zip(weather.hourly.temperature_2m.iter())
        .map(|(time, temp)| HourlyForecast {
            time: time.clone(),
            temperature: *temp,
        })
        .collect();

    Ok(WeatherResponse {
        city: city.to_string(),
        temperature: WeatherTemperature {
            min: min_temp,
            max: max_temp,
        },
        hourly_forecast,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::get, Router};
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_get_weather_api() {
        let app = Router::new().route("/api/weather", get(get_weather));
        let server = TestServer::new(app.into_make_service()).unwrap();

        // Test successful case
        let response = server
            .get("/api/weather")
            .add_query_param("city", "London")
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: WeatherResponse = response.json();
        assert_eq!(body.city, "London");

        // Test invalid city
        let response = server
            .get("/api/weather")
            .add_query_param("city", "NonExistentCity123")
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }
}
