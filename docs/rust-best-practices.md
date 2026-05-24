# Rust Language Best Practices

> Research findings — May 2026

---

## Core Principles

| Principle | Description |
|-----------|-------------|
| **Ownership & Borrowing** | Memory safety without a GC. Each value has one owner; references must not outlive the owner. |
| **Zero-cost abstractions** | High-level code compiles to efficient machine code with no runtime overhead. |
| **Fearless concurrency** | The type system prevents data races at compile time. |

---

## Best Practices

### Ownership & Memory

- Prefer borrowing (`&T`, `&mut T`) over cloning unless necessary.
- Use `Rc`/`Arc` only when shared ownership is truly needed.
- Avoid `unsafe` unless you fully understand the invariants being upheld.

### Error Handling

- Use `Result<T, E>` for recoverable errors; `panic!` only for truly unrecoverable states.
- Use the `?` operator for ergonomic error propagation.
- Define custom error types with [`thiserror`](https://crates.io/crates/thiserror); aggregate errors in binaries with [`anyhow`](https://crates.io/crates/anyhow).

### Types & Traits

- Prefer `impl Trait` in function signatures over explicit generics when flexibility is not needed.
- Use `From`/`Into` for type conversions to keep APIs ergonomic.
- Leverage `Iterator` combinators (`map`, `filter`, `collect`, etc.) over manual loops.

### Performance

- Profile before optimizing — use [`cargo flamegraph`](https://crates.io/crates/flamegraph) and [`criterion`](https://crates.io/crates/criterion) for benchmarks.
- Avoid premature clones — use slices (`&[T]`, `&str`) instead of owned types in function arguments.
- Derive `Copy` for small, plain data types to avoid move semantics overhead.

### Code Quality

- Run [`clippy`](https://doc.rust-lang.org/clippy/) (`cargo clippy`) — catches many anti-patterns and common mistakes.
- Format code with `rustfmt` (`cargo fmt`) for consistent style.
- Write doc comments (`///`) with runnable examples; test them with `cargo test --doc`.
- Use [`cargo audit`](https://crates.io/crates/cargo-audit) to check for known vulnerabilities in dependencies.

### Project Structure

- Split large crates into a workspace using `[workspace]` in `Cargo.toml`.
- Keep `lib.rs` for library logic; use `main.rs` as a thin entry point.
- Use Cargo feature flags to make optional dependencies truly optional.

### Idiomatic Patterns

- Prefer `Option` combinator methods (`map`, `unwrap_or_else`, `and_then`) over verbose `if let` chains.
- Use the **newtype pattern** to add type safety around primitives (e.g., `struct UserId(u64)`).
- Implement `Display` for user-facing output and `Debug` for developer/diagnostic output.

---

## Essential Tools

| Tool | Command | Purpose |
|------|---------|---------|
| Clippy | `cargo clippy` | Linting & anti-pattern detection |
| Rustfmt | `cargo fmt` | Consistent code formatting |
| Test runner | `cargo test` | Unit, integration & doc tests |
| Benchmark | `cargo bench` | Performance benchmarking |
| Security audit | `cargo audit` | Vulnerability scanning |
| Documentation | `cargo doc --open` | Generate & view API docs |

---

## References

- [The Rust Book](https://doc.rust-lang.org/book/) — Authoritative introduction to Rust
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — Guide to unsafe Rust
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) — Idiomatic library design
- [Rust Clippy Lints](https://rust-lang.github.io/rust-clippy/master/) — Full list of Clippy checks
