// src/main.rs

// dependencies
use server::configuration::get_configuration;
use server::startup::Application;
use server::telemetry::{get_subscriber, init_subscriber};
use server::template::{TemplateRenderer, tera::TeraRenderer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("server".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let template_renderer: Box<dyn TemplateRenderer> = Box::new(TeraRenderer::new()?);
    let application = Application::build(configuration.clone(), template_renderer).await?;
    application.run_until_stopped().await?;

    Ok(())
}
