// src/startup.rs

// dependencies
use crate::configuration::Settings;
use crate::routes::{api, frontend};
use crate::template::TemplateRenderer;
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web, web::Data};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(
        configuration: Settings,
        template_renderer: Box<dyn TemplateRenderer>,
    ) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let server = run(listener, template_renderer).await?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(
    listener: TcpListener,
    template_renderer: Box<dyn TemplateRenderer>,
) -> Result<Server, anyhow::Error> {
    let template_renderer = Data::new(template_renderer);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(frontend::get_index_page))
            .service(web::scope("/api/v1").route("/health_check", web::get().to(api::health_check)))
            .service(
                Files::new("/static", "static")
                    .index_file("index.html")
                    .prefer_utf8(true),
            )
            .default_service(web::to(frontend::not_found))
            .app_data(template_renderer.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
