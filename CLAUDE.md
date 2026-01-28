# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Allfeat Token Economy Explorer - a full-stack Rust web app for viewing token economics on the Allfeat blockchain. Uses Leptos 0.8 with SSR + client hydration via WASM.

**Tech Stack**: Leptos (Rust web framework), Axum (HTTP server), Subxt (Substrate light client), Tailwind CSS 4, Playwright (E2E tests)

## Build & Development Commands

```bash
# Development (live reload on 127.0.0.1:3000)
cargo leptos watch

# Release build
cargo leptos build --release
# Output: target/server/release/token-app (binary), target/site/ (assets)

# End-to-end tests (from end2end/ directory)
npx playwright test

# Format Leptos view macros
leptosfmt <file>
```

**Prerequisites**: `cargo install cargo-leptos --locked`, `rustup target add wasm32-unknown-unknown`

## Architecture

### SSR + Hydration Pattern
- Server renders initial HTML via Axum + Leptos
- Client hydrates with WASM for interactivity
- Server functions in `src/lib.rs` execute on server, callable from client

### Key Modules
- `src/lib.rs` - Server functions: `get_block_number_stream()`, `get_allocations()`, `get_balance_of(id)`, `get_total_issuance()`, `get_circulating_supply()`
- `src/main.rs` - Axum server setup, light client initialization
- `src/app.rs` - Root component, routing (`/`, `/allocations`, `/account/:id`)
- `src/state.rs` - `AppState` struct holding Leptos options + Subxt client
- `src/substrate/mod.rs` - Subxt-generated chain bindings, light client init

### Blockchain Integration
- Uses Subxt light client (not full node RPC)
- Chain spec fetched dynamically via environment variables:
  - `RPC_URL` - WebSocket URL to fetch chain spec (e.g., `wss://rpc.allfeat.network`)
  - `BOOTNODES` - Comma-separated list of bootnode addresses
- Light client is critical - app exits on init failure
- Treasury account hardcoded: `5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z`

### Feature Flags
- `ssr` (default) - Server-side rendering with Axum, Subxt, tracing
- `hydrate` - Client WASM bundle with hydration

### Data Types (src/lib.rs)
- `Balances` - free, reserved, frozen amounts
- `EnvelopeAllocation` - Token envelope config (cap, vesting, cliff, etc.)
- `Allocation` - User's allocation within an envelope

## Code Style

- Use `leptosfmt` for Leptos view macro formatting
- Components in `src/components/`, pages in `src/pages/`
- Server functions use `expect_context::<AppState>()` to access state
