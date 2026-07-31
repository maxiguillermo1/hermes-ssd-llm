# CONTRIBUTING.md — Hermes SSD LLM

Thank you for contributing. This guide covers engineering standards, workflow, and quality gates for anyone working on Hermes SSD LLM.

For system design context, read [ARCHITECTURE.md](ARCHITECTURE.md) first.  
For deep technical reference, see [TECHNICAL.md](TECHNICAL.md).

---

## Engineering Constitution

All work on this repository is governed by **[CONSTITUTION.md](CONSTITUTION.md)** (Hermes SSD LLM Engineering Constitution v1.0). This is the highest-priority standard.

Persistent engineering and documentation policies live in **[`.hermes/`](.hermes/README.md)** — they travel with the repository so any contributor or agent gets consistent guidance without relying on session memory.

Before writing code:

1. Understand the real problem — not just the symptom
2. Design before implementing — boundaries, data flow, failure modes
3. Challenge your own design from multiple engineering perspectives
4. Implement progressively: correct first, optimize later
5. Self-review before submitting: races, leaks, API consistency, maintenance cost

AI agents working on this repo must also read **[AGENTS.md](AGENTS.md)** and **[`.hermes/DOCUMENTATION_STANDARD.md`](.hermes/DOCUMENTATION_STANDARD.md)**.

The user's personal engineering constitution (`ENGINEERING-CONSTITUTION.md`) is bootstrapped to the SSD Hermes home and applies to all Hermes sessions.

To apply this documentation standard to another repository:

```bash
./scripts/bootstrap-hermes-standards.sh /path/to/repo
```

---

## Who Can Contribute

- Bug fixes and documentation improvements are always welcome
- Feature work should start with an issue or design discussion for significant changes
- All contributors must follow the quality gates below

---

## Development Setup

### Prerequisites

- macOS (primary target; Linux may compile but SSD features require macOS)
- Rust toolchain (`rustup.rs`)
- Hermes Agent installed
- A registered external SSD (for integration testing)

### Build

```bash
git clone https://github.com/maxiguillermo1/hermes-ssd-llm.git
cd hermes-ssd-llm
cargo build
```

### Install locally

```bash
./install.sh
hermes ssd doctor
```

---

## Workflow

1. **Understand** — Read ARCHITECTURE.md and the relevant source modules
2. **Design** — For non-trivial changes, document tradeoffs before coding
3. **Implement** — Minimal focused diff; match existing conventions
4. **Test** — Unit tests for logic; integration tests for paths and CLI
5. **Verify** — Run all quality gates
6. **Document** — Update README, TECHNICAL, or ARCHITECTURE if behavior changes
7. **Changelog** — Add entry to CHANGELOG.md for user-visible changes

---

## Code Style

### Rust

- Edition 2021
- `cargo fmt` before committing (no manual style debates)
- One module, one clear responsibility
- Prefer typed errors (`HermesSsdLlmError`) over string errors
- Use `Result<T>` consistently; map errors with context
- No `unwrap()` in production paths — only in tests
- `#[allow(dead_code)]` on inference engine is intentional (future wiring)

### Naming

- Modules: lowercase (`device`, `environment`)
- Types: PascalCase (`RoutedEnvironment`, `SessionLock`)
- Functions: snake_case (`verify_volume`, `bootstrap_hermes_home`)
- Constants: SCREAMING_SNAKE (`SSD_ROOT_DIR`, `BOOTSTRAP_FILES`)

### Comments

- Code should be self-explanatory
- Comments only for non-obvious business logic or safety invariants
- No commented-out code in commits

### Shell scripts

- `set -euo pipefail` at top
- Quote all paths (volume names may contain spaces)
- ShellCheck clean when available

---

## Critical Rules

These are non-negotiable:

| Rule | Why |
|------|-----|
| **No silent fallback to internal storage** | Core product promise |
| **`allow_internal_fallback` must stay rejected** | Enforced at config load |
| **Preserve `hermes` passthrough** | Normal mode must be unchanged |
| **Reset must validate managed paths** | Prevent data loss outside project dirs |
| **No secrets in commits** | No UUIDs, API keys, `.env`, personal paths |
| **Measured benchmarks only** | No estimated numbers in BENCHMARKS.md |
| **Doctor redacts secrets** | TOKEN, SECRET, PASSWORD, API_KEY |

---

## Testing Expectations

### Required before every PR

```bash
cargo fmt --check
cargo test --lib --tests
cargo build --release
```

### Recommended

```bash
shellcheck install.sh uninstall.sh scripts/*.sh benchmarks/scripts/*.sh
hermes ssd doctor
```

### Test categories

| Category | When to add |
|----------|-------------|
| Unit test | New logic in any module |
| Integration test | Path routing, CLI behavior, config changes |
| Reset safety test | Any change to `reset/` or path validation |
| Benchmark | Performance claims (run on real hardware) |

### Test conventions

- Use `tempfile::TempDir` for filesystem tests
- No network calls in unit tests
- Test both success and failure paths for validation logic

---

## Pull Request Guidelines

### Before opening

- Branch from `main`
- Rebase if needed (no force-push to `main`)
- Ensure clean `cargo test` and `cargo fmt --check`

### PR description should include

1. **What** changed (concise summary)
2. **Why** (problem being solved)
3. **How** (approach and tradeoffs)
4. **Tests run** (exact commands and results)
5. **Workflow verification** — confirm both `hermes` and `hermes ssd` still work

### Review criteria

Reviewers evaluate against the Engineering Constitution:

- Correctness and edge cases
- Security (path safety, secret handling)
- Maintainability (will this make sense in 5 years?)
- Test coverage for behavior changes
- Documentation updates if user-facing

### What we reject

- Drive-by refactors unrelated to the PR
- Silent behavior changes without tests
- Estimated benchmark numbers
- Committed credentials or hardware UUIDs
- Breaking `hermes` passthrough without explicit discussion

---

## Documentation Requirements

| Change type | Update |
|-------------|--------|
| User-facing behavior | README.md |
| Architecture/module changes | ARCHITECTURE.md |
| Technical detail, ADRs | TECHNICAL.md |
| Contributor workflow | CONTRIBUTING.md (this file) |
| Security model | SECURITY.md |
| Performance numbers | BENCHMARKS.md (measured only) |
| Version release | CHANGELOG.md |

Documentation should be written for the correct audience:

- **README.md** — Plain English, no jargon without explanation
- **TECHNICAL.md** — Senior engineers, complete detail
- **ARCHITECTURE.md** — Living component map, update on structural changes

---

## Commit Style

Use conventional prefixes:

```text
feat: add volume health caching
fix: correct stale lock detection on fast restart
docs: rewrite README for beginners
bench: measure startup on M2 + SanDisk Extreme
refactor: extract verify pipeline stages
test: add reset symlink escape case
chore: update Cargo.lock
```

One logical change per commit when possible.

---

## Benchmarks

Run on the registered SSD only:

```bash
./scripts/capture-test-system.sh
./benchmarks/scripts/generate-report.sh
```

Rules:

- Commit sanitized results under `benchmarks/results/`
- Raw private captures stay gitignored
- Redact UUIDs and personal paths in committed reports
- Update BENCHMARKS.md with measured values only
- Note test hardware (Mac model, SSD model, connection type)

---

## Security

- Read [SECURITY.md](SECURITY.md) before touching path handling, exec, or config
- Config files created with mode `0600`
- SSD directories created with mode `0750`
- Report security issues privately — do not open public issues with exploit details

---

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Run full test suite and benchmarks
4. Tag release
5. Verify `install.sh` on clean machine

---

## License

Contributions are licensed under the MIT License. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

By contributing, you agree that your contributions will be licensed under the same terms.

---

## Getting Help

- Architecture questions: start with ARCHITECTURE.md, then TECHNICAL.md
- Hermes Agent integration: [Hermes docs](https://hermes-agent.nousresearch.com/docs)
- Open an issue for bugs; discuss significant features before implementing

---

*Governed by CONSTITUTION.md v1.0 · Hermes SSD LLM v0.3.1*
