# Domain-Driven Design (DDD) Patterns & Code Structure

> Research findings — May 2026

---

## Core Building Blocks

| Concept | Description |
|---------|-------------|
| **Entity** | Object with a unique identity that persists over time (e.g., `User`, `Order`) |
| **Value Object** | Immutable, identity-less object defined by its attributes (e.g., `Money`, `Address`) |
| **Aggregate** | Cluster of entities/value objects treated as a single unit; has one **Aggregate Root** |
| **Domain Event** | Immutable record of something that happened (e.g., `OrderPlaced`, `UserRegistered`) |
| **Repository** | Abstraction for persisting/retrieving aggregates — hides storage details |
| **Domain Service** | Stateless logic that doesn't belong to a single entity (e.g., `PricingService`) |
| **Application Service** | Orchestrates use cases; calls domain services/repos; no business logic itself |
| **Factory** | Encapsulates complex creation logic for aggregates or entities |

---

## Strategic Patterns

- **Bounded Context** — Explicit boundary within which a domain model applies. Different contexts can use the same term differently (e.g., "Account" in billing vs. auth).
- **Ubiquitous Language** — Shared vocabulary between devs and domain experts used consistently in code, docs, and conversation.
- **Context Map** — Documents relationships between bounded contexts (Shared Kernel, Anti-Corruption Layer, Open Host Service, etc.).
- **Anti-Corruption Layer (ACL)** — Translates between your model and external/legacy models to prevent pollution.

---

## Recommended Project Structure

```
src/
├── domain/               # Pure business logic — no I/O, no frameworks
│   ├── model/
│   │   ├── order.rs      # Aggregate root
│   │   ├── order_item.rs # Entity
│   │   └── money.rs      # Value object
│   ├── events/
│   │   └── order_placed.rs
│   ├── repositories/
│   │   └── order_repository.rs  # Trait (interface only)
│   └── services/
│       └── pricing_service.rs
│
├── application/          # Use case orchestration
│   ├── commands/
│   │   └── place_order.rs
│   └── queries/
│       └── get_order.rs
│
├── infrastructure/       # Concrete implementations (DB, HTTP, etc.)
│   ├── persistence/
│   │   └── postgres_order_repo.rs
│   └── messaging/
│       └── event_publisher.rs
│
└── interfaces/           # Entry points (HTTP handlers, CLI, gRPC)
    └── http/
        └── order_handler.rs
```

---

## Key Rules

1. **Domain layer has zero dependencies** on infrastructure or frameworks.
2. **Dependencies point inward** — infrastructure depends on domain, never the reverse (Dependency Inversion).
3. **Aggregates are the only consistency boundary** — only modify one aggregate per transaction.
4. **Repositories return fully-formed aggregates** — never partial/anemic objects.
5. **Application services are thin** — they coordinate, never contain business rules.
6. **Value Objects are immutable** — "changing" one means creating a new one.
7. **Raise domain events inside aggregates** — don't create them in services.

---

## Common Anti-Patterns to Avoid

| Anti-Pattern | Problem |
|---|---|
| **Anemic Domain Model** | Entities are just data bags; logic lives in services — violates encapsulation |
| **Fat Controllers** | Business logic bleeds into handlers/endpoints |
| **Leaky Abstractions** | DB/ORM concepts (rows, tables) appear in the domain layer |
| **God Aggregate** | One aggregate owns too much — becomes a bottleneck |
| **Skipping Bounded Contexts** | One monolithic model that tries to mean everything |

---

## In Rust Specifically

- Use **enums** for domain states and events — exhaustive pattern matching enforces correctness.
- Model **Value Objects** as `struct` with `Copy` or strict `PartialEq`/`Eq` implementations.
- Define **Repository traits** in the domain crate; implement them in the infrastructure crate.
- Use **`#[derive(Debug, Clone, PartialEq)]`** on value objects; keep `Display` for user-facing output.
- Leverage **Rust's module system** to enforce layer boundaries — `pub(crate)` restricts visibility.
- Use a **Cargo workspace** to physically separate domain, application, and infrastructure into distinct crates.

---

## References

- *Domain-Driven Design* — Eric Evans (the "Blue Book")
- *Implementing Domain-Driven Design* — Vaughn Vernon (the "Red Book")
- [DDD Reference](https://www.domainlanguage.com/ddd/reference/) — Evans' condensed reference card
- [Cargo Workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html) — enforce layer separation via separate crates
