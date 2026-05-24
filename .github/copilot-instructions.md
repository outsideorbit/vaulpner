# Copilot Instructions — vaulpner

Kubernetes sidecar that automatically initializes and unseals HashiCorp Vault in development
environments. Rust 2021 edition, Tokio async runtime, hexagonal architecture (Ports & Adapters).

Read these before making any changes:
- **Coding standards**: [AGENTS.md](../AGENTS.md)
- **Rust best practices**: [docs/rust-best-practices.md](../docs/rust-best-practices.md)
- **Domain-Driven Design background**: [docs/domain-driven-design.md](../docs/domain-driven-design.md)
- **Code review findings**: [docs/code-review.md](../docs/code-review.md)

---

## Commands

```bash
cargo fmt                          # Format — required before every commit
cargo clippy -- -D warnings        # Lint — required before every commit
cargo test                         # Run all tests
cargo test <test_name>             # Run a single test
cargo test -- --nocapture          # Tests with stdout
cargo audit                        # Dependency security audit
```

## Architecture

Hexagonal (Ports & Adapters). Dependency direction: `main.rs` → `adapters/` → `core/`.
`core/` never imports from `adapters/` or any external I/O crate.

```
core/
  model.rs                — domain types: VaultState, InitResult, UnsealKey, RootToken, VaulpnerError
  ports.rs                — port traits (VaultRepository, SecretStore) + BoxFuture type alias
  services/
    vault_lifecycle.rs    — sole business logic entry point: ensure() drives the init/unseal state machine
adapters/
  vault.rs                — vaultrs implementation of VaultRepository
  k8s.rs                  — kube implementation of SecretStore; namespace detection
main.rs                   — wires adapters into ports; exponential-backoff retry loop (no logic)
```

`vault_lifecycle::ensure()` is the only business logic. It checks Vault state and dispatches
to initialize, store keys, retrieve keys, or unseal. `main.rs` does nothing except construct
adapters and call `ensure()` in a retry loop.

## Key Conventions

### Async trait methods — required pattern

All port traits use explicit `BoxFuture`. **Never use `#[async_trait]`**.

```rust
// Defined once in core/ports.rs:
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

// Port trait:
pub trait VaultRepository: Send + Sync {
    fn status(&self) -> BoxFuture<'_, Result<VaultState, VaulpnerError>>;
}

// Adapter implementation:
fn status(&self) -> BoxFuture<'_, Result<VaultState, VaulpnerError>> {
    Box::pin(async move { /* ... */ })
}
```

Callers use `.await` normally — the boxing is transparent at the call site.

### Dependency injection

- `Box<dyn Port>` for exclusively owned dependencies; `Arc<dyn Port>` for shared across tasks.
- Wiring happens only in `main.rs` — never inside `core/`.
- No bare generic bounds (`V: VaultRepository`) for owned dependencies; use `Box`/`Arc`.

### Error handling

- `thiserror` for all error enums — no stringly-typed errors.
- `Box<dyn std::error::Error>` is acceptable only in `main()`.
- No `.unwrap()` or `.expect()` outside tests.
- Adapters convert external errors into `VaulpnerError` variants at layer boundaries.

### Tracing

Structured key-value fields only — never format values into the message string:

```rust
info!(namespace = %namespace, secret = %name, "Storing root token");  // ✅
info!("Storing root token in {} {}", namespace, name);                 // ❌
```

### Testing

- Unit tests in `tests/` use `mockall` mocks injected via port traits — no real I/O.
- Use `#[tokio::test(flavor = "current_thread")]` for any test that mutates environment variables.
- Integration tests in `tests/client_tests.rs` cover adapter construction and namespace detection.
- Mock port traits directly (see `tests/vault_lifecycle_tests.rs` for the pattern).

### Derive order

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
```

`Debug` always first. `Copy` only for small plain-data types with no heap allocation.

### Visibility

- `pub(crate)` for internal implementation details; `pub` only for stable public API.
- Port traits in `core/ports.rs` are `pub` (consumed by `main.rs` and tests).
- Adapter internals are `pub(crate)` or private.

---

## Permissions

### Read Operations
All read operations are automatically permitted. This includes:
- Reading files, directories, and project structure
- Running read-only commands (`cargo check`, `cargo test`, `grep`, `find`, etc.)
- Viewing git history, diffs, and status

### Write Operations
All write operations require explicit user approval before proceeding. This includes:
- Creating, editing, or deleting any file
- Installing or modifying dependencies
- Running commands that mutate state (e.g., `cargo add`, `rm`, etc.)

### Git — Strictly Prohibited Without Approval
- **Never stage or commit changes** (`git add`, `git commit`)
- **Never push to any remote** (`git push`, `git push --force`)
- **Never modify git history** (`git rebase`, `git reset --hard`, etc.)
- Always present proposed changes as diffs or file edits for review first

When in doubt, ask before acting.
