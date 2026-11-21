// tests/api/health_check.rs

use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(format!("{}/api/v1/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(reqwest::StatusCode::OK, response.status());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn health_check_requires_a_supported_api_version() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    for path in ["/health_check", "/api/health_check", "/api/v2/health_check"] {
        let response = client
            .get(format!("{}{path}", app.address))
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            reqwest::StatusCode::NOT_FOUND,
            response.status(),
            "Unexpected status for {path}"
        );
    }
}