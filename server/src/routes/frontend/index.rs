// src/routes/frontend/index.rs

// src/routes/frontend/index.rs

use crate::{routes::frontend::index, template::TemplateRenderer};
use crate::utils::error_chain_fmt;
use actix_web::{HttpResponse, ResponseError, web::Data};
use serde::Serialize;
use tera::Context;

#[derive(thiserror::Error)]
pub enum IndexError {
    #[error("template rendering failed")]
    Template(#[from] tera::Error),
}

impl std::fmt::Debug for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for IndexError {
    fn error_response(&self) -> HttpResponse {
        tracing::error!(error = ?self, "index page failed");
        HttpResponse::InternalServerError().body("Something went wrong.")
    }
}

#[derive(Serialize)]
struct IndexContext<'a> {
    title: &'a str,
    header: &'a str,
    sub_header: &'a str,
}

pub async fn get_index_page(
    templates: Data<Box<dyn TemplateRenderer>>,
) -> Result<HttpResponse, actix_web::Error> {
    let index_context = IndexContext {
        title: "Home",
        header: "Server",
        sub_header: "jeff-mitchell.dev API",
    };

    let html = templates.render("index.html", &Context::from_serialize(&index_context).unwrap())?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
