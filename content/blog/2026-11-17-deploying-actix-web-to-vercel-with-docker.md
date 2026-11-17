+++
title = "Deploying Actix Web to Vercel with Docker"
date = 2026-11-17
description = "A small proof of concept: taking an Actix Web starter to Vercel with a multi-stage Docker build, explicit runtime configuration, and a little less unnecessary code."
categories = ["Projects"]
tags = ["rust", "actix-web", "docker", "vercel", "deployment"]
draft = true
aliases = ["/2026-11-17-deploying-actix-web-to-vercel-with-docker"]
+++

Earlier today, someone in the Actix Web Discord asked how to deploy an Actix Web application to Vercel.

I didn't have an answer ready, but I found Vercel's guide to [deploying Rust with Docker](https://vercel.com/kb/guide/deploy-rust-on-vercel-with-docker). That seemed like a good excuse to take my [Actix Web starter](https://github.com/crustyrustacean/actix-web-starter) and find out whether I could get a small example running.

The result is live at [actix-web-starter.vercel.app](https://actix-web-starter.vercel.app/api/v1/hello).

I wanted to get the starter running on Vercel before worrying about a production setup.

## The starter

My starter isn't the application from Vercel's article. It's a template I built in the style of *Zero to Production in Rust*, with its own configuration and application structure. I used the article for the deployment approach.

The starter is a versioned HTTP API with YAML configuration, environment-variable overrides, structured logging, and a health endpoint. There is no database or external service to set up.

For this experiment, I added a second endpoint:

```rust
use actix_web::web::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    message: String,
}

pub async fn hello() -> Json<Message> {
    Json(Message {
        message: "Hello from Rust on Vercel".to_string(),
    })
}
```

The handler lives in `src/routes/hello.rs`, is re-exported by the routes module, and is registered alongside the health check:

```rust
web::scope("/api/v1")
    .route("/health_check", web::get().to(health_check))
    .route("/hello", web::get().to(hello))
```

The greeting gives me something recognisable to check after deployment.

## The Dockerfile

Here is the Dockerfile used for version `0.3.2` of the starter:

```dockerfile
FROM lukemathwalker/cargo-chef:latest-rust-1-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked --bin actix-web-starter

# Match the builder's Debian release for compatibility with its linked libraries.
# A shell is needed to map Vercel's PORT to the application's configuration key.
FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libgcc-s1 libstdc++6 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /app/target/release/actix-web-starter /usr/local/bin/server
COPY configuration/ ./configuration/
ENV APP_ENVIRONMENT=production \
    APP_APPLICATION__HOST=0.0.0.0 \
    PORT=80
USER 65532:65532
# exec lets Actix receive termination signals directly, rather than through sh.
ENTRYPOINT ["/bin/sh", "-c", "export APP_APPLICATION__PORT=\"${PORT:-80}\"; exec /usr/local/bin/server"]
```

### Cache dependency builds

[Cargo Chef](https://github.com/LukeMathWalker/cargo-chef) separates dependency compilation from application compilation. The planner produces a recipe, and the build stage uses that recipe before copying in the application source.

Docker can reuse that layer when the recipe hasn't changed, so a source edit needn't trigger a full dependency rebuild.

The `--locked` flags make Cargo use the committed `Cargo.lock` without updating it. The final build also names the binary explicitly: `actix-web-starter`.

My first Dockerfile still referred to the guide's example executable in its final `COPY` instruction. This project builds `actix-web-starter`, so I had to change the path.

The release profile also enables stripping and thin link-time optimization:

```toml
[profile.release]
strip = true
lto = "thin"
```

Those are build choices, not requirements imposed by Actix Web or Vercel.

### Match the build and runtime images

The builder uses Debian Bookworm, and so does the runtime image.

A release binary built in a Linux container isn't automatically independent of its operating-system libraries. Matching the Debian releases avoids introducing an unnecessary compatibility mismatch between the environment that links the binary and the environment that runs it.

The final image contains the executable, configuration files, and runtime packages rather than the Rust toolchain. It runs under a numeric non-root UID and GID.

The builder tag still floats within Rust 1.x. For a more tightly controlled deployment, I'd pin image versions or digests as well. A locked Cargo dependency graph does not pin the entire container build environment.

## Map the port setting

The starter doesn't read a bare `PORT` environment variable. Its configuration loader accepts nested application settings through variables such as:

```text
APP_APPLICATION__HOST
APP_APPLICATION__PORT
```

That means this instruction alone would not change the server's listening port:

```dockerfile
ENV PORT=80
```

The entrypoint bridges the two conventions:

```sh
export APP_APPLICATION__PORT="${PORT:-80}"
exec /usr/local/bin/server
```

The shell expands `PORT` when the container starts and exports the name the application actually understands. The fallback is port 80.

I originally had a distroless runtime in the Dockerfile. I switched to Debian slim so this version could use a shell for the translation without changing the Rust configuration loader. Reading `PORT` directly in the application would be another reasonable approach.

The `exec` is intentional. It replaces the shell with the server process, allowing Actix to receive termination signals directly. In the local container check, stopping the container produced Actix's graceful-shutdown log messages and an exit code of zero.

The host setting is just as important. The container uses `0.0.0.0` so the application listens on all IPv4 interfaces, rather than only on its own loopback interface.

## Include the YAML files

The starter loads configuration relative to its current working directory:

```text
configuration/base.yaml
configuration/production.yaml
```

Copying only the executable would leave it without files it expects at startup. The Dockerfile sets `/app` as the working directory and copies `configuration/` underneath it.

The production environment is selected with `APP_ENVIRONMENT=production`. Environment overrides are applied after the YAML settings.

The `.dockerignore` is small:

```text
.gitignore
target
tests
```

In particular, the local `target/` directory stays out of the build context. The image builds its own Linux executable; it doesn't need my Windows build artifacts.

## Remove an unused setting

This exercise also exposed something I had added before I needed it: an application base URL.

Neither endpoint used it, but the configuration type still required it. Removing its wrapper type and local YAML default hadn't removed the underlying requirement. The application still loaded the value and passed it into application data.

The first container version supplied a placeholder just to satisfy that requirement. That worked, but it was solving a problem I'd created myself.

So I removed the setting entirely: the configuration field, the startup plumbing, the Docker default, and the documentation telling people to provide it.

If a feature needs a public URL later, I can add the setting then.

I have a habit of adding things before they're warranted. This setting was one of them.

## Check the container locally

From the repository root:

```sh
docker build -t actix-vercel .
docker run --rm -p 8080:80 actix-vercel
```

Then, in another terminal:

```sh
curl -i http://localhost:8080/api/v1/health_check
curl -i http://localhost:8080/api/v1/hello
```

On Windows PowerShell, use `curl.exe` if `curl` resolves to an alias rather than the curl executable.

I also checked that the server followed a different runtime port:

```sh
docker run --rm -e PORT=9090 -p 8080:9090 actix-vercel
```

The same requests to `localhost:8080` should work. This checks the port translation rather than just the default.

Both integration tests passed with local and production configuration, without a base URL override. The `0.3.2` Docker image also built successfully and returned the expected health and greeting responses.

## Check the deployed application

With the image working locally, the next step was deploying the project using [Vercel's Rust-with-Docker guide](https://vercel.com/kb/guide/deploy-rust-on-vercel-with-docker). Refer to that guide for the current platform setup and availability requirements; this experiment isn't a claim that every Vercel project accepts an arbitrary Dockerfile without configuration.

<!-- Before publishing: add the exact Vercel project setup and deployment steps used, including any required access or settings. These were not recorded in the implementation session. -->

The deployed application responded to:

```sh
curl -i https://actix-web-starter.vercel.app/api/v1/health_check
curl -i https://actix-web-starter.vercel.app/api/v1/hello
```

The health endpoint returned `200 OK` with an empty body. The hello endpoint returned `200 OK` with `Content-Type: application/json` and:

```json
{"message":"Hello from Rust on Vercel"}
```

The root URL returned `404 Not Found`. That's expected: the starter doesn't register a route at `/`. An API doesn't need a home page to be working.

## What this proves

The proof of concept answers the original question for this application: it is running and responding on Vercel.

It doesn't establish performance under load, cold-start behaviour, platform limits, or suitability for a particular production workload. There is no database, persistent storage, or background-job setup in this example.

For this starter, the changes came down to the executable path, runtime files, listening address, and port mapping. Removing the unused setting also meant one less environment variable to supply.

The code for this version is in [the starter repository at commit `451fad4`](https://github.com/crustyrustacean/actix-web-starter/tree/451fad4). The [live greeting endpoint](https://actix-web-starter.vercel.app/api/v1/hello) is the shortest demonstration.
