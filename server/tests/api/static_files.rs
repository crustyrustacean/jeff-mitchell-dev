// tests/api/static_files.rs

use crate::helpers::spawn_app;

#[tokio::test]
async fn root_serves_index_html() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(reqwest::StatusCode::OK, response.status());
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        content_type.starts_with("text/html"),
        "Expected text/html, got {content_type}"
    );
    let body = response.text().await.expect("Failed to read body.");
    assert!(!body.is_empty());
}

#[tokio::test]
async fn missing_static_file_returns_404() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/does-not-exist-anywhere", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(reqwest::StatusCode::NOT_FOUND, response.status());
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        content_type.starts_with("text/html"),
        "Expected text/html, got {content_type}"
    );
    let body = response.text().await.expect("Failed to read body.");
    assert!(!body.is_empty());
}
