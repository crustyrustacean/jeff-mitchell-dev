+++
title = "Deploy Actix Web on Vercel with Docker"
date = 2026-11-17
description = "Build a Rust application with Actix Web and Docker, then deploy it to Vercel Functions."
categories = ["Advanced Guides"]
tags = ["rust", "web-frameworks", "actix-web", "docker", "vercel"]
draft = false
aliases = ["/2026-11-17-deploying-actix-web-to-vercel-with-docker"]
+++

Someone in the Actix Web Discord asked how to deploy an application to Vercel. I found Vercel's guide to [deploying Rust on Vercel with Docker](https://vercel.com/kb/guide/deploy-rust-on-vercel-with-docker), but it uses Axum.

Here's the same approach with Actix Web instead.

Vercel has a native Rust runtime in beta, but a container lets us run an Actix Web HTTP server with control over its Linux dependencies. Vercel builds the image and serves it from a Function that scales with traffic.

## Prerequisites

- A [Vercel account](https://vercel.com)
- Docker Desktop or a running Docker daemon
- [Vercel CLI](https://vercel.com/docs/cli) (`npm install -g vercel`)

## How it works

The project is one Actix Web application with three parts:

- `src/main.rs` defines `/` and `/health`, and binds to `0.0.0.0` on the port in `$PORT`.
- `Cargo.toml` declares Actix Web and Serde, and `Cargo.lock` pins their resolved versions.
- `Dockerfile` compiles a release binary and copies it into a minimal final stage. Vercel detects it at the project root, so no `vercel.json` is needed.

## 1. Initialize the project

Create the project and add the dependencies:

```bash
cargo new vercel-docker-rust
cd vercel-docker-rust
cargo add actix-web
cargo add serde --features derive
```

The resulting `Cargo.toml` should look like this. The release profile is optional, but keeps the shipped binary small:

```toml
[package]
name = "vercel-docker-rust"
version = "0.1.0"
edition = "2024"

[dependencies]
actix-web = "4"
serde = { version = "1", features = ["derive"] }

[profile.release]
strip = true
lto = "thin"
```

Now update `src/main.rs`:

```rust
use actix_web::{App, HttpServer, web};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: &'static str,
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn hello() -> web::Json<Message> {
    web::Json(Message {
        message: "Hello from Rust on Vercel",
    })
}

async fn health() -> web::Json<Health> {
    web::Json(Health { status: "ok" })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(80);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/health", web::get().to(health))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
```

The server must bind to `0.0.0.0`, not `127.0.0.1`, and read `PORT`. The fallback to port 80 matches Vercel's default container port. Actix Web handles `SIGTERM` for us, so we don't need the explicit shutdown handler from the Axum example.

## 2. Add a Dockerfile

Create a `Dockerfile` in the project root:

```dockerfile
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /app/target/release/vercel-docker-rust /usr/local/bin/server
ENV PORT=80
ENTRYPOINT ["/usr/local/bin/server"]
```

Compiling dependencies from the Cargo Chef recipe before copying the application code lets Docker reuse the dependency layer. The final image contains the binary and its C libraries, but not Cargo or the Rust compiler. It also runs as a non-root user.

The path in the `COPY` instruction must match the package name in `Cargo.toml`.

## 3. Run locally

Run the project through Vercel's local container environment:

```bash
vercel dev -L
```

Use the URL printed by the CLI, usually `http://localhost:3000`:

```bash
curl http://localhost:3000/
curl http://localhost:3000/health
```

You should get the greeting and a JSON response with `"status":"ok"`.

## 4. Deploy to Vercel

Authenticate and create a production deployment:

```bash
vercel login
vercel deploy --prod
```

For Git-based deployments, push the directory to a repository, import it in Vercel, and keep the `Dockerfile` at the configured project root.

On a new project, Vercel detects the root `Dockerfile` and selects the Container preset. If you deploy into an existing project that uses another framework, run this first:

```bash
vercel project update --framework container --yes
```

You can then add routes, middleware, authentication, and external data services. Use external storage for persistent data: Vercel Functions don't provide a persistent container file system.

If you're looking for a more complete starting point, take a look at my [actix-web-starter](https://github.com/crustyrustacean/actix-web-starter). It's my take on a solid Actix Web template, founded on the principles from [*Zero to Production in Rust*](https://www.zero2prod.com/).
