use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn index() -> IndexTemplate {
    IndexTemplate
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::get, Router};
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_index_page() {
        let app = Router::new().route("/", get(index));
        let server = TestServer::new(app.into_make_service()).unwrap();

        let response = server.get("/").await;

        assert_eq!(response.status_code(), 200);
        let html = response.text();
        assert!(html.contains("Weather Forecast"));
        assert!(html.contains(r#"<form action="/weather""#));
    }
}
