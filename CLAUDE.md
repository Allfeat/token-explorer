# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Allfeat Token Explorer is a Rust full-stack web application for exploring the Allfeat blockchain token economy. It combines Leptos 0.8 (SSR + WASM hydration), Axum for HTTP serving, and Subxt for Substrate blockchain integration.

**Note:** This application is under active development with potential breaking changes.

## Build Commands

```bash
# Development (watch mode with live reload)
cargo leptos watch

# Release build
cargo leptos build --release
# Outputs: target/server/release/token-app (binary), target/site (static assets)

# End-to-end tests
cargo leptos end-to-end

# Single E2E test
cd end2end && npx playwright test tests/<file>.spec.ts

# First-time E2E setup
cd end2end && npm install
```

## Lint and Format

```bash
# Leptos-aware formatter (preferred)
leptosfmt .

# Linting (strict)
cargo clippy --all-targets --all-features -D warnings

# Full quality gate before committing
leptosfmt . && cargo clippy --all-targets --all-features -D warnings && cargo check --all-features
```

## Architecture

### SSR + Hydration Model
- Binary compiles with `ssr` feature (Axum server, Subxt client)
- Library compiles with `hydrate` feature (WASM for browser)
- Use `#[cfg(feature = "ssr")]` / `#[cfg(feature = "hydrate")]` to separate server/client code

### Blockchain Integration
- `main.rs`: Initializes Substrate light client via `subxt::LightClient::relay_chain()`
- `state.rs`: Wraps client in `AppState` (Axum extractor context)
- `lib.rs`: Server functions (`#[server]`) call `get_chain_api()` for on-chain data
- Chain spec: `allfeat.json`; Metadata artifact: `artifacts/allfeat_metadata.scale`

### Key Directories
- `src/pages/`: Routed pages (overview, allocations, accounts)
- `src/components/`: Reusable UI components (Card, Header, Footer, etc.)
- `src/substrate/`: Substrate metadata and client type aliases
- `end2end/`: Playwright E2E tests (TypeScript)

## Conventions

### Code Style
- Imports: group std::*, external crates, then crate:: modules
- Avoid `unwrap()`/`expect()` in application code; use `?` operator
- Max 120 columns; run `leptosfmt` before commits
- Components as pure functions returning `impl IntoView`
- Colocate state with component using `signal()` or `RwSignal::new()`

### Async
- Use `tokio::spawn` for background work on SSR
- Never block in async (no `std::thread::sleep`)
- Use timeouts for network I/O (Subxt queries)

### Naming
- Functions/vars: `snake_case`
- Types: `CamelCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Component suffixes: descriptive (e.g., `MetricsCard`, `BlockStatus`)

### Styling
- Tailwind CSS 4 for all styling
- Dark theme: `bg-[#050505]`, `neutral-200` text
- Custom animations defined in `input.css`
