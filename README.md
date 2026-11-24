# jeff-mitchell.dev

Source repository for [jeff-mitchell.dev](https://jeff-mitchell.dev) — a developer blog and its companion Rust backend server.

The repository is organized as a monorepo with two independently deployed parts:

1. **Static Site (root):** Built with [Taxus](https://github.com/crustyrustacean/taxus) and deployed to **Cloudflare Pages**.
2. **Backend Server (`server/`):** A Rust web server powering the site's control panel and API, built with **Actix Web**, **Tera**, and **Datastar**, and deployed to **Fly.io**.

---

## Repository Structure

```text
.
├── content/                 # Markdown content for the static site
│   ├── blog/                # Blog posts and blog index
│   └── {about,archive,blogroll,contact,privacy,projects}/
├── templates/               # Tera templates for the static site
├── styles/                  # SCSS stylesheets (light & dark themes + syntax highlighting)
├── scripts/                 # Client-side TypeScript (theme persistence & toggle)
├── static/                  # Static assets (favicon, images)
├── site.toml                # Taxus site configuration
├── tsconfig.json            # TypeScript compiler configuration
├── wrangler.toml            # Cloudflare Pages configuration
├── server/                  # Rust control panel & API server
│   ├── src/                 # Actix Web application, routes, config, telemetry, Tera wrapper
│   ├── templates/           # Server-rendered Tera HTML templates
│   ├── static/              # Server static assets (screen.css, datastar.js, scripts.js)
│   ├── configuration/       # Layered TOML config (base, local, production)
│   ├── tests/               # Integration tests
│   ├── Cargo.toml
│   ├── deny.toml            # cargo-deny advisory, license, ban, and source policy
│   ├── Dockerfile           # Multi-stage cargo-chef production image
│   └── fly.toml             # Fly.io deployment configuration
├── .githooks/               # Opt-in local git hooks (e.g. pre-push CI)
├── justfile                 # Unified local & CI task runner
└── .github/workflows/       # Unified gated CI/CD pipeline (ci-cd.yml)
```

---

## Static Site (Frontend)

### How It Works

* **Generator:** [Taxus](https://github.com/crustyrustacean/taxus) reads Markdown from `content/`, templates from `templates/`, SCSS from `styles/`, static files from `static/`, and TypeScript from `scripts/`, outputting the built site to `dist/` (configured in `site.toml`).
* **Interactive Search:** Uses a WebAssembly island (`SearchBox`) hydrated on load from `/wasm/client.js` and `/wasm/client_bg.wasm`.
* **Theming:** Supports light and dark themes via `data-theme` on `<html>`. An inline bootstrap script in `templates/base.html` applies the saved or OS-preferred theme before first paint, and `scripts/theme.ts` handles the toggle button and `localStorage` persistence.
* **Syntax Highlighting:** Enabled in `site.toml` with the `hl-` class prefix, styled via `styles/_highlight-light.scss` and `styles/_highlight-dark.scss`.
* **Feeds & Analytics:** Generates an RSS feed at `/feed.xml` and uses Plausible for privacy-friendly analytics.

### Building Locally

Prerequisites: [just](https://github.com/casey/just) and a Rust toolchain (plus `wasm32-unknown-unknown` and pinned `taxus-cli`). You can install all local CI tools from the repo root:

```bash
just install-tools
just ci-site
```

To run the static build manually:

```bash
rustup target add wasm32-unknown-unknown
cargo install --git https://github.com/crustyrustacean/taxus.git --rev 3497ce97 --locked --features typescript --bin taxus taxus-cli
taxus build
```

The generated site is written to `dist/` and verified by `just site-verify`. To bump the Taxus version used locally and in GitHub Actions, update `taxus_rev` in `justfile`.

---

## Backend Server (`server/`)

### How It Works

* **Stack:** Rust (edition 2024) using `actix-web`, `actix-files`, `tera`, `datastar`, `confik`, and structured logging via `tracing` + `tracing-bunyan-formatter`.
* **Routes:**
  * `GET /` — Renders the server home page (`templates/index.html`).
  * `GET /api/v1/health_check` — Health check endpoint (also used by Fly.io HTTP checks).
  * `GET /static/*` — Serves static assets from `server/static/` without directory listing (missing assets return a non-HTML `404`).
  * Fallback — Unknown routes render the HTML `404` error page (`templates/error.html`).
* **Styling:** `server/static/screen.css` mirrors the frontend's dark color palette, typography, and layout using native modern CSS (`@layer`, CSS nesting, logical properties, `color-scheme: dark`).
* **Configuration:** Layered TOML files in `server/configuration/`:
  * `base.toml` sets defaults (`port = 8000`, `host = "0.0.0.0"`, `static_dir = "static"`, `templates_glob = "templates/**/*.html"`).
  * `local.toml` (`host = "127.0.0.1"`) or `production.toml` (`host = "0.0.0.0"`) is selected via `APP_ENVIRONMENT` (defaults to `local`).
  * Any setting can be overridden with environment variables prefixed by `APP_` using `__` as the nesting separator (e.g. `APP_APPLICATION__PORT=5001`).

### Running & Testing Locally

Run commands from the `server/` directory so relative paths (`configuration/`, `templates/`, `static/`) resolve properly:

```bash
cd server
cargo run
```

To run the server CI suite (`rustfmt`, `clippy -D warnings`, `cargo-nextest`, doctests, and `cargo-deny`) from the repo root:

```bash
just ci-server
```

Or run `just ci` to run both `ci-site` and `ci-server`, `just server-docker` to validate the Docker image build locally, and `just setup-hooks` to opt in to `.githooks/pre-push` (which runs affected CI checks before `git push`).

To run tests manually inside `server/`:

```bash
cd server
cargo test
```

---

## Deployment

Both CI and deployment are unified in `.github/workflows/ci-cd.yml` (mirroring the `justfile` recipes):

* **Change Detection (`changes`):** Runs `ci-site` on static site changes and `ci-server` on `server/**` changes (or both when `justfile`, `ci-cd.yml`, or `workflow_dispatch` runs).
* **Gated CI (`ci-gate`):** Required upstream gate for all deployments. `deploy-site` and `deploy-server` only run on `trunk` after `ci-gate` succeeds.
* **Static Site (`deploy-site`):** Downloads the validated `dist/` artifact from `ci-site` and deploys to Cloudflare Pages (`jeff-mitchell-dev`) via Wrangler.
* **Backend Server (`deploy-server`):** Deploys `server/` to Fly.io via `flyctl deploy --remote-only` (after `ci-server` passes tests, formatting, Clippy, `cargo-deny`, and a Docker build check) and smoke-checks `https://api.jeff-mitchell.dev/api/v1/health_check`.

---

## License

This repository uses a split license depending on the type of asset:

* **Code** (`server/`, `templates/`, `styles/`, `scripts/`, configuration, and workflows): Dual-licensed under either the [MIT License](LICENSE-MIT) or the [Apache License, Version 2.0](LICENSE-APACHE), at your option.
* **Written Content** (`content/`): Licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](LICENSE-CONTENT).
* **Media** (`static/`): Copyright &copy; 2026 Jeffery D. Mitchell. All Rights Reserved.

