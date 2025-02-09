use crate::services::weather_service::{ServiceError, WeatherService};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    city: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    city: String,
    temperature: Temperature,
    hourly_forecast: Vec<HourlyForecast>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    min: f64,
    max: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyForecast {
    time: String,
    temperature: f64,
}

pub async fn get(Query(query): Query<QueryParams>) -> impl IntoResponse {
    match fetch_data(&query.city).await {
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

async fn fetch_data(city: &str) -> Result<Response, ServiceError> {
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

    Ok(Response {
        city: city.to_string(),
        temperature: Temperature {
            min: min_temp,
            max: max_temp,
        },
        hourly_forecast,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::weather;
    use axum::{routing::get, Router};
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_get_weather_api() {
        let app = Router::new().route("/api/weather", get(weather::get));
        let server = TestServer::new(app.into_make_service()).unwrap();

        // Test a successful case
        let response = server
            .get("/api/weather")
            .add_query_param("city", "London")
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: Response = response.json();
        assert_eq!(body.city, "London");

        // Test invalid city
        let response = server
            .get("/api/weather")
            .add_query_param("city", "NonExistentCity123")
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }
}
