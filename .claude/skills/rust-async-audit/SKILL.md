---
name: rust-async-audit
description: Audit async Rust / tokio code for blocking calls, lock-across-await, cancellation safety, runtime mixing, and concurrency anti-patterns. Use when reviewing tokio code, troubleshooting async performance issues, or auditing a tokio-based binary/service. Do NOT use for general Rust review (use rust-review) or error refactors (use rust-error-design).
---

# Async / tokio audit

Async Rust has a small set of footguns that produce subtle, hard-to-debug failures: stalls, deadlocks, dropped state on cancellation, runtime panics. Walk the code against this checklist.

## Workflow

1. Identify the runtime: tokio, async-std, smol? Mixing two in one binary is Major.
2. Grep for the high-signal anti-patterns first (see below).
3. Read each `async fn` and `tokio::spawn` / `task::spawn_blocking` / `select!` / `join!` block.
4. Report findings with `file:line` and severity.

## High-signal greps

Run these and triage results:

```bash
# Blocking calls inside async contexts
rg -n 'std::thread::sleep|std::fs::|std::io::stdin|reqwest::blocking|rusqlite::' --type rust

# Sync locks that may be held across .await
rg -nU 'std::sync::Mutex|parking_lot::Mutex' --type rust -A 5

# Spawned tasks (need to check for Send bounds, panics, error handling)
rg -n 'tokio::spawn|task::spawn' --type rust

# select! without biased — fairness / cancellation traps live here
rg -n 'select!' --type rust -A 2

# Manual Future / Poll impls — usually unnecessary, often buggy
rg -n 'impl.*Future for|fn poll\(' --type rust
```

## Checklist

### Runtime hygiene
- **One runtime per binary.** Mixing tokio + async-std → Major. Even within tokio, don't call `Runtime::block_on` from inside another tokio task.
- **`#[tokio::main]` flavor.** Default is multi-threaded; if the workload is single-task glue, `flavor = "current_thread"` is lighter. Note as Minor if obviously wrong.
- **`tokio::main` worker_threads tuning** is almost always premature; flag unjustified non-default values.

### Blocking inside async — Major
Anything that parks the OS thread inside an async task starves the runtime:
- `std::thread::sleep` → `tokio::time::sleep`
- `std::fs::*` → `tokio::fs::*`
- CPU-bound work (>~10µs) → wrap in `tokio::task::spawn_blocking`
- Synchronous DB / HTTP clients (`reqwest::blocking`, `rusqlite` without `tokio-rusqlite`) → use the async variant or `spawn_blocking`
- Calling `.block_on()` from within an async context → panic on multi-thread runtime, deadlock on current-thread

### Locks across `.await` — Major (subtle)
- `std::sync::Mutex` / `parking_lot::Mutex` held across `.await` → deadlock risk + `!Send` future. Either drop the guard before `.await` or use `tokio::sync::Mutex`.
- `RefCell` borrow across `.await` → panics at runtime if re-entered.
- Pattern to look for:
  ```rust
  let mut guard = state.lock().unwrap();
  let v = some_async_call().await;   // <-- guard still held
  guard.update(v);
  ```
  Fix: drop the guard first, or use `tokio::sync::Mutex` and `await` the lock.

### Concurrency primitives — pick the right one
- **Independent fire-and-forget tasks:** `tokio::spawn` — but ensure the `JoinHandle` is awaited or stored, otherwise panics in the task are silent.
- **Fixed set of awaits, all must succeed:** `tokio::try_join!(a, b, c)`.
- **Fixed set, run concurrently regardless:** `tokio::join!(a, b, c)`.
- **Dynamic set of futures:** `FuturesUnordered` (no spawn) or `JoinSet` (spawns each).
- **Awaiting sequentially when they could run concurrently** is a Minor perf bug — `a.await; b.await;` vs `join!(a, b)`.

### Cancellation safety — Major when violated
`tokio::select!` drops all but the winning branch. Any state partially built up inside a losing branch is lost. Audit each `select!`:
- Are the futures cancel-safe? (Reads from a channel: yes. Sending into a channel: maybe not. Anything that mutates external state mid-future: no.)
- If a branch is not cancel-safe, restructure: pull it out of `select!` and into its own task, or use `tokio::pin!` and `select! { biased; ... }` to control polling order.
- Reference: <https://docs.rs/tokio/latest/tokio/macro.select.html#cancellation-safety>

### Spawned task hygiene
- `tokio::spawn(async move { ... })` with no error path: panics print to stderr but disappear. Wrap the body in a result, log on drop, or use a supervisor pattern.
- `JoinHandle`s dropped without `.await` → task runs detached. Fine if intentional; flag as Minor if it looks accidental.
- Spawning from a `!Send` context will fail to compile; spawning a `!Send` future on the multi-thread runtime ditto. Use `tokio::task::spawn_local` + `LocalSet` if the future is `!Send` on purpose.

### Channels
- `tokio::sync::mpsc` for backpressure (bounded). `unbounded_channel` only when you can prove the producer can't outrun the consumer — otherwise memory leak.
- `tokio::sync::broadcast` for fan-out with lag tolerance; `watch` for "latest value only".
- `oneshot` for single-reply request/response.

### Retry / backoff loops
- Pure `loop { ... tokio::time::sleep(...).await; }` is fine, but:
  - Use `tokio::time::sleep_until` if you need wall-clock anchoring.
  - Add a max-attempts cap; an infinite loop in a sidecar with no exit path is a hang.
  - Use `tokio::select! { _ = shutdown.recv() => break, _ = sleep(...) => {} }` so the loop respects shutdown signals.
- Arithmetic in backoff: use `.saturating_mul()` / `.saturating_add()`, not plain `*` / `+`. A comment claiming "saturating" near plain arithmetic is a Major correctness smell.

### Tracing in async
- `#[tracing::instrument]` on async fns is great; it ensures spans follow tasks.
- Bare `info!`/`debug!` inside `tokio::spawn` will inherit the *spawning* span, not a fresh one — usually desired, but flag if span context looks wrong.

### Tests
- `#[tokio::test]` is fine; `#[tokio::test(flavor = "multi_thread")]` if the test needs real concurrency.
- Sleep-based tests are flaky; prefer `tokio::time::pause()` + `advance()` for time-dependent logic.
- Tests that touch global state (env vars, current dir, signal handlers) cannot run in parallel with `cargo test` — use `serial_test` or single-thread.

## Report format

```
## Async audit — <scope>

### Major
- [`path.rs:L`] <issue> → <fix>

### Minor
- [`path.rs:L`] <issue> → <fix>

### Cancellation review
<one paragraph per `select!` block flagging cancel-safety>

### Lock audit
<table of every sync Mutex/RefCell and whether it crosses .await>

### Recommended next step
<single highest-leverage fix>
```
