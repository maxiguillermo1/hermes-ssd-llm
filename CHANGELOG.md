# Changelog

## v0.2.0 — Hermes SSD LLM rename (2026-07-30)

### Changed

- Project renamed from `hermes-ssd` to `hermes-ssd-llm`
- Rust package `hermes-ssd-llm`, library crate `hermes_ssd_llm`, binary `hermes-ssd-llm`
- SSD directory root: `<mount>/Hermes-SSD-LLM/`
- Config directory: `~/.config/hermes-ssd-llm/`
- Environment variables prefixed with `HERMES_SSD_LLM_`

### Added

- Config migration from legacy `~/.config/hermes-ssd/`
- Backward-compatible SSD root detection for existing `Hermes-SSD/` directories

### Unchanged

- User command remains `hermes ssd`
- Normal `hermes` behavior unchanged
- Hermes TUI, providers, skills, and tools unchanged

---

## v0.1.0 — Initial release (2026-07-30)

### Added

- **`hermes ssd` command** — dispatcher wrapping the real Hermes executable with SSD verification and environment routing
- **Stable SSD identity** — volume UUID registration via `diskutil`, external-device checks, read/write probes, free-space thresholds
- **SSD directory layout** — models, cache, data, runtime, logs on external SSD
- **Environment routing** — `HERMES_HOME`, `TMPDIR`, `HF_HOME`, `HUGGINGFACE_HUB_CACHE`, `TRANSFORMERS_CACHE`, `CARGO_TARGET_DIR`, XDG paths
- **`hermes ssd doctor`** — diagnostics without exposing secrets
- **Install/uninstall scripts** — idempotent user-local install to `~/.local/bin`
- **Local inference engine** — SSD streaming, Metal, GGUF, API stack

---
