// tests/api/helpers.rs

// dependencies
use server::configuration::get_configuration;
use server::startup::Application;
use server::telemetry::{get_subscriber, init_subscriber};
use server::template::{TemplateRenderer, tera::TeraRenderer};
use std::sync::{Arc, LazyLock};

// Ensure that the `tracing` stack is only initialised once.
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    #[allow(dead_code)]
    pub port: u16,
    api_client: reqwest::Client,
}

impl TestApp {
    pub async fn get(&self, path: &str) -> reqwest::Response {
        self.api_client
            .get(format!("{}{path}", self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let template_renderer: Arc<dyn TemplateRenderer> = Arc::new(
        TeraRenderer::new(&configuration.application.templates_glob)
            .expect("Failed to build the template renderer."),
    );
    spawn_app_with_renderer(template_renderer).await
}

pub async fn spawn_app_with_renderer(template_renderer: Arc<dyn TemplateRenderer>) -> TestApp {
    LazyLock::force(&TRACING);

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.application.port = 0;
        c
    };

    // Launch the application as a background task
    let application = Application::build(configuration, template_renderer)
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    drop(tokio::spawn(application.run_until_stopped()));

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        api_client: client,
    }
}
