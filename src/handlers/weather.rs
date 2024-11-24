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

pub async fn show(Query(query): Query<QueryParams>) -> impl IntoResponse {
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

async fn generate_weather_response(city: &str) -> Result<Html<String>, ServiceError> {
    let service = WeatherService::new();

    let coords = service.fetch_coordinates(city).await?;
    let weather = service.fetch_weather(&coords).await?;

    let min_temp = weather.hourly.temperature_2m.iter()
        .fold(f64::INFINITY, |a, &b| a.min(b));
    let max_temp = weather.hourly.temperature_2m.iter()
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let hourly_forecasts: Vec<HourlyForecast> = weather.hourly.time.iter()
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
    use axum::{
        Router,
        routing::get,
    };
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_show_weather_page() {
        let app = Router::new().route("/weather", get(show));
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