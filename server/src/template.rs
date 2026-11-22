// src/template.rs

use crate::utils::error_chain_fmt;
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;

pub mod tera;

#[derive(Error)]
pub enum TemplateError {
    #[error("template not found: {0}")]
    NotFound(String),

    #[error("A failure was encountered while trying to render the template.")]
    Operation(#[from] anyhow::Error),
}

impl std::fmt::Debug for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for TemplateError {
    fn status_code(&self) -> StatusCode {
        match self {
            TemplateError::NotFound(_) => StatusCode::NOT_FOUND,
            TemplateError::Operation(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        tracing::error!(error = ?self, "template rendering error");
        match self {
            TemplateError::NotFound(_) => HttpResponse::NotFound().body("Not Found"),
            TemplateError::Operation(_) => {
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
    }
}

pub trait TemplateRenderer: Send + Sync {
    fn render(
        &self,
        template_name: &str,
        context: &serde_json::Value,
    ) -> Result<String, TemplateError>;
}
