use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;

const GEOCODING_API_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const WEATHER_API_URL: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Failed to fetch coordinates: {0}")]
    GeocodingError(String),

    #[error("Failed to fetch weather: {0}")]
    WeatherError(String),

    #[error("City not found")]
    CityNotFound(String),

    #[error("Failed to parse response")]
    InvalidResponse(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize, Clone)]
pub struct LatLong {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct GeoResponse {
    pub results: Option<Vec<LatLong>>,
}

#[derive(Debug, Deserialize)]
pub struct WeatherData {
    pub hourly: HourlyData,
}

#[derive(Debug, Deserialize)]
pub struct HourlyData {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
}

pub struct WeatherService {
    client: Client,
}

impl WeatherService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_coordinates(&self, city: &str) -> Result<LatLong, ServiceError> {
        tracing::debug!("Fetching coordinates for city: {}", city);

        if city.trim().is_empty() {
            tracing::warn!("Empty city name provided");
            return Err(ServiceError::CityNotFound(
                "City name cannot be empty".to_string(),
            ));
        }

        let url = format!("{GEOCODING_API_URL}?name={city}&count=1&language=en&format=json");
        tracing::debug!("Geocoding API request: {}", url);

        let response = self.client.get(&url).send().await.map_err(|e| {
            tracing::error!("Geocoding API request failed: {}", e);
            ServiceError::GeocodingError(e.to_string())
        })?;

        let geo_data: GeoResponse = response.json().await.map_err(|e| {
            tracing::error!("Failed to parse Geocoding API response: {}", e);
            ServiceError::GeocodingError(format!("Failed to parse JSON: {e}"))
        })?;

        match geo_data.results {
            Some(results) if !results.is_empty() => {
                tracing::info!("Found coordinates for {}: {:?}", city, results[0]);
                Ok(results[0].clone())
            }
            _ => {
                tracing::warn!("No coordinates found for city: {}", city);
                Err(ServiceError::CityNotFound(format!(
                    "No coordinates found for {city}"
                )))
            }
        }
    }

    pub async fn fetch_weather(&self, coords: &LatLong) -> Result<WeatherData, ServiceError> {
        tracing::debug!(
            "Fetching weather for coordinates: lat={}, lon={}",
            coords.latitude,
            coords.longitude
        );

        let url = format!(
            "{WEATHER_API_URL}?latitude={}&longitude={}&hourly=temperature_2m",
            coords.latitude, coords.longitude
        );
        tracing::debug!("Weather API request: {}", url);

        let response = self.client.get(&url).send().await.map_err(|e| {
            tracing::error!("Weather API request failed: {}", e);
            ServiceError::WeatherError(e.to_string())
        })?;

        let weather_data = response.json().await.map_err(|e| {
            tracing::error!("Failed to parse Weather API response: {}", e);
            ServiceError::WeatherError(e.to_string())
        })?;

        tracing::info!("Successfully fetched weather data");
        Ok(weather_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[tokio::test]
    async fn test_fetch_coordinates_london() {
        let service = WeatherService::new();
        let result = service.fetch_coordinates("London").await;

        assert!(
            result.is_ok(),
            "Expected Ok result for London, got {result:?}"
        );

        let coords = result.unwrap();
        assert!(
            (50.0..52.0).contains(&coords.latitude),
            "London latitude {} should be between 50 and 52",
            coords.latitude
        );
        assert!(
            (-1.0..1.0).contains(&coords.longitude),
            "London longitude {} should be between -1 and 1",
            coords.longitude
        );

        println!("London coordinates: {coords:?}");
    }

    #[test_case("Paris"   ; "when querying Paris")]
    #[test_case("Berlin"  ; "when querying Berlin")]
    #[test_case("Tokyo"   ; "when querying Tokyo")]
    #[test_case("NewYork" ; "when querying NewYork")]
    #[tokio::test]
    async fn test_fetch_coordinates_major_cities(city: &str) {
        let service = WeatherService::new();
        let result = service.fetch_coordinates(city).await;

        assert!(
            result.is_ok(),
            "Failed to fetch coordinates for {city}: {result:?}"
        );

        let coords = result.unwrap();
        println!(
            "{} coordinates: {:.4}°N, {:.4}°E",
            city, coords.latitude, coords.longitude
        );
    }

    #[tokio::test]
    async fn test_fetch_coordinates_invalid_city() {
        let service = WeatherService::new();
        let result = service.fetch_coordinates("ThisCityDoesNotExist123").await;

        match result {
            Err(ServiceError::CityNotFound(msg)) => {
                assert!(
                    msg.contains("ThisCityDoesNotExist123"),
                    "Error message '{msg}' should contain the city name"
                );
                println!("Got expected error: {msg}");
            }
            other => panic!("Expected CityNotFound error, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_fetch_weather() {
        let service = WeatherService::new();
        let coords = LatLong {
            latitude: 51.5074,
            longitude: -0.1278,
        };
        let result = service.fetch_weather(&coords).await;
        assert!(result.is_ok());

        let weather = result.unwrap();
        assert!(!weather.hourly.time.is_empty());
        assert!(!weather.hourly.temperature_2m.is_empty());
        assert_eq!(
            weather.hourly.time.len(),
            weather.hourly.temperature_2m.len()
        );
    }
}
