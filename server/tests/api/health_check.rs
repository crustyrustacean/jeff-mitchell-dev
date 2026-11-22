// tests/api/health_check.rs

use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let response = app.get("/api/v1/health_check").await;

    assert_eq!(reqwest::StatusCode::OK, response.status());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn health_check_requires_a_supported_api_version() {
    let app = spawn_app().await;

    for path in ["/health_check", "/api/health_check", "/api/v2/health_check"] {
        let response = app.get(path).await;

        assert_eq!(
            reqwest::StatusCode::NOT_FOUND,
            response.status(),
            "Unexpected status for {path}"
        );
    }
}
