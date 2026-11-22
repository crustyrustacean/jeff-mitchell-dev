// tests/api/static_files.rs

use crate::helpers::{TempDir, spawn_app_with_static_dir};

fn create_static_fixtures() -> (TempDir, std::path::PathBuf) {
    let root = TempDir::new("static");
    let static_dir = root.path().join("static");
    std::fs::create_dir_all(&static_dir).expect("Failed to create fixture static dir.");
    std::fs::write(static_dir.join("test.css"), "body { color: black; }\n")
        .expect("Failed to write CSS fixture.");
    std::fs::write(static_dir.join("test.js"), "console.log('ok');\n")
        .expect("Failed to write JS fixture.");
    std::fs::write(root.path().join("secret.txt"), "top secret\n")
        .expect("Failed to write sibling secret fixture.");
    (root, static_dir)
}

#[tokio::test]
async fn css_and_js_assets_are_served_with_expected_content_types() {
    let (_root, static_dir) = create_static_fixtures();
    let app = spawn_app_with_static_dir(static_dir.to_string_lossy()).await;

    for (path, expected_prefixes, expected_body) in [
        (
            "/static/test.css",
            &["text/css"][..],
            "body { color: black; }\n",
        ),
        (
            "/static/test.js",
            &["text/javascript", "application/javascript"][..],
            "console.log('ok');\n",
        ),
        (
            "/static/test.js?v=5",
            &["text/javascript", "application/javascript"][..],
            "console.log('ok');\n",
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
        assert_eq!(body, expected_body, "Unexpected body for {path}");
    }
}

#[tokio::test]
async fn missing_static_assets_return_404_without_html_body() {
    let (_root, static_dir) = create_static_fixtures();
    let app = spawn_app_with_static_dir(static_dir.to_string_lossy()).await;

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
    let (_root, static_dir) = create_static_fixtures();
    let app = spawn_app_with_static_dir(static_dir.to_string_lossy()).await;

    let response = app.get("/static/").await;

    assert_eq!(reqwest::StatusCode::NOT_FOUND, response.status());
}

#[tokio::test]
async fn static_path_traversal_is_rejected() {
    let (_root, static_dir) = create_static_fixtures();
    let app = spawn_app_with_static_dir(static_dir.to_string_lossy()).await;

    for path in ["/static/../secret.txt", "/static/%2e%2e/secret.txt"] {
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
