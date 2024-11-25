use crate::services::weather_service::ServiceError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ServiceError::CityNotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ServiceError::GeocodingError(msg) | ServiceError::WeatherError(msg) => {
                (StatusCode::BAD_GATEWAY, msg)
            }
            ServiceError::InvalidResponse(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to process response: {e}"),
            ),
        };

        (status, message).into_response()
    }
}
