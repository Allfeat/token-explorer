CRUSH.md

Repository quick facts
- Language/stack: Rust (Leptos 0.8, Axum SSR optional, WASM hydrate), Playwright end-to-end tests (TypeScript)
- Tooling: cargo-leptos, leptosfmt, rust-toolchain via rustup, optional Nix devShell via flake.nix
- Env: default features = ssr for bin; lib compiles with hydrate for WASM

Build/run
- Dev server (watch, SSR + WASM): cargo leptos watch
- Build release: cargo leptos build --release
- Run server binary after build: target/server/release/token-app (serves target/site)
- Set env (if running manually): LEPTOS_OUTPUT_NAME=token-app, LEPTOS_SITE_ROOT=site, LEPTOS_SITE_PKG_DIR=pkg, LEPTOS_SITE_ADDR=127.0.0.1:3000, LEPTOS_RELOAD_PORT=3001

Tests
- End-to-end (uses Playwright): cargo leptos end-to-end
- E2E release mode: cargo leptos end-to-end --release
- Run a single E2E test: cd end2end && npx playwright test tests/<file>.spec.ts
- Run headed / specific project: npx playwright test tests/<file>.spec.ts --headed --project=chromium
- Install E2E deps first: cd end2end && npm i

Lint/format/typecheck
- Rust format (Leptos-aware): leptosfmt .
- Rust format alternative: cargo fmt --all
- Rust lint: cargo clippy --all-targets --all-features -D warnings
- Rust build check: cargo check --all-features
- Playwright types (optional): cd end2end && npx tsc -p tsconfig.json --noEmit

Conventions and style
- Imports: group std::*, external crates, then crate:: (self/super) modules; prefer absolute crate paths; keep unused imports clean (clippy -D warnings)
- Formatting: run leptosfmt or cargo fmt before commits; 120 cols max preferred; keep argument lists and builder chains multiline when long
- Types: favor explicit types in public APIs; use Option/Result idiomatically; prefer &str over String for params; avoid unwrap/expect in app code (use ? and map_err)
- Errors: bubble with anyhow/thiserror if introduced; otherwise use Result<_, E> and ?; log context rather than panicking on recoverable paths
- Naming: snake_case for functions/vars, CamelCase for types, SCREAMING_SNAKE_CASE for consts; modules are snake_case; keep components suffixed clearly (e.g., FooCard)
- Leptos: keep components as pure functions returning impl IntoView; colocate state with component when local; use leptos_router for navigation; avoid blocking ops in SSR handlers
- Async: prefer tokio::spawn for background work on SSR; never block in async (no std::thread::sleep); prefer timeouts when doing network IO (subxt)
- WASM: keep wasm-bindgen and web-sys feature-gated under `hydrate`; avoid direct panics (console_error_panic_hook is enabled under hydrate)

Working with features
- Bin builds with features=["ssr"], lib with ["hydrate"]; use cfg(feature = "ssr") / cfg(feature = "hydrate") gates to separate server vs client code paths

Cursor/Copilot rules
- No Cursor or Copilot instruction files detected (no .cursor/rules, no .cursorrules, no .github/copilot-instructions.md). If added later, mirror key rules here.

Helpful one-liners
- Single test (UI E2E): npx playwright test tests/example.spec.ts
- Clean build artifacts: cargo clean && rm -rf target/site
- Full quality gate: leptosfmt . && cargo clippy --all-targets --all-features -D warnings && cargo check --all-features && (cd end2end && npx tsc -p tsconfig.json --noEmit)