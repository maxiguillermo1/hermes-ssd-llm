# Roadmap — Hermes SSD LLM

This document separates **what ships today** from **future or advanced work** that exists in the repository but is not part of the daily `hermes ssd` workflow.

For the plain-language overview, see [README.md](README.md).

**Last reviewed:** 2026-07-30 · v0.3.5

---

## Shipped today (primary product)

These are the features the project is built and tested around:

| Feature | Command / location |
|---------|-------------------|
| SSD volume registration | `./install.sh`, `hermes-ssd-llm register` |
| SSD validation (UUID, space, RW, external) | Every `hermes ssd` launch |
| Hermes path routing | `HERMES_HOME`, `TMPDIR`, HF/Rust caches → SSD |
| First-run bootstrap | Seeds SSD `HERMES_HOME` from `~/.hermes` |
| Session lock | One active SSD session per volume |
| Launch upstream Hermes | `hermes ssd` → `hermes.real` |
| Normal Hermes passthrough | `hermes` (unchanged) |
| Health report | `hermes ssd doctor` |
| Scoped reset | `hermes ssd reset` (+ `--dry-run`, `--include-models`, `--all-managed-data`) |
| Standalone admin CLI | `hermes-ssd-llm doctor`, `register`, `launch`, `info`, `models` |
| Measured launcher benchmarks | `benchmarks/scripts/*`, [BENCHMARKS.md](BENCHMARKS.md) |

**Default inference path:** whatever provider you select in Hermes (`hermes model`). Cloud providers run remotely. No GGUF file is required.

---

## Advanced / in-repo (not daily workflow)

The repository contains a **local GGUF inference engine** (derived from upstream [ssd-llm](https://github.com/redbasecap-buiss/ssd-llm)) under `src/inference/`, `src/metal/`, `src/ssd/`, and `src/api/`. It is library code with unit tests. It is **not** wired into `hermes ssd` and several planned CLI entry points are **not exposed yet**.

| Capability | Status | Notes |
|------------|--------|-------|
| GGUF parse / metadata | Shipped in library | `hermes-ssd-llm info`, `models` |
| Layer streaming + Metal kernels | In repo, not daily path | See [ADVANCED.md](ADVANCED.md) |
| OpenAI/Ollama HTTP server | In repo, no CLI | `src/api/server.rs` |
| Inference benchmarks | Criterion bench only | `cargo bench --bench inference_bench` |
| `hermes-ssd-llm bench` CLI | **Planned** | Documented in older drafts; not in binary yet |
| `hermes-ssd-llm serve` CLI | **Planned** | Same |
| Auto-start local server with `hermes ssd` | **Planned** | Would make "SSD LLM" name fully literal |

See [ADVANCED.md](ADVANCED.md) for how to explore the inference engine as a developer.

---

## Future roadmap

### Local inference integration

- Expose `hermes-ssd-llm bench` and `hermes-ssd-llm serve` CLI commands
- Optional auto-start of local OpenAI-compatible endpoint when `config.yaml` requests it
- MLX model path alongside GGUF
- Documented Hermes provider preset for `http://127.0.0.1:8080/v1`

### Security and credentials

- macOS Keychain integration for API keys (alternative to SSD-stored `.env` / `auth.json`)
- Symlink/bootstrap options to keep secrets on internal disk while data stays on SSD
- Encrypted-volume detection warnings in `doctor`

### Storage and portability

- APFS-first optimizations (clones, sparse files) with ExFAT fallback
- Multi-SSD profiles (work vs personal volumes)
- Automated backup hooks into `backups/`
- Reduce `doctor` latency with cached validation

### UX

- Menu bar / GUI SSD health indicator
- Clearer first-run credential placement wizard

### CLI naming (optional)

Original design used `ssd hermes` (SSD manager launches Hermes). The shipped binary uses `hermes ssd` (Hermes subcommand). Both are documented here; changing the canonical command would be an install/symlink change, not docs-only.

---

## Naming note

**Hermes SSD LLM** describes the full vision (SSD workspace + optional local models). If you only use **provider mode**, the accurate short description is **Hermes SSD workspace** or **Hermes SSD runtime** — storage routing with cloud inference. The project name stays as-is until local inference is integrated into the default launch path.
