# Codebase Audit — Best Practices Review

> Reviewed against [Rust Best Practices](./rust-best-practices.md) and [Domain-Driven Design](./domain-driven-design.md)
> Date: May 2026

---

## Severity Scale

| Rating | Meaning |
|--------|---------|
| 🔴 **3 — Critical** | Runtime risk, untestable code, or data correctness issue |
| 🟠 **2 — Moderate** | Violates best practices, hinders maintainability or reliability |
| 🟡 **1 — Minor** | Code smell, style inconsistency, or low-risk improvement |

---

## Summary

| Severity | Count |
|----------|-------|
| 🔴 Critical (3) | 2 |
| 🟠 Moderate (2) | 13 |
| 🟡 Minor (1) | 7 |
| **Total** | **22** |

---

## 🔴 Critical Findings

### C1 — Panic-prone index access on Vault keys response
- **File:** `src/vault.rs:34`
- **Code:** `Ok(init_response.keys[0].clone())`
- **Issue:** If Vault returns an empty `keys` list the program panics at runtime. No bounds check or guard exists.
- **Fix:** `init_response.keys.first().cloned().ok_or(VaulpnerError::EmptyKeysResponse)?`

---

### C2 — Business logic unreachable from the test harness
- **File:** `src/main.rs:11–127`
- **Issue:** `initialize_vault`, `get_current_namespace`, and `ensure` are defined in `main.rs`. Because `main.rs` is a binary entry point, none of these functions are accessible to `tests/` via `use vaulpner::...` — they are silently invisible to the test harness. The three most complex functions in the project have zero test coverage.
- **Fix:** Move all three into `src/lib.rs` (or a dedicated `src/app.rs` re-exported from `lib.rs`).

---

## 🟠 Moderate Findings

### M1 — `ensure()` swallows all errors behind a `bool`
- **File:** `src/main.rs:39–127`
- **Issue:** The signature `pub async fn ensure(...) -> bool` discards every error. Callers cannot distinguish "vault not ready" from an unrecoverable failure. The retry loop in `main()` blindly retries regardless of why `ensure` returned `false`.
- **Fix:** Return `Result<bool, VaulpnerError>` and propagate errors upward.

---

### M2 — `get_current_namespace()` silently eats errors
- **File:** `src/main.rs:26–33`
- **Issue:** On failure it logs a warning and returns `"default"`. In a misconfigured environment this silently allows the program to operate in the wrong namespace. The `-> String` signature hides the failure from callers.
- **Fix:** Return `Result<String, VaulpnerError>` and propagate the error.

---

### M3 — String errors used throughout instead of typed error variants
- **Files:** `src/vault.rs:12,15`, `src/k8s.rs:12,15`, `src/main.rs:13`
- **Issue:** `Box<dyn std::error::Error>` with `format!`-generated strings prevents callers from pattern-matching error kinds, makes structured logging impossible, and erases original error types. `thiserror` is already in `Cargo.toml` but is never used.
- **Fix:** Define a `VaulpnerError` enum with `#[derive(thiserror::Error)]` and replace all `Box<dyn Error>`.

---

### M4 — Double-logged errors in `initialize_vault()` and `vault::initialize()`
- **Files:** `src/main.rs:11–24`, `src/vault.rs:27–41`
- **Issue:** `vault::initialize()` logs the error, then `initialize_vault()` catches and logs the same error again. Every initialization failure produces two identical log lines.
- **Fix:** Remove the redundant match in `vault::initialize()` — use `?` with a success-path log. Let `initialize_vault()` own the error log.

---

### M5 — Same modules declared in both `main.rs` and `lib.rs`
- **Files:** `src/main.rs:4–5`, `src/lib.rs:1–2`
- **Issue:** Both files contain `mod k8s; mod vault;`. The binary gets its own private shadow copies of the modules, separate from what `lib.rs` exports. Changes must be reflected in two places.
- **Fix:** Remove the `mod` declarations from `main.rs`; import with `use vaulpner::k8s; use vaulpner::vault;`.

---

### M6 — No layer separation (domain / application / infrastructure)
- **File:** `src/` directory
- **Issue:** All logic lives in four flat files with no separation between domain rules, use-case orchestration, and infrastructure adapters (Vault/k8s HTTP). This makes it impossible to swap or mock infrastructure without real external services.
- **Suggested structure:**
  ```
  src/
    domain/     # VaultState, VaulpnerError, policy rules
    app/        # ensure, initialize_vault (use-cases)
    infra/      # vault.rs, k8s.rs (HTTP/k8s adapters)
    main.rs     # wiring only
  ```
- See [Domain-Driven Design](./domain-driven-design.md#recommended-project-structure)

---

### M7 — Retry / back-off policy hard-coded in `main()`
- **File:** `src/main.rs:139–152`
- **Issue:** Max attempts (`5`), initial delay (`2`), and the back-off formula are business-policy decisions embedded in the binary entry point. They are untestable, unconfigurable, and non-standard.
- **Additional bug:** The formula `count_increment = (count_increment * count).min(60)` produces inconsistent delays (2, 4, 12, 48, 60…). The variable `count_increment` actually stores the sleep duration — a misleading name.
- **Fix:** Extract into a configurable struct or function; use standard linear or exponential back-off.

---

### M8 — No repository abstraction — direct infra calls in orchestration
- **File:** `src/main.rs:47–112`
- **Issue:** `ensure()` calls `vault::initialize`, `k8s::create_secret`, `k8s::get_secret`, and `vault::unseal` directly with no trait abstractions. There is no way to inject mock implementations, making the core logic integration-test-only.
- See [Domain-Driven Design](./domain-driven-design.md#key-rules)

---

### M9 — Semantically incorrect double base64 encoding
- **Files:** `src/k8s.rs:49–52`, `src/main.rs:69–96`
- **Issue:** `create_secret` manually base64-encodes the value before wrapping it in `ByteString`. The `kube` library handles base64 for transport automatically — manual encoding means the stored secret contains base64-encoded bytes where raw bytes are expected. The read path manually decodes to compensate. This works accidentally but is fragile; any change to one side without the other silently corrupts tokens.
- **Fix:** Store `ByteString(value.as_bytes().to_vec())` in `create_secret`; read with `String::from_utf8(...)`, removing the manual `base64::decode` call.

---

### M10 — No doc comments on any public items
- **Files:** `src/vault.rs`, `src/k8s.rs`, `src/main.rs`, `src/lib.rs`
- **Issue:** All public functions are undocumented. `cargo doc` produces empty API pages. The crate is published to docs.rs (per `Cargo.toml`).
- **Fix:** Add `///` doc comments to all public items; add a `//!` module-level comment to `lib.rs`.

---

### M11 — `env::set_var` in async tests causes race conditions
- **File:** `tests/client_tests.rs:52,80,88`
- **Issue:** `std::env::set_var` / `remove_var` mutate the process-global environment. Concurrent async tests racing on `POD_NAMESPACE` / `KUBECONFIG` produce flaky, non-deterministic results.
- **Fix:** Use `#[tokio::test(flavor = "current_thread")]` or the `serial_test` crate for these tests.

---

### M12 — No tests for core business logic
- **Files:** `tests/client_tests.rs`, `src/main.rs`
- **Issue:** Zero tests cover `ensure()`, `initialize_vault()`, or `get_current_namespace()`. The five existing tests only cover client construction and namespace env-var detection. The root cause is M5/C2 above — these functions are unreachable from the test harness.

---

### M13 — Anemic `lib.rs` — no public API surface
- **File:** `src/lib.rs:1–2`
- **Issue:** The library crate only re-exports raw infrastructure modules. There are no public types, traits, or documented stable interfaces. Any consumer of the crate gets unabstracted implementation details.

---

## 🟡 Minor Findings

### m1 — Redundant `client.clone()` on every k8s API call
- **File:** `src/k8s.rs:25,64`
- **Issue:** `kube::Client` is `Arc`-wrapped internally so `.clone()` is cheap, but the functions accept `&kube::Client` and immediately clone to satisfy the `kube::Api` constructor. Worth noting for future refactoring.

---

### m2 — `key.to_string()` allocation forced by external API
- **File:** `src/vault.rs:44`
- **Issue:** `vaultrs::sys::unseal` requires `Option<String>`, forcing a `to_string()`. Consider a local wrapper that accepts `&str` and converts internally to keep callers clean.

---

### m3 — `test_namespace_detection_with_service_account` is a no-op
- **File:** `tests/client_tests.rs:100–113`
- **Issue:** The test creates and deletes a temp file but never exercises the code path it claims to test. The inline comment admits it won't run in practice. Provides false confidence.
- **Fix:** Implement with path injection or remove the test.

---

### m4 — `eprintln!` used instead of `tracing::warn!`
- **File:** `src/k8s.rs:41,45`
- **Issue:** Two warning messages bypass the project's `tracing` logger, won't appear in log collectors, and can't be filtered by log level.
- **Fix:** Replace with `tracing::warn!(...)`.

---

### m5 — Empty placeholder comments in three source files
- **Files:** `src/main.rs:1–2`, `src/vault.rs:1–2`, `src/k8s.rs:1–2`
- **Issue:** `/* */` block comments at the top of each file add no information.
- **Fix:** Remove or replace with meaningful crate/module-level doc comments.

---

### m6 — `thiserror` in `[dependencies]` but never used
- **File:** `Cargo.toml:31`
- **Issue:** `thiserror` is compiled into the binary but only appears to be there for transitive version pinning. If version pinning is the goal, use `[patch.crates-io]` instead.
- **Fix:** Either use `thiserror` properly (strongly recommended — see M3) or remove it from direct dependencies.

---

### m7 — `vault::initialize()` could use `?` instead of nested match
- **File:** `src/vault.rs:27–41`
- **Issue:** The match block adds no logic beyond redundant logging. Idiomatic Rust would use `?` propagation, reducing five lines of boilerplate.

---

## Top Priorities (Ordered)

1. **Move `ensure`, `initialize_vault`, `get_current_namespace` into `lib.rs`** — unblocks all testing and fixes the core DDD violation.
2. **Define `VaulpnerError` with `thiserror`** — replaces all `Box<dyn Error>` and format-string errors.
3. **Guard `keys[0]`** — replace with `.first().ok_or(VaulpnerError::EmptyKeysResponse)?`.
4. **Fix the double base64 encoding** — store raw bytes, drop the manual decode.
5. **Return `Result` from `ensure()` and `get_current_namespace()`** — enables proper error handling in the retry loop.
6. **Extract retry/back-off policy** into a configurable, testable structure outside `main()`.
7. **Add unit tests** using `mockall` (already in dev-deps) behind `VaultRepository` and `SecretStore` traits.
