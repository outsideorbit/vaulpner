---
name: rust-review
description: Review Rust code for idiomatic style, anti-patterns, and common pitfalls — ownership, API design, logging, lints, secrets handling. Use when the user asks to review Rust code, judge whether something is idiomatic, refactor a `.rs` file, or audit a Rust module. Do NOT use for pure error-handling refactors (use rust-error-design) or async/tokio audits (use rust-async-audit) — invoke those instead when the focus is narrow.
---

# Rust review

Walk the changed/specified Rust files and flag findings against the checklist below. Don't lecture on theory — for each finding, cite `file:line` and propose the concrete fix. End with a punch list ordered by severity.

## Workflow

1. **Run the toolchain first.** Before reading, run these in parallel and let their output drive the review:
   - `cargo clippy --all-targets --all-features -- -W clippy::pedantic 2>&1 | head -200`
   - `cargo fmt -- --check` (report formatting violations, do not auto-fix)
   - `cargo check --all-targets 2>&1 | head -100`
2. **Read the target files.** If the user named files, just those. Otherwise diff against `main` and review only what changed.
3. **Score findings as Major / Minor / Nit.** Major = correctness, security, or public-API damage. Minor = idiom violations the reader will trip on. Nit = style.
4. **Propose fixes inline** — don't just point at problems. If the fix is mechanical, offer to apply it.

## Checklist

### Ownership & borrowing
- `&String` / `&Vec<T>` / `&PathBuf` parameters → should be `&str` / `&[T]` / `&Path`.
- Reflexive `.clone()` to silence the borrow checker — flag and ask whether design change is better.
- `Rc<RefCell<T>>` / `Arc<Mutex<T>>` used as the default ownership model → usually indicates the data flow fights ownership.
- Self-referential structs → recommend indices, `Rc`, or redesign.

### API surface
- `pub fn foo(...) -> Result<T, Box<dyn Error>>` in library code — flag as Major. Public APIs need a typed error (see `rust-error-design`).
- `pub` struct fields that should preserve invariants.
- Boolean parameters at call sites (`render(thing, true, false)`) — recommend an enum.
- Returning `Vec<T>` where `impl Iterator<Item = T>` would let callers skip allocation.
- Newtype opportunities: raw `String`/`u64` IDs passed around — flag domain primitives that deserve `struct UserId(u64)`.

### Errors (quick screen — defer deep refactors to `rust-error-design`)
- `Result<T, String>` — Major. Stringly-typed errors lose structure.
- `.map_err(|e| format!("...: {:?}", e))` — Major. Discards the source error; breaks `?` composition.
- `.unwrap()` / `.expect()` outside tests without a justifying comment — Major in `lib`, Minor in `bin`.
- `let _ = result;` or `.ok();` swallowing errors — flag and ask why.

### Logging & secrets
- **Secrets in log statements.** Grep for `debug!`, `info!`, `trace!`, `println!`, `eprintln!` near identifiers containing `token`, `secret`, `password`, `key`, `credential`. Flag as Major.
- `println!` / `eprintln!` mixed with `tracing` / `log` macros — pick one. Flag as Minor.
- Logging an error AND returning it (every layer doing this) — produces duplicate log lines. Errors should be enriched with `.context(...)` and logged exactly once at the top.
- Glob imports of macro crates (`use tracing::*;`) — Nit; prefer `use tracing::{info, error, debug};`.

### Control flow
- Deeply nested `match` (>3 levels) — usually collapsible with `?`.
- `let mut result = false;` + late-mutation pattern — extract a helper that returns `Result<_, _>` or `Option<_>`.
- Early `return` from inside match arms after logging — flag as readability smell.

### Tests
- `env::set_var` / `env::remove_var` in `#[test]` without serialization — `cargo test` runs tests in parallel; env mutation races. Recommend `serial_test` crate or moving the test to a separate binary target.
- `.unwrap()` in tests is OK; `.expect("why")` is better for debuggability.
- Tests asserting on error *message strings* (`assert!(e.to_string().contains("Failed to"))`) — fragile. Prefer matching on error variants once the error type is typed.

### Performance smells
- `format!("{}", x)` where `x.to_string()` or pass-through would do.
- `Vec::push` in a hot loop without `Vec::with_capacity`.
- `.collect::<Vec<_>>().iter()` — allocated for nothing.
- `.to_owned()` / `.to_string()` / `.clone()` on values only read.

### Hygiene
- `/* */` empty doc-comment placeholders at file tops — delete.
- TODO/FIXME/XXX with no owner or date.
- `#[allow(...)]` without a comment explaining why.
- `unsafe` blocks without a `// SAFETY:` comment naming invariants — Major.
- Comments that explain WHAT (duplicating the code) instead of WHY.
- Misleading comments — e.g., "saturating" near `*` rather than `.saturating_mul()`.

### Main / binary specific
- `fn main() -> Result<(), Box<dyn Error>>` is fine for `bin`; not for `lib`.
- Retry/wait loops that exhaust attempts then `return Ok(())` — exit code should be non-zero on giving up.
- Silent fallback values (`"default".to_string()` on error path) — surface the failure to the caller.

## Report format

```
## Rust review — <scope>

### Major
- [`path.rs:L`] <finding> → <fix>

### Minor
- [`path.rs:L`] <finding> → <fix>

### Nits
- [`path.rs:L`] <finding>

### Clippy
<paste notable lints, omit duplicates of findings above>

### Suggested next step
<single most valuable action — e.g. "Refactor error types per `rust-error-design` before anything else; everything downstream depends on it.">
```

Keep the report skimmable. The user can ask you to apply any fix.
