# Project Standards — Hermes SSD LLM

Version: 1.0  
Repository: hermes-ssd-llm

---

## Governance hierarchy

1. **[CONSTITUTION.md](../CONSTITUTION.md)** — absolute engineering authority for this repository
2. **[.hermes/ENGINEERING_CONSTITUTION.md](ENGINEERING_CONSTITUTION.md)** — pointer and workstation-level rules
3. **[DOCUMENTATION_STANDARD.md](DOCUMENTATION_STANDARD.md)** — documentation requirements
4. **[ARCHITECTURE_PRINCIPLES.md](ARCHITECTURE_PRINCIPLES.md)** — design invariants
5. **[AGENTS.md](../AGENTS.md)** — agent-specific instructions

When policies conflict on repo-specific concerns, CONSTITUTION.md wins.

---

## Immutable user workflow

```text
1. Connect the registered SanDisk Portable SSD.
2. Run: hermes ssd
3. Use Hermes normally.
```

Never change the Hermes TUI, provider flow, or require manual environment exports for normal use.

---

## Critical safety rules

| Rule | Rationale |
|------|-----------|
| SSD mode must **never** silently fall back to internal storage | User trusts external drive isolation |
| Reset deletes only under verified `Hermes-SSD-LLM/` on the registered volume | Prevent data loss |
| Reject empty paths, `/`, home, `/Volumes`, volume root, symlink escapes | Path traversal prevention |
| Do not commit secrets, UUIDs in public docs, personal paths, or model weights | Security and privacy |
| Preserve `hermes.real` — never back up the Rust wrapper as real Hermes | Launcher integrity |

---

## Quality gates (required before merge)

```bash
cargo fmt --check
cargo test --lib --tests
cargo build --release
```

ShellCheck on `install.sh`, `uninstall.sh`, `scripts/*.sh`, `benchmarks/scripts/*.sh`.

Benchmarks in BENCHMARKS.md must be **measured**, never estimated.

---

## Code style

- Match existing Rust module boundaries and naming
- Read `ARCHITECTURE.md` and the relevant `src/` module before editing
- Design before implementing (see constitution lifecycle)
- Verify claims against code — do not trust README alone
- Minimal scope: smallest correct diff

---

## Documentation requirements per change type

| Change type | Required doc updates |
|-------------|---------------------|
| New CLI command or flag | README.md, TECHNICAL.md, CHANGELOG.md |
| Architecture change | ARCHITECTURE.md, TECHNICAL.md (+ ADR if significant) |
| New module | ARCHITECTURE.md, TECHNICAL.md |
| Performance change | BENCHMARKS.md (re-run benchmarks) |
| Security change | SECURITY.md, TECHNICAL.md |
| Breaking change | CHANGELOG.md, MIGRATION.md, README.md |
| Bootstrap or config change | README.md, TECHNICAL.md |

---

## Pull request checklist

- [ ] Constitution and safety rules respected
- [ ] Tests pass (`cargo test --lib --tests`)
- [ ] Formatting clean (`cargo fmt --check`)
- [ ] Documentation updated per change type table above
- [ ] No secrets, UUIDs, or personal paths in committed files
- [ ] Benchmarks re-run if performance claims changed

---

## Upstream

Local inference engine derived from [ssd-llm](https://github.com/redbasecap-buiss/ssd-llm). See [NOTICE](../NOTICE).
