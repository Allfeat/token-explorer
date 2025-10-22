# Allfeat Token Economy Explorer App

Important: This application is unstable and under active development. It has been released quickly for the Allfeat mainnet launch. Expect breaking changes, incomplete features, and rough edges.

Overview

- Stack: Rust, Leptos 0.8 (SSR + Hydration), Axum (SSR optional), WASM for client
- Tooling: cargo-leptos, leptosfmt, Playwright for end-to-end tests
- Features: Server-side rendering with client hydration; Substrate/Subxt integration

Prerequisites

- Rust toolchain (as specified in rust-toolchain.toml)
- Install cargo-leptos: cargo install cargo-leptos --locked
- For WASM: rustup target add wasm32-unknown-unknown

Getting Started (Development)

- Start dev server with live reload: cargo leptos watch
- App serves SSR + builds WASM and static assets into target/site

Build for Release

- Build: cargo leptos build --release
- Artifacts:
  - Server binary: target/server/release/token-app
  - Site assets: target/site
- To run without cargo-leptos, set env vars:
  - LEPTOS_OUTPUT_NAME=token-app
  - LEPTOS_SITE_ROOT=site
  - LEPTOS_SITE_PKG_DIR=pkg
  - LEPTOS_SITE_ADDR=127.0.0.1:3000
  - LEPTOS_RELOAD_PORT=3001

Project Structure

- src/app.rs: main application components
- src/pages: routed pages
- src/components: reusable UI components
- public/: static assets copied to site

Notes & Caveats

- This is a rapid release for mainnet launch; APIs, UI, and architecture may change without notice
- Error handling and UX polish are ongoing; please open issues for critical problems
- Substrate network assumptions and metadata may evolve with network upgrades

License

- Template portions are based on Leptos starter examples. See LICENSE for details.
