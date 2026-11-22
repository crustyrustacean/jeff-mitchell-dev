// src/routes/frontend/index.rs

use crate::template::TemplateRenderer;
use actix_web::{HttpResponse, web::Data};
use serde::Serialize;

#[derive(Serialize)]
struct IndexContext<'a> {
    title: &'a str,
    header: &'a str,
    sub_header: &'a str,
}

pub async fn get_index_page(
    templates: Data<dyn TemplateRenderer>,
) -> Result<HttpResponse, actix_web::Error> {
    let index_context = IndexContext {
        title: "Home",
        header: "Server",
        sub_header: "jeff-mitchell.dev API",
    };

    let json_index_context = serde_json::to_value(&index_context)?;

    let html = templates.render("index.html", &json_index_context)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
