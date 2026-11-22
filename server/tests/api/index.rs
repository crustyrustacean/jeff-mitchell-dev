// tests/api/index.rs

use crate::helpers::{spawn_app, spawn_app_with_renderer};
use server::template::{TemplateError, TemplateRenderer};
use std::sync::Arc;

struct FailingRenderer;

impl TemplateRenderer for FailingRenderer {
    fn render(
        &self,
        _template_name: &str,
        _context: &serde_json::Value,
    ) -> Result<String, TemplateError> {
        Err(TemplateError::Operation(anyhow::anyhow!(
            "simulated render failure"
        )))
    }
}

struct MissingTemplateRenderer;

impl TemplateRenderer for MissingTemplateRenderer {
    fn render(
        &self,
        template_name: &str,
        _context: &serde_json::Value,
    ) -> Result<String, TemplateError> {
        Err(TemplateError::NotFound(template_name.to_string()))
    }
}

#[tokio::test]
async fn index_page_returns_200_ok_and_renders_html_content() {
    let app = spawn_app().await;

    let response = app.get("/").await;

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
    assert!(body.contains("<!DOCTYPE html>"));
    assert!(body.contains("<title>jeff-mitchell.dev | Home</title>"));
    assert!(body.contains("<p>Welcome!</p>"));
}

#[tokio::test]
async fn unknown_route_renders_404_error_template() {
    let app = spawn_app().await;

    let response = app.get("/does-not-exist").await;

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
    assert!(body.contains("<!DOCTYPE html>"));
    assert!(body.contains("404"));
    assert!(body.contains("The page you are looking for does not exist."));
}

#[tokio::test]
async fn index_page_returns_500_when_template_rendering_fails() {
    let app = spawn_app_with_renderer(Arc::new(FailingRenderer)).await;

    let response = app.get("/").await;

    assert_eq!(
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        response.status()
    );
    let body = response.text().await.expect("Failed to read body.");
    assert_eq!(body, "Internal Server Error");
}

#[tokio::test]
async fn index_page_returns_404_when_template_is_missing() {
    let app = spawn_app_with_renderer(Arc::new(MissingTemplateRenderer)).await;

    let response = app.get("/").await;

    assert_eq!(reqwest::StatusCode::NOT_FOUND, response.status());
    let body = response.text().await.expect("Failed to read body.");
    assert_eq!(body, "Not Found");
}

#[tokio::test]
async fn not_found_page_returns_500_when_error_template_fails() {
    let app = spawn_app_with_renderer(Arc::new(FailingRenderer)).await;

    let response = app.get("/does-not-exist").await;

    assert_eq!(
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        response.status()
    );
    let body = response.text().await.expect("Failed to read body.");
    assert_eq!(body, "Internal Server Error");
}
