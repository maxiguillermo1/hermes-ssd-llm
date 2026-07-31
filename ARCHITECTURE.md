# ARCHITECTURE.md — Hermes SSD LLM

Living high-level architecture reference. Update this document when components, boundaries, or data flows change.

For plain-language overview: [README.md](README.md)  
For deep technical detail and ADRs: [TECHNICAL.md](TECHNICAL.md)

**Last reviewed:** 2026-07-30 · v0.3.4

---

## Purpose

Hermes SSD LLM is a macOS **SSD storage router and launcher** with an **optional** local GGUF inference runtime for [Hermes Agent](https://hermes-agent.nousresearch.com/). It validates a registered external SSD, redirects Hermes-controlled paths, and launches upstream Hermes unchanged. Cloud providers still perform inference remotely unless you configure a local model endpoint.

---

## Component Map

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           Hermes SSD LLM                                │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐   ┌───────────┐ │
│  │   hermes    │   │    cli      │   │   device    │   │   paths   │ │
│  │ (dispatcher)│──▶│  (routing)  │──▶│ (discovery/ │──▶│ (layout)  │ │
│  │             │   │             │   │  verify)    │   │           │ │
│  └─────────────┘   └─────────────┘   └─────────────┘   └───────────┘ │
│         │                                    │                │       │
│         │              ┌─────────────┐       │                │       │
│         │              │  bootstrap  │◀──────┴────────────────┘       │
│         │              │ (seed home) │                                │
│         │              └─────────────┘                                 │
│         │                    │                                         │
│         │    ┌───────────────┼───────────────┐                        │
│         │    ▼               ▼               ▼                        │
│         │ ┌──────┐    ┌────────────┐   ┌──────────┐                  │
│         │ │locks │    │environment │   │ launcher │                  │
│         │ │(PID) │    │ (env vars) │   │ (exec)   │                  │
│         │ └──────┘    └────────────┘   └────┬─────┘                  │
│         │                                    │                        │
│         │                                    ▼                        │
│         │                           ┌─────────────┐                  │
│         │                           │ hermes.real │                  │
│         │                           │  (upstream) │                  │
│         │                           └─────────────┘                  │
│         │                                                             │
│  ┌──────┴──────────────────────────────────────────────────────┐     │
│  │              hermes-ssd-llm (inference CLI)                  │     │
│  │  ┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐  ┌───────┐ │     │
│  │  │ model  │─▶│  ssd   │─▶│ metal  │─▶│inference│─▶│  api  │ │     │
│  │  │ (GGUF) │  │ (mmap) │  │ (GPU)  │  │(engine) │  │(HTTP) │ │     │
│  │  └────────┘  └────────┘  └────────┘  └────────┘  └───────┘ │     │
│  └───────────────────────────────────────────────────────────────┘     │
│                                                                         │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐                  │
│  │   config    │   │    reset    │   │ diagnostics │                  │
│  │ (TOML I/O)  │   │ (safe wipe) │   │  (doctor)   │                  │
│  └─────────────┘   └─────────────┘   └─────────────┘                  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Components and Responsibilities

| Component | Module | Responsibility |
|-----------|--------|----------------|
| **Dispatcher** | `bin/hermes.rs` | Route `hermes` vs `hermes ssd`; no recursive wrapping |
| **CLI** | `cli/` | Subcommand handling: launch, doctor, reset, help |
| **Config** | `config/` | Load/save/validate/migrate `config.toml` |
| **Device** | `device/` | Discover volumes via `diskutil`; verify UUID, space, RW |
| **Paths** | `paths/` | SSD directory constants; `ensure_ssd_layout()` |
| **Bootstrap** | `bootstrap.rs` | Seed SSD `HERMES_HOME` from `~/.hermes` on first launch |
| **Environment** | `environment/` | Build `RoutedEnvironment` env var map |
| **Launcher** | `launcher/` | Resolve `hermes.real`; `exec()` with routed env |
| **Locks** | `locks/` | PID session lock; unclean shutdown detection |
| **Reset** | `reset/` | Scoped cleanup with path safety validation |
| **Diagnostics** | `diagnostics/` | Doctor report generation and printing |
| **Model** | `model/` | GGUF parsing, layer metadata, LRU cache |
| **SSD** | `ssd/` | mmap pool, prefetch, block swap, memory pressure |
| **Metal** | `metal/` | GPU compute kernels (matmul, attention, RoPE) |
| **Inference** | `inference/` | Transformer forward pass, KV cache, sampling |
| **API** | `api/` | OpenAI/Ollama-compatible HTTP server |
| **Errors** | `errors/` | Typed errors with exit codes |

---

## Operating Modes

### Mode A — SSD-backed storage (primary)

Triggered by: `hermes ssd`

Hermes data, caches, temp files, and build artifacts route to the external SSD. Upstream Hermes runs unchanged. Inference is determined by the provider in SSD-based `config.yaml`:

- **Provider mode (default for most users):** OpenRouter, Cursor, Anthropic, etc. — remote inference; `models/gguf` may be empty.
- **Local LLM mode (optional):** Hermes points at a local OpenAI-compatible endpoint; GGUF on SSD + `llama.cpp` or `hermes-ssd-llm` CLI.

### Mode B — Normal passthrough

Triggered by: `hermes` (no `ssd` argument)

No validation. No env mutation. Direct exec to `hermes.real`.

### Mode C — Local inference CLI (optional)

Triggered by: `hermes-ssd-llm` CLI (bench, serve)

GGUF models streamed from SSD with Metal GPU acceleration. Not auto-launched by `hermes ssd`. SSD placement reduces internal-drive use, not inference RAM.

## Credentials

Hermes stores secrets in `HERMES_HOME/.env` and `HERMES_HOME/auth.json`. SSD mode redirects `HERMES_HOME` to `<SSD>/Hermes-SSD-LLM/data/hermes`, so **credentials may reside on the external SSD** after bootstrap. This project does not implement macOS Keychain integration by default. See `SECURITY.md`.

---

## Data Flow

### Launch flow

```mermaid
flowchart LR
    A[config.toml] -->|volume_uuid| B[diskutil]
    B -->|mount_point| C[SsdPaths]
    C --> D[bootstrap]
    D --> E[SessionLock]
    E --> F[RoutedEnvironment]
    F -->|HERMES_HOME, TMPDIR, ...| G[hermes.real]
```

### Inference flow

```mermaid
flowchart LR
    A[GGUF on SSD] -->|mmap| B[Layer N]
    B -->|prefetch| C[Layer N+1]
    B -->|Metal| D[GPU Compute]
    D --> E[KV Cache]
    E -->|pressure| F[SSD Block Swap]
    D --> G[Sampler]
    G -->|token| H[Output]
```

---

## Storage Boundaries

```
┌──────────────── MacBook (internal) ─────────────────┐
│  ~/.config/hermes-ssd-llm/config.toml  (0600)    │
│  ~/.local/bin/hermes          (Rust dispatcher)   │
│  ~/.local/bin/hermes.real     (upstream Hermes)   │
│  (optional) symlinks for secrets on internal disk │
└───────────────────────────────────────────────────┘
                        │
                        │ USB
                        ▼
┌──────────────── External SSD ─────────────────────┐
│  /Volumes/<name>/Hermes-SSD-LLM/                  │
│    data/hermes/     ← HERMES_HOME (.env, auth.json may live here) │
│    models/gguf/     ← optional local models       │
│    cache/           ← HF, Rust, Hermes caches     │
│    tmp/             ← TMPDIR                      │
│    logs/            ← Application logs            │
│    runtime/         ← Locks, sessions, state      │
│    repositories/    ← Git clones                  │
│    workspaces/      ← Active projects             │
└───────────────────────────────────────────────────┘
```

---

## Key Invariants

1. `allow_internal_fallback` is always `false` and rejected if set `true`
2. SSD validation runs before every `hermes ssd` launch
3. Reset only operates on paths under `<SSD>/Hermes-SSD-LLM/`
4. `HERMES_HOME` redirect may place `.env` / `auth.json` on the SSD unless symlinked elsewhere
5. One active SSD session per volume (PID lock)
6. `hermes` passthrough never modifies environment

---

## External Dependencies

| Dependency | Role |
|------------|------|
| Hermes Agent | Upstream AI assistant (Python) |
| macOS `diskutil` | Volume discovery |
| macOS `df` | Free space check |
| Metal framework | GPU inference (Apple Silicon) |
| Rust toolchain | Build and runtime |

---

## Test Coverage Map

| Area | Test Location |
|------|---------------|
| Config validation | `config/hermes_ssd_llm_config.rs` |
| Path layout | `paths/mod.rs`, `tests/integration_paths.rs` |
| Env routing | `environment/mod.rs` |
| Lock stale detection | `locks/mod.rs` |
| Reset path safety | `reset/mod.rs`, `tests/reset_safety.rs` |
| CLI dispatch | `tests/cli_routing.rs` |
| Bootstrap | `bootstrap.rs` |

---

## Change Log

| Date | Change |
|------|--------|
| 2026-07-30 | Added `bootstrap.rs` for SSD home seeding |
| 2026-07-30 | Documentation rewrite (README, TECHNICAL, ARCHITECTURE, CONTRIBUTING) |
| 2026-07-30 | Documentation accuracy pass: provider vs local modes, credentials honesty |

---

## When to Update This Document

Update ARCHITECTURE.md when you:

- Add or remove a module
- Change data flow or storage boundaries
- Add a new operating mode
- Change external dependencies
- Modify the SSD directory layout
- Change validation or lock behavior

Do **not** duplicate ADR rationale here — link to [TECHNICAL.md](TECHNICAL.md#architecture-decision-records).
