# Project Improvement TODO List

## High Priority
- **Configuration Management**: Replace hardcoded values (DB URL, port, API endpoints) with a centralized config module using environment variables
- **Authentication**: Implement proper auth for admin routes, especially the stats page which lacks protection
- **Caching**: Add caching for geocoding results and weather data to reduce external API calls
- **Error Handling**: Standardize error types and improve propagation between services and API layers

## Medium Priority
- **Test Coverage**: Add integration tests with mocks for external API calls
- **Dependency Injection**: Refactor service creation for better testability (currently created directly in handlers)
- **Rate Limiting**: Add protection for both external API calls and public endpoints
- **API Documentation**: Implement OpenAPI/Swagger specs for better developer experience

## Low Priority
- **Logging**: Implement structured logging and request tracing
- **Performance**: Optimize database queries and connection pooling
- **Frontend**: Improve templates with client-side validation and better mobile responsiveness
- **Security**: Add CSRF protection and more thorough input validation