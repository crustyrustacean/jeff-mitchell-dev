// tests/api/index.rs

use crate::helpers::{spawn_app, spawn_app_with_renderer};
use server::template::{TemplateError, TemplateRenderer};
use std::sync::{Arc, Mutex};

struct RecordingRenderer {
    rendered_templates: Mutex<Vec<String>>,
}

impl RecordingRenderer {
    fn new() -> Self {
        Self {
            rendered_templates: Mutex::new(Vec::new()),
        }
    }

    fn rendered(&self) -> Vec<String> {
        self.rendered_templates.lock().unwrap().clone()
    }
}

impl TemplateRenderer for RecordingRenderer {
    fn render(
        &self,
        template_name: &str,
        _context: &serde_json::Value,
    ) -> Result<String, TemplateError> {
        self.rendered_templates
            .lock()
            .unwrap()
            .push(template_name.to_string());
        Ok(format!("<!DOCTYPE html><html><body>{template_name}</body></html>"))
    }
}

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
async fn index_page_returns_200_ok_and_renders_html_document() {
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
    assert!(body.contains("<html"));
    assert!(body.contains("</html>"));
}

#[tokio::test]
async fn unknown_route_returns_404_and_renders_html_document() {
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
    assert!(body.contains("<html"));
    assert!(body.contains("</html>"));
}

#[tokio::test]
async fn frontend_routes_render_expected_template_names() {
    let renderer = Arc::new(RecordingRenderer::new());
    let app = spawn_app_with_renderer(renderer.clone()).await;

    let index_response = app.get("/").await;
    assert_eq!(reqwest::StatusCode::OK, index_response.status());

    let not_found_response = app.get("/does-not-exist").await;
    assert_eq!(reqwest::StatusCode::NOT_FOUND, not_found_response.status());

    assert_eq!(renderer.rendered(), vec!["index.html", "error.html"]);
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
