// src/bin/main.rs

// dependencies
use server::configuration::get_configuration;
use server::startup::Application;
use server::telemetry::{get_subscriber, init_subscriber};
use server::template::{TemplateRenderer, tera::TeraRenderer};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("server".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration()?;
    let template_renderer: Arc<dyn TemplateRenderer> =
        Arc::new(TeraRenderer::new(&configuration.application.templates_glob)?);
    let application = Application::build(configuration, template_renderer).await?;
    application.run_until_stopped().await?;

    Ok(())
}
