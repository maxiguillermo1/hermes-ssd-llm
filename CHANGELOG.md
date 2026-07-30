# Changelog

## v0.3.1 — Engineering Constitution governance (2026-07-30)

### Added

- **`CONSTITUTION.md`** — Hermes SSD LLM Engineering Constitution v1.0 (absolute engineering authority)
- **`AGENTS.md`** — agent and contributor instructions referencing the constitution
- **`ARCHITECTURE.md`** — system design, modes, and module map
- **`SECURITY.md`** — threat model, path safety, and secret handling
- **`CONTRIBUTING.md`** — contribution workflow and quality gates
- **`NOTICE`** — upstream ssd-llm and Hermes Agent attribution

### Unchanged

- User workflow: connect SSD → `hermes ssd` → use Hermes normally
- `hermes` command behavior and Hermes TUI/UX

---

## v0.3.0 — First-run reset and measured benchmarks (2026-07-30)

### Added

- **`hermes ssd reset`** — safe scoped cleanup of runtime state on the registered SSD
- **`hermes ssd reset --dry-run`** — preview paths without deleting
- **`hermes ssd reset --include-models`** — also remove downloaded models
- **`hermes ssd reset --all-managed-data`** — reset all project-managed SSD directories
- Backup manifest written to `<SSD>/Hermes-SSD-LLM/backups/` on each reset
- **`scripts/capture-test-system.sh`** — sanitized hardware + toolchain report
- **Benchmark suite** under `benchmarks/scripts/` (storage, startup, routing, memory)
- Reset path safety validation (rejects home, `/Volumes`, SSD root, symlink escapes)

### Changed

- **README.md** replaced with current Hermes SSD LLM documentation (Rust-first)
- **BENCHMARKS.md** replaced with measured results from detected test hardware only
- Removed inherited/unverified benchmark claims from prior ssd-llm documentation

### Documentation

- `benchmarks/README.md` — how to run benchmarks
- `MIGRATION.md` — naming history and safe reset guide
- `.gitignore` updated for benchmark private captures and artifacts

### Unchanged

- `hermes` command behavior
- Hermes TUI / UX
- SSD identity registration and no-fallback policy

---

## v0.2.0 — Hermes SSD LLM rename (2026-07-30)

- Project renamed to `hermes-ssd-llm`
- SSD root: `<mount>/Hermes-SSD-LLM/`
- Config: `~/.config/hermes-ssd-llm/`

---

## v0.1.0 — Initial release (2026-07-30)

- `hermes ssd` dispatcher, SSD validation, environment routing, doctor, install/uninstall
- Local GGUF inference engine with Metal acceleration
