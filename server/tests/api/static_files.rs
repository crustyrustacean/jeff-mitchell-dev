// tests/api/static_files.rs

use crate::helpers::spawn_app;

#[tokio::test]
async fn css_and_js_assets_are_served_with_expected_content_types() {
    let app = spawn_app().await;

    for (path, expected_prefixes, expect_non_empty) in [
        ("/static/screen.css", &["text/css"][..], false),
        (
            "/static/scripts.js",
            &["text/javascript", "application/javascript"][..],
            false,
        ),
        (
            "/static/datastar.js",
            &["text/javascript", "application/javascript"][..],
            true,
        ),
        (
            "/static/datastar.js?v=5",
            &["text/javascript", "application/javascript"][..],
            true,
        ),
    ] {
        let response = app.get(path).await;

        assert_eq!(
            reqwest::StatusCode::OK,
            response.status(),
            "Unexpected status for {path}"
        );
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(
            expected_prefixes
                .iter()
                .any(|prefix| content_type.starts_with(prefix)),
            "Unexpected content-type for {path}: {content_type}"
        );
        let body = response.text().await.expect("Failed to read body.");
        if expect_non_empty {
            assert!(!body.is_empty(), "Expected non-empty body for {path}");
        }
    }
}

#[tokio::test]
async fn missing_static_assets_return_404_without_html_body() {
    let app = spawn_app().await;

    for path in ["/static/does-not-exist.css", "/static/does-not-exist.js"] {
        let response = app.get(path).await;

        assert_eq!(
            reqwest::StatusCode::NOT_FOUND,
            response.status(),
            "Unexpected status for {path}"
        );
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(
            !content_type.starts_with("text/html"),
            "Missing static asset {path} should not return text/html"
        );
    }
}

#[tokio::test]
async fn static_directory_listing_is_not_exposed() {
    let app = spawn_app().await;

    let response = app.get("/static/").await;

    assert_eq!(reqwest::StatusCode::NOT_FOUND, response.status());
}

#[tokio::test]
async fn static_path_traversal_is_rejected() {
    let app = spawn_app().await;

    for path in ["/static/../Cargo.toml", "/static/%2e%2e/Cargo.toml"] {
        let response = app.get(path).await;

        assert!(
            matches!(
                response.status(),
                reqwest::StatusCode::BAD_REQUEST | reqwest::StatusCode::NOT_FOUND
            ),
            "Expected 400 or 404 for {path}, got {}",
            response.status()
        );
    }
}
