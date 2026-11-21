// src/startup.rs

// dependencies
use crate::configuration::Settings;
use crate::routes::health_check;
use actix_files::{Files, NamedFile};
use actix_web::dev::Server;
use actix_web::http::StatusCode;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;
use std::path::PathBuf;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let server = run(listener).await?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn not_found(req: HttpRequest, path: web::Data<PathBuf>) -> HttpResponse {
    match NamedFile::open(path.as_ref()) {
        Ok(file) => file
            .customize()
            .with_status(StatusCode::NOT_FOUND)
            .respond_to(&req)
            .map_into_boxed_body(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn run(listener: TcpListener) -> Result<Server, anyhow::Error> {
    let static_dir = if std::path::Path::new("./dist").is_dir() {
        "./dist"
    } else {
        "../dist"
    };
    let not_found_path = web::Data::new(PathBuf::from(static_dir).join("404.html"));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(not_found_path.clone())
            .service(web::scope("/api/v1").route("/health_check", web::get().to(health_check)))
            .service(
                Files::new("/", static_dir)
                    .index_file("index.html")
                    .prefer_utf8(true)
                    .default_handler(web::to(not_found)),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
