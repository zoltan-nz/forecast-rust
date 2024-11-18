# Weather Forecast Service

## Project Overview
A Rust-based web service that provides weather forecasts with:
- City-based weather search
- Historical search tracking
- Protected admin statistics

## Directory Structure
```
src/
├── api/              # HTTP API handlers
│   ├── mod.rs
│   ├── weather.rs    # Weather endpoints
│   └── admin.rs      # Admin endpoints
├── services/         # Business logic
│   ├── mod.rs
│   ├── weather.rs    # Weather service
│   └── city.rs       # City tracking service
├── models/           # Database entities
│   ├── mod.rs
│   └── cities.rs     # City model
├── config/           # Configuration
│   ├── mod.rs
│   └── settings.rs   # App settings
├── errors/           # Error handling
│   ├── mod.rs
│   └── api_error.rs  # API error types
└── main.rs          # Application entry point

migrations/          # Database migrations
templates/          # HTML templates
tests/             # Integration tests
```

## Core Features

### 1. Weather Search
- City name search with geocoding
- Current weather display
- 5-day forecast
- Caching of geocoding results

### 2. Admin Features
- Protected statistics dashboard
- Recent searches tracking
- Search frequency analytics

### 3. Technical Features
- RESTful API
- Database persistence
- Error handling
- Authentication
- API rate limiting
- Response caching

## External APIs

### Geocoding API
- Base URL: `https://geocoding-api.open-meteo.com/v1/search`
- Query params: `?name={city}&count=1&language=en&format=json`
- Rate limit: 10,000/day

### Weather API
- Base URL: `https://api.open-meteo.com/v1/forecast`
- Query params: `?latitude={lat}&longitude={lon}&hourly=temperature_2m`
- Rate limit: 10,000/day

### Using GeoCoding API

> [API Documentation](https://open-meteo.com/en/docs/geocoding-api)

Search URL: `https://geocoding-api.open-meteo.com/v1/search`

Query Parameters:
- `name`
  String to search for. An empty string or only 1 character will return an empty result. 2 characters will only match exact matching locations. 3 and more characters will perform fuzzy matching. The search string can be a location name or a postal code.
- `count`, default is `10` - should be set to `1`
- `format`, default is `json` - we don't need to list it
- `language`, default is `en` - we don't need to list it

Result:
```json
{
  "results": [
    {
      "id": 2950159,
      "name": "Berlin",
      "latitude": 52.52437,
      "longitude": 13.41053,
      "elevation": 74.0,
      "feature_code": "PPLC",
      "country_code": "DE",
      "admin1_id": 2950157,
      "admin2_id": 0,
      "admin3_id": 6547383,
      "admin4_id": 6547539,
      "timezone": "Europe/Berlin",
      "population": 3426354,
      "postcodes": [
        "10967",
        "13347"
      ],
      "country_id": 2921044,
      "country": "Deutschland",
      "admin1": "Berlin",
      "admin2": "",
      "admin3": "Berlin, Stadt",
      "admin4": "Berlin"
    },
    {}
  ]
}
```
- When the city is not valid
```json
{
}
```

- we need only `latitude`, `longitude`.

#### Using Weather API

> [API Documentation](https://open-meteo.com/en/docs/forecast-api)

Forecast URL: `https://api.open-meteo.com/v1/forecast`

- default time period is 7 days

Query Parameters:
- `latitude`, `longitude` (required)
  Geographical coordinates in decimal degrees
- `hourly`
  List of weather variables for current weather. We use:
    - `temperature_2m`
- `timezone`
    - If `auto` is set as a time zone, the coordinates will be automatically resolved to the local time zone.

Result:
```json
{
  "latitude": 43.70455,
  "longitude": -79.404625,
  "generationtime_ms": 0.0219345092773438,
  "utc_offset_seconds": -14400,
  "timezone": "America/New_York",
  "timezone_abbreviation": "EDT",
  "elevation": 175,
  "hourly_units": {
    "time": "iso8601",
    "temperature_2m": "°C"
  },
  "hourly": {
    "time": [
      "2024-10-26T00:00",
      "2024-10-26T01:00"
    ],
    "temperature_2m": [9.4, 8.8]
  }
}
```


## Wireframes

### a) Home Page (index.html):
```
+----------------------------------+
|        Weather Forecast          |
|                                  |
|  +----------------------------+  |
|  |     Enter city name        |  |
|  +----------------------------+  |
|           [Search]               |
|                                  |
+----------------------------------+
```

### b) Weather Results Page (weather.html):
```
+----------------------------------+
|   Weather for [City Name]        |
|                                  |
|   Current Weather:               |
|   Temperature: XX°C              |
|   Humidity: XX%                  |
|   Wind Speed: XX km/h            |
|                                  |
|   5-Day Forecast:                |
|   +----------------------------+ |
|   | Date | Temp | Conditions   | |
|   |------|------|--------------|
|   | Day1 | XX°C | Sunny        | |
|   | Day2 | XX°C | Cloudy       | |
|   | Day3 | XX°C | Rainy        | |
|   | Day4 | XX°C | Partly Cloudy| |
|   | Day5 | XX°C | Clear        | |
|   +----------------------------+ |
|                                  |
+----------------------------------+
```

### c) Admin Statistics Page (stats.html):
```
+----------------------------------+
|        Admin Statistics          |
|                                  |
|   Recent Searches:               |
|   +----------------------------+ |
|   | City      | Search Count   | |
|   |-----------|----------------|
|   | City1     | XX             | |
|   | City2     | XX             | |
|   | City3     | XX             | |
|   | City4     | XX             | |
|   | City5     | XX             | |
|   +----------------------------+ |
|                                  |
|   Total Searches: XXXX           |
|                                  |
+----------------------------------+
```

## Development Setup

### Prerequisites
```bash
rustup toolchain install 1.75+
cargo install sea-orm-cli
```

### Environment Setup
```bash
cp .env.example .env
# Configure DATABASE_URL and other settings
```
```env
# Database Configuration
DATABASE_URL=sqlite://weather.db?mode=rwc

# Server Configuration
PORT=3000
HOST=127.0.0.1

# Logging
RUST_LOG=info

# Authentication
ADMIN_USERNAME=forecast
ADMIN_PASSWORD=forecast

# Rate Limiting
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_DURATION_SECS=3600

# Cache Configuration
CACHE_TTL_SECS=3600
GEO_CACHE_TTL_SECS=86400  # 24 hours
```

### Database Setup
```bash
cargo run -p migration
```

### Run Development Server
```bash
cargo run
```

## Testing
```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test '*'
```

## API Documentation

### Public Endpoints
`GET /api/weather?city={city}`
- Returns current weather and forecast
- Rate limited to 100 requests per hour per IP

### Protected Endpoints
`GET /api/stats`
- Requires Basic Auth
- Returns search statistics

## Error Handling
| Status Code | Description           |
|-------------|--------------------|
| 400         | Invalid request    |
| 401         | Unauthorized       |
| 404         | City not found     |
| 429         | Rate limit exceeded|
| 500         | Internal error     |

## Design Decisions

### 1. Technology Choices
- **Axum**: Modern, async web framework
- **SeaORM**: Type-safe database access
- **SQLite**: Simple deployment, suitable for moderate load

### 2. Architecture
- Service layer pattern
- Repository pattern for data access
- Error-first design

## Development Logs

### Setting up the project

```
cargo init --name weather-forecast
```
### Creating migration for the database using SeaORM

```
cargo install sea-orm-cli
sea-orm-cli migrate init
sea-orm-cli migrate generate create_cities_table
```

