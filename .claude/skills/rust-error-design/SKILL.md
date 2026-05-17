---
name: rust-error-design
description: Design or refactor Rust error types — choose between thiserror (libraries) and anyhow (applications), eliminate Box<dyn Error> and Result<T, String>, and wire up `?` with proper context. Use when the user asks to design an error enum, refactor error handling, fix `Box<dyn Error>` leaking through a public API, or convert `.map_err(|e| format!(...))` chains. Do NOT use for general Rust review (use rust-review).
---

# Rust error design

Convert ad-hoc error handling to typed errors. The wrong shape leaks through public APIs and infects every caller, so this refactor is usually worth doing early.

## Decision tree

```
Is this code a library / reusable module (has consumers who may need to
match on error variants, retry, or wrap)?

├── YES → thiserror. Define a concrete error enum per module.
│         Use #[from] for upstream errors so `?` composes cleanly.
│         Never return Box<dyn Error> or Result<T, String>.
│
└── NO  → It's a binary / application top layer.
          anyhow::Result<T> with .context("...") at each layer.
          Use anyhow::bail!() / ensure!() for ad-hoc errors.
          Match on root_cause() / downcast_ref() only at boundaries.
```

A single crate often needs both: `thiserror` enums in library modules, `anyhow::Result` in `main.rs` / CLI glue.

## thiserror template

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("failed to build vault client settings")]
    Settings(#[source] vaultrs::client::VaultClientSettingsBuilderError),

    #[error("failed to create vault client")]
    Client(#[source] vaultrs::error::ClientError),

    #[error("initialization returned no keys")]
    NoKeys,

    #[error("vault status request failed")]
    Status(#[from] vaultrs::error::ClientError),
}
```

Rules:
- One enum per module / per logical surface. Don't make a megaenum spanning the whole crate.
- Each variant gets a `#[error("...")]` message that reads as a sentence fragment (lowercase, no trailing period, no "Error:" prefix — `Display` adds context).
- Wrap source errors with `#[source]` (manual) or `#[from]` (auto-converts via `?`). Use `#[from]` only when the variant has no other fields and conversion is unambiguous.
- Don't include sensitive data (tokens, keys) in `#[error("...")]` format strings.

## anyhow template

```rust
use anyhow::{Context, Result};

async fn run() -> Result<()> {
    let vault = vault::client()
        .await
        .context("starting vault client")?;

    let k8s = k8s::client()
        .await
        .context("starting kubernetes client")?;

    ensure(&vault, &k8s)
        .await
        .context("ensuring vault is ready")?;

    Ok(())
}
```

Rules:
- `.context("...")` at every boundary you cross. The chain becomes the error message — read top-down it tells the story.
- Context strings are lowercase verb phrases ("starting vault client"), not sentences.
- `anyhow::bail!("...")` to return an ad-hoc error; `anyhow::ensure!(cond, "...")` for precondition checks.

## Refactor patterns

### Pattern: stringly-typed → typed

Before:
```rust
.map_err(|e| format!("Failed to build Vault client settings: {:?}", e))?
```

After (library):
```rust
.map_err(VaultError::Settings)?    // typed variant, source preserved
```

After (application):
```rust
.context("building vault client settings")?  // anyhow chain
```

### Pattern: `Box<dyn Error>` in public API → typed enum

If the function is `pub` in a `lib.rs` or `pub mod`, the boxed-error return is a Major issue. Callers can't match. Replace with a `thiserror` enum.

If the function is private and only `main` consumes it, anyhow is fine — but consider promoting to a typed error if the function is non-trivial.

### Pattern: log-and-return → log-once-at-top

Before (every layer does this):
```rust
match foo().await {
    Ok(v) => Ok(v),
    Err(e) => {
        error!("Failed to foo: {:?}", e);   // <-- log
        Err(e)                                // <-- and propagate
    }
}
```

After:
```rust
foo().await.context("doing foo")?           // attach context, propagate
```

Then log exactly once at the top of the call stack (`main`, request handler, task spawn):
```rust
if let Err(e) = run().await {
    error!(error = ?e, "vault sidecar failed");
    std::process::exit(1);
}
```

This produces ONE log line per failure with the full chain, instead of N duplicated logs.

### Pattern: silent fallback → surfaced failure

Before:
```rust
pub async fn namespace() -> String {
    match k8s::namespace().await {
        Ok(ns) => ns,
        Err(e) => {
            error!("Failed: {:?}", e);
            "default".to_string()        // hide the error
        }
    }
}
```

After:
```rust
pub async fn namespace() -> Result<String, K8sError> {
    k8s::namespace().await   // let the caller decide
}
```

If a default genuinely is the right behavior, name it: `unwrap_or_default_namespace()` and document why.

## Workflow

1. Identify the boundary: which crate is this code? `lib` (typed) vs `bin` (anyhow)?
2. Inventory current error shapes (`grep -nE 'Box<dyn|Result<.*String>|map_err\(\|.*format!'`).
3. Sketch the enum(s) — one per module. Show to the user before mass-editing.
4. Convert mechanically: variants → `#[from]` where unambiguous, `#[source]` elsewhere.
5. Replace `.map_err(|e| format!(...))` with the typed variant or `.context(...)`.
6. Move all `error!` logging to the top of the call stack. Lower layers only attach context.
7. Run `cargo check && cargo clippy` — fix the cascade of breakage.
8. Update tests that asserted on `e.to_string().contains(...)` to match on variants.

## Anti-patterns to never reintroduce

- `Result<T, String>` in any function signature.
- `Box<dyn Error>` in a public library function (returning it from `main` is fine).
- `anyhow::Error` in a library's public API (consumers can't match on it).
- Megaenum `AppError` with 40 variants spanning the whole crate.
- `#[from]` on multiple variants that wrap the same upstream type — `?` becomes ambiguous; use `#[source]` + explicit `.map_err`.
- Including secrets (tokens, passwords) in `#[error("...")]` strings.
