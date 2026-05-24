# AGENTS.md — vaulpner Coding Standards

Kubernetes sidecar that automatically initializes and unseals HashiCorp Vault in development environments.
Rust 2021 edition, Tokio async runtime, hexagonal architecture (Ports & Adapters).

See [docs/rust-best-practices.md](docs/rust-best-practices.md) and [docs/domain-driven-design.md](docs/domain-driven-design.md) for background research.

---

## Architecture

This project follows **Hexagonal Architecture (Ports & Adapters)**:

```
core/         — business logic, port traits, domain types (no external crate imports)
adapters/     — concrete implementations of ports (vaultrs, kube)
main.rs       — wiring only: construct adapters, inject into core, retry loop
```

**Dependency rule:** `main.rs` → `adapters/` → `core/`. `core/` never imports from `adapters/` or external I/O crates.

---

## Naming (RFC 430)

- **Types / Traits**: `UpperCamelCase`
- **Functions / Methods / Variables**: `snake_case`
- **Constants / Statics**: `SCREAMING_SNAKE_CASE`
- **Conversions**: `as_` (borrowed view), `to_` (allocates new), `into_` (consumes ownership)
- **Getters**: use the field name directly — no `get_` prefix

---

## Error Handling

- Use `thiserror` for all error enums — no stringly-typed errors
- Use `Result<T, E>` with `?` — never `.unwrap()` or `.expect()` outside tests
- `Box<dyn std::error::Error>` is acceptable **only** in `main()` — use typed errors everywhere else
- Map errors at layer boundaries; adapters convert external errors into `VaulpnerError` variants

---

## Async Patterns

- **Rust 2021 / 1.75+**: async port methods use explicit `Pin<Box<dyn Future>>` — see RPITIT Boxing below
- **Do not use `#[async_trait]`** — explicit boxing is required
- **Blocking I/O** must be wrapped in `tokio::task::spawn_blocking` — never block the async runtime

### Async Trait Methods — Required Pattern (RPITIT Boxing)

All async trait methods **must** use explicit `Pin<Box<dyn Future>>` return types. This is the
required pattern for all port traits in `core/ports.rs`.

**Do not use `#[async_trait]`** — it produces identical runtime behaviour but hides the boxing
cost behind a proc macro. Explicit boxing keeps the contract visible and removes the dependency.

Define a shared type alias in `core/ports.rs`:

```rust
use std::future::Future;
use std::pin::Pin;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
```

Apply it to every async port method:

```rust
pub trait VaultRepository: Send + Sync {
    fn status(&self) -> BoxFuture<'_, Result<VaultState, VaulpnerError>>;
    fn initialize(&self) -> BoxFuture<'_, Result<RootToken, VaulpnerError>>;
    fn unseal(&self, token: &RootToken) -> BoxFuture<'_, Result<(), VaulpnerError>>;
}
```

Implement using `Box::pin(async move { ... })`:

```rust
impl VaultRepository for VaultAdapter {
    fn status(&self) -> BoxFuture<'_, Result<VaultState, VaulpnerError>> {
        Box::pin(async move {
            // implementation
        })
    }
}
```

Callers use `.await` normally — the boxing is transparent at the call site:

```rust
let state = vault.status().await?;
```

---

## Dependency Injection & Composition

This section defines the required patterns for all new code.

### Dependency Injection via `Box<dyn Trait>` / `Arc<dyn Trait>`

Inject dependencies as owned trait objects. This enables runtime polymorphism, clear ownership
semantics, and full testability without a DI framework.

- **`Box<dyn Port>`** — use when a service owns its dependency exclusively (single owner, not shared)
- **`Arc<dyn Port>`** — use when a dependency is shared across tasks or held in application state

```rust
// owned, not shared — use Box
pub struct VaultService {
    vault: Box<dyn VaultRepository>,
    secrets: Box<dyn SecretStore>,
}

// shared across async tasks — use Arc
pub struct AppState {
    vault: Arc<dyn VaultRepository>,
}
```

Construct and inject at the call site (`main.rs`) — never inside `core/`:

```rust
// main.rs — wire concrete adapters into ports
let service = VaultService {
    vault: Box::new(VaultAdapter(vault::client().await?)),
    secrets: Box::new(K8sAdapter(k8s::client().await?)),
};
```

- Do not use DI frameworks — manual wiring only
- Do not use bare generic type parameters (`V: VaultRepository`) as a substitute — prefer `Box`/`Arc` for owned dependencies; use generics only for short-lived, non-owned borrows where heap allocation is measurably undesirable

### Trait-Based Abstractions (Ports)

All external dependencies (Vault API, Kubernetes API, any future secret backends) must be
abstracted behind port traits defined in `core/ports.rs`. Adapters in `adapters/` implement
the ports; `core/` never references concrete adapter types.

```rust
// core/ports.rs — define the port
pub trait VaultRepository: Send + Sync {
    async fn status(&self) -> Result<VaultState, VaulpnerError>;
    async fn initialize(&self) -> Result<RootToken, VaulpnerError>;
    async fn unseal(&self, token: &RootToken) -> Result<(), VaulpnerError>;
}

// adapters/vault.rs — implement the port
impl VaultRepository for VaultAdapter { ... }
```

- New traits must be `Send + Sync`
- Only abstract **external** dependencies — do not create traits for pure internal logic
- Start concrete; extract a trait only when a second implementation exists or testability requires it

### Composition over Inheritance

Rust has no class inheritance. **Composition is the only pattern** — this is not a constraint
but a feature. Apply it explicitly:

- Build complex behaviour by **combining structs** that each hold a single responsibility
- Use **trait objects** (`Box<dyn Port>`) when runtime polymorphism is needed (e.g., selecting
  between multiple future secret backends)
- Use **wrapper structs** (newtype pattern) to add behaviour to external types without modifying them
- **Never** embed trait default method implementations as a substitute for inheritance hierarchies —
  default methods are for shared convenience, not shared state

**Anti-patterns to avoid:**

| Anti-pattern | Why |
|---|---|
| Trait with many default methods holding shared logic | This is inheritance by another name |
| God struct that owns all dependencies | Breaks single responsibility; prefer smaller composed structs |
| Bare generic bounds (`V: Trait`) for owned dependencies | Use `Box<dyn Trait>` instead — generics are for short-lived borrows only |
| Premature abstraction | Start concrete; extract a trait when a real second impl exists |

---

## Trait Derive Order

Always derive in this order when applicable:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
```

- `Debug` — always
- `Clone` — if the type needs to be duplicated
- `PartialEq` / `Eq` — on error enums and domain value types (enables test assertions)
- `Copy` — only for small, plain data types (no heap allocation)

---

## Visibility

- Use `pub(crate)` for types and functions that are internal implementation details
- Only use `pub` for items that form the stable public API of the crate
- Port traits in `core/ports.rs` are `pub` (consumed by `main.rs` and tests)
- Adapter internals are `pub(crate)` or private

---

## Tracing / Logging

Use structured tracing with key-value fields — never format values into the message string:

```rust
// ✅ correct
info!(namespace = %namespace, secret = %name, "Storing root token");

// ❌ wrong
info!("Storing root token in namespace {} secret {}", namespace, name);
```

---

## Testing

- Unit tests for `core/` use `mockall` mocks injected via generic bounds — no real I/O
- Integration tests in `tests/` cover adapter construction and namespace detection
- Use `#[tokio::test(flavor = "current_thread")]` for any test that mutates environment variables
- `core/` must be fully testable without a running Vault or Kubernetes cluster

---

## Tooling

| Tool | Command | Required |
|------|---------|---------|
| Format | `cargo fmt` | Before every commit |
| Lint | `cargo clippy -- -D warnings` | Before every commit |
| Test | `cargo test` | Before every commit |
| Security | `cargo audit` | Periodically |
