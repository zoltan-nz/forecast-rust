# Weather Forecast Web Application

A Rust-based weather forecast web application that provides current weather and 5-day forecasts using the Open-Meteo API.

## 1. Project Summary

We're building a web-based weather forecast application that allows users to search for weather information by city name. The application will have both a user interface and an API backend.

### Key Features

- 🌍 City-based weather search
- 🌡️ Current weather conditions
- 📅 5-day weather forecast
- 📊 Search history tracking
- 🔒 Protected admin statistics

## 2. Wireframes

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

## 3. API Documentation

Endpoints

- GET / - Home page with search form
- GET /weather?city={city} - Weather for specified city
- GET /stats - Search statistics (requires authentication)

## 4. Todo List

1. Project Setup
    - [x] Initialize Rust project
    - [x] Create basic project structure

2. Geo Location Service
    - [x] Implement FetchLatLong function (TDD)
    - [x] Add error handling and input validation

3. Weather Service
    - [x] Implement FetchWeather function (TDD)
    - [x] Parse and structure weather data (current and 5-day forecast)

4. API Server
    - [x] Setup Actix Web server
    - [x] Create /api/weather endpoint
    - [x] Integrate Geo Location and Weather services

5. Database Integration
    - [ ] Set up SQLite connection
    - [ ] Create Cities table migration
    - [ ] Implement functions to store and retrieve recent searches

6. HTML Templates
    - [ ] Create index page with search form and autocomplete
    - [ ] Create weather results page with current weather and 5-day forecast
    - [ ] Create admin statistics page
    - [ ] Implement template rendering in API server

7. Authentication
    - [ ] Implement basic auth for /api/stats endpoint
    - [ ] Create /api/stats endpoint to show recent searches and total count

8. User Interface
    - [ ] Implement city search autocomplete functionality
    - [ ] Style pages with CSS for a better user experience

9. Error Handling and Logging
    - [ ] Implement consistent error handling across the application
    - [ ] Add logging for important events and errors

10. Testing
    - [ ] Unit tests for all main functions
    - [ ] Integration tests for API endpoints

11. Documentation
    - [ ] Write API documentation
    - [ ] Add usage instructions in README.md

## External APIs

- Geocoding API: https://geocoding-api.open-meteo.com/v1/search
- Weather API: https://api.open-meteo.com/v1/forecast

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

- we need only `latitude`, `longitude`.

## Using Weather API

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

# Development Setup

## Prerequisites

- Rust 1.82 or higher
- SQLite 3.47 or higher
- Docker (optional, for containerized development)

## Project Setup

After cloning the repository, setup the local `.env` file with copying the `.env.example` file.

```bash
cp .env.example .env
```

Install dependencies and run the application:

```bash
cargo check
cargo build
cargo run
```

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

- migration file is created in the `migrations` folder

### Project Structure

```
src/
├── api/          # API handlers
├── services/     # Business logic
├── models/       # Domain models
├── db/           # Database interactions
├── templates/    # HTML templates
└── main.rs       # Application entry point
```