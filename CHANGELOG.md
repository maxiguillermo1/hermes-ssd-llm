# Changelog

## v0.3.5 — Shipped vs roadmap clarity (2026-07-30)

### Added

- **`ROADMAP.md`** — separates shipped launcher features from planned work
- **`ADVANCED.md`** — developer guide for in-repo inference engine (not daily workflow)

### Changed

- **README.md** — accurate SSD contents table; removed misleading embeddings/indexes/vector-db rows; roadmap links
- **ARCHITECTURE.md** — inference engine labeled as advanced library; Mode C corrected
- **TECHNICAL.md** — scope table; CLI split into shipped vs planned; version 0.3.5
- **BENCHMARKS.md** — fixed RAM claim; removed nonexistent `hermes-ssd-llm bench` command
- **PROJECT_MANIFEST.md** — fixed host config path; shipped vs advanced split
- **AGENTS.md**, **benchmarks/README.md** — aligned with roadmap

---

## v0.3.4 — Documentation accuracy (2026-07-30)

### Changed

- **README.md** — reframed as SSD storage router/launcher; added provider vs local inference; corrected Keychain, RAM, and portability claims while keeping plain-English structure
- **ARCHITECTURE.md** — provider/local mode split; honest credential placement via `HERMES_HOME` redirect
- **SECURITY.md** — documents `.env` / `auth.json` on SSD
- **MIGRATION.md** — clarified credential storage on reset
- **BENCHMARKS.md** — disambiguated provider-mode vs local-inference scope
- **TECHNICAL.md** — corrected credential placement via `HERMES_HOME` redirect

---

## v0.3.3 — Documentation standardization initiative (2026-07-30)

### Added

- **`.hermes/`** — persistent repository policies (engineering constitution pointer, documentation standard, project standards, architecture principles)
- **`.hermes/templates/`** — README and TECHNICAL starter templates
- **`scripts/bootstrap-hermes-standards.sh`** — install `.hermes/` standards into any Git repository

### Changed

- **`AGENTS.md`** — agents must read `.hermes/` policies; expanded documentation requirements
- **`CONTRIBUTING.md`** — references `.hermes/` and bootstrap script
- **`.gitignore`** — ignore macOS `._*` metadata files on ExFAT volumes

---

## v0.3.2 — Documentation rewrite (2026-07-30)

### Added

- **`TECHNICAL.md`** — comprehensive technical reference for senior engineers (architecture, boot sequence, ADRs, threat model, benchmark methodology)

### Changed

- **`README.md`** — complete rewrite in plain English for non-technical readers
- **`ARCHITECTURE.md`** — living component map with diagrams and change log
- **`CONTRIBUTING.md`** — expanded engineering standards, PR guidelines, and constitution references

---

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
