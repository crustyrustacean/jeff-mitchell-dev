set shell := ["bash", "-euo", "pipefail", "-c"]
set windows-shell := ["C:\\Program Files\\Git\\bin\\bash.exe", "-euo", "pipefail", "-c"]

taxus_git := "https://github.com/crustyrustacean/taxus.git"
taxus_rev := "3497ce97"

# List available recipes
default:
    @just --list

# Run full CI across both static site and backend server
ci: ci-site ci-server

# Install pinned taxus CLI and wasm target
install-taxus:
    rustup target add wasm32-unknown-unknown
    cargo install --git {{ taxus_git }} --rev {{ taxus_rev }} --locked --features typescript --bin taxus taxus-cli --force

# Install all local CI tools (taxus, cargo-nextest, cargo-deny)
install-tools: install-taxus
    @command -v cargo-nextest >/dev/null 2>&1 || cargo install cargo-nextest --locked
    @command -v cargo-deny >/dev/null 2>&1 || cargo install cargo-deny --locked

# Build static site via Taxus
site-build:
    taxus clean
    taxus build

# Verify required generated static site outputs exist and are non-empty
site-verify:
    #!/usr/bin/env bash
    set -euo pipefail
    required=(
      dist/index.html
      dist/404.html
      dist/feed.xml
      dist/sitemap.xml
      dist/robots.txt
      dist/css/main.css
      dist/wasm/client.js
      dist/wasm/client_bg.wasm
      dist/search_index.bin
      dist/static/ts/main.js
      dist/static/ts/theme.js
    )
    for f in "${required[@]}"; do
      if [ ! -s "$f" ]; then
        echo "Missing or empty build artifact: $f" >&2
        exit 1
      fi
    done

# Run static site CI (build + verify artifacts)
ci-site: site-build site-verify

# Check Rust formatting in server/
[working-directory: 'server']
server-fmt:
    cargo fmt --all -- --check

# Run Clippy with warnings denied in server/
[working-directory: 'server']
server-clippy:
    cargo clippy --all-targets --all-features --locked -- -D warnings

# Run unit, integration, and doc tests in server/
[working-directory: 'server']
server-test:
    cargo nextest run --all-targets --locked
    cargo test --doc --locked

# Run dependency advisory and license checks in server/
[working-directory: 'server']
server-deny:
    cargo deny check

# Run backend server CI (fmt, clippy, tests, cargo-deny)
ci-server: server-fmt server-clippy server-test server-deny

# Build the server Docker image locally
server-docker:
    docker build -t jeff-mitchell-dev-server:ci ./server

# Opt in to the tracked pre-push git hook
setup-hooks:
    git config core.hooksPath .githooks
    chmod +x .githooks/pre-push
