# PROJECT_MANIFEST.md — Agent Entry Point

**Purpose:** Machine-friendly project summary for AI coding agents. Humans should start with [README.md](README.md).

**Last verified:** 2026-07-30  
**Repository:** hermes-ssd-llm

---

## Identity

| Field | Value |
|-------|-------|
| **Name** | Hermes SSD LLM |
| **Domain** | macOS storage routing + optional local GGUF inference for Hermes Agent |
| **Status** | Active (v0.3.x) |
| **License** | MIT — Copyright (c) 2026 Maxi Guillermo |

---

## What this project does

Routes Hermes Agent data (models, caches, logs, builds, temp files) to a verified external SSD so the MacBook stays lightweight. Optionally runs local GGUF inference from SSD-backed weights. Never silently falls back to internal storage in SSD mode.

---

## Tech stack

| Layer | Technology |
|-------|------------|
| **Language** | Rust (edition 2021) |
| **Framework / runtime** | Native macOS CLI binaries (`hermes`, `hermes-ssd-llm`) |
| **Backend / data** | TOML config, diskutil volume verification, mmap on SSD |
| **Build / deploy** | `cargo build --release`, `install.sh` |
| **Test** | `cargo test --lib --tests`, ShellCheck on shell scripts |

---

## Repository layout

```text
src/
  bin/hermes.rs           # Dispatcher: hermes / hermes ssd
  bin/hermes_ssd_llm.rs   # Doctor, register, inference CLI
  cli/ config/ device/ environment/ launcher/ locks/
  reset/ paths/ diagnostics/ ssd/ metal/ inference/
.hermes/                  # Engineering + documentation policies (read first)
scripts/                  # bootstrap-hermes-standards.sh, install helpers
benchmarks/               # Measured performance scripts + results
```

---

## Key commands

| Command | Purpose |
|---------|---------|
| `cargo build --release` | Build binaries |
| `cargo test --lib --tests` | Unit + integration tests |
| `cargo fmt --check` | Format check |
| `./install.sh` | Install wrapper to system |
| `hermes ssd doctor` | Verify SSD routing health |
| `./benchmarks/scripts/generate-report.sh` | Run benchmark suite |
| `./scripts/bootstrap-hermes-standards.sh <repo>` | Copy `.hermes/` to another Git repo |

---

## Architecture (summary)

1. User runs `hermes ssd` → CLI validates registered volume (UUID, space, mount, RW).
2. `environment` builds routed env vars (`HERMES_HOME`, `TMPDIR`, `HF_HOME`, etc.).
3. `launcher` execs upstream `hermes.real` with routed environment.
4. Optional: `hermes-ssd-llm` runs GGUF inference with SSD mmap + Metal GPU.

**Canonical detail:** [ARCHITECTURE.md](ARCHITECTURE.md) · [TECHNICAL.md](TECHNICAL.md)

---

## Data & configuration

| Item | Location |
|------|----------|
| **Config** | `<SSD>/Hermes-SSD-LLM/config.toml` (after registration) |
| **User / runtime data** | Under verified `Hermes-SSD-LLM/` on registered volume |
| **Secrets** | Never commit; bootstrap copies `~/.hermes/.env` to SSD home only when missing |

---

## Engineering standards

Read before editing:

1. [.hermes/README.md](.hermes/README.md) — policy index
2. [CONSTITUTION.md](CONSTITUTION.md) — engineering constitution
3. [.hermes/PROJECT_STANDARDS.md](.hermes/PROJECT_STANDARDS.md) — quality gates
4. [AGENTS.md](AGENTS.md) — agent workflow
5. [CONTRIBUTING.md](CONTRIBUTING.md) — contributor workflow

---

## Testing workflow

```bash
cargo fmt --check
cargo test --lib --tests
cargo build --release
# ShellCheck: install.sh, uninstall.sh, scripts/*.sh, benchmarks/scripts/*.sh
```

Benchmarks: measured results only in `BENCHMARKS.md` — run `generate-report.sh` first.

---

## Deployment

`./install.sh` installs the Rust wrapper; preserves `hermes.real` as upstream Hermes. SSD must be connected and registered before `hermes ssd` works.

---

## Constraints (do not violate)

- SSD mode must **never** silently fall back to internal storage
- Reset/delete only paths under verified `Hermes-SSD-LLM/` on the registered volume
- Reject empty paths, `/`, home, `/Volumes` root, symlink escapes
- Do not commit secrets, model weights, or personal paths in public docs
- Never replace `hermes.real` with the Rust wrapper backup

---

## Important files

| File | Why it matters |
|------|----------------|
| `src/cli/mod.rs` | SSD subcommand routing |
| `src/environment/` | Path routing logic |
| `src/device/` | Volume discovery and verification |
| `src/launcher/` | Exec upstream Hermes |
| `src/bootstrap.rs` | First-run SSD home seeding |
| `CONSTITUTION.md` | Highest-priority engineering standard |

---

## Documentation map

| Audience | Start here |
|----------|------------|
| **Beginner** | [README.md](README.md) |
| **Contributor** | [CONTRIBUTING.md](CONTRIBUTING.md) |
| **Engineer** | [TECHNICAL.md](TECHNICAL.md) |
| **Agent** | This file |
