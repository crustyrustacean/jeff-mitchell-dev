// src/routes/frontend.rs

use crate::template::TemplateRenderer;
use actix_web::{HttpResponse, web::Data};
use serde::Serialize;
// module declarations
pub mod index;

// re-exports
pub use index::*;

#[derive(Serialize)]
struct NotFoundContext {
    title: &'static str,
    header: &'static str,
    sub_header: &'static str,
    status_code: &'static str,
    message: &'static str,
}

pub async fn not_found(
    templates: Data<Box<dyn TemplateRenderer>>,
) -> Result<HttpResponse, actix_web::Error> {
    let not_found_context = NotFoundContext {
        title: "Not Found",
        header: "jeff-mitchell.dev",
        sub_header: "API",
        status_code: "404",
        message: "The page you are looking for does not exist.",
    };
    let json_ctx = serde_json::to_value(&not_found_context)?;
    let html = templates.render("error.html", &json_ctx)?;
    Ok(actix_web::HttpResponse::NotFound()
        .content_type("text/html")
        .body(html))
}
