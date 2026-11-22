// src/template/tera.rs

use crate::template::{TemplateError, TemplateRenderer};
use anyhow::Context;
use tera::{Context as TeraContext, Tera};
pub struct TeraRenderer {
    engine: Tera,
}

impl TeraRenderer {
    pub fn new(templates_glob: &str) -> Result<Self, TemplateError> {
        let mut engine = Tera::default();
        engine
            .load_from_glob(templates_glob)
            .context("Unable to load the templates")?;

        Ok(Self { engine })
    }

    pub fn engine(&self) -> &Tera {
        &self.engine
    }
}

impl TemplateRenderer for TeraRenderer {
    fn render(
        &self,
        template_name: &str,
        context: &serde_json::Value,
    ) -> Result<String, TemplateError> {
        if !self.engine.contains_template(template_name) {
            return Err(TemplateError::NotFound(template_name.to_string()));
        }

        let template_context =
            TeraContext::from_serialize(context).context("Unable to build the page context")?;

        let output = self
            .engine
            .render(template_name, &template_context)
            .context("Tera engine failed to render template")?;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn render_returns_404_not_found_for_missing_template() {
        // Arrange
        let renderer = TeraRenderer::new("templates/**/*.html").unwrap();
        let template_name = "test";
        let context = serde_json::json!({});

        // Act
        let result = renderer.render(template_name, &context);

        // Assert
        assert!(matches!(result, Err(TemplateError::NotFound(_))));
    }

    #[test]
    fn render_returns_operation_error_for_non_object_context() {
        let renderer = TeraRenderer::new("templates/**/*.html").unwrap();
        let context = serde_json::json!(42);

        let result = renderer.render("index.html", &context);

        assert!(matches!(result, Err(TemplateError::Operation(_))));
    }
}
