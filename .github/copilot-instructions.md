# Project Guidelines

## Architecture & Code Standards

This is a Rust project using hexagonal architecture. The authoritative coding standards are in **[AGENTS.md](../AGENTS.md)** — read it before making any changes.

- **Rust best practices**: See [docs/rust-best-practices.md](../docs/rust-best-practices.md)
- **Domain-Driven Design background**: See [docs/domain-driven-design.md](../docs/domain-driven-design.md)
- **Code review findings**: See [docs/code-review.md](../docs/code-review.md)

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
