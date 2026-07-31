# Architecture Principles — Hermes SSD LLM

Version: 1.0  
Living document — update when core invariants change.  
Complements [ARCHITECTURE.md](../ARCHITECTURE.md) (component map) and [TECHNICAL.md](../TECHNICAL.md) (implementation detail).

---

## Core purpose

Hermes SSD LLM is a **storage routing layer** and optional **local inference runtime** for Hermes Agent on macOS. It validates a registered external SSD, redirects data paths, and launches upstream Hermes unchanged.

---

## Design invariants

These must never be violated without an explicit ADR and user-visible migration path.

### 1. No silent fallback

SSD mode routes all heavy paths to the external drive. If the SSD is missing, unregistered, or unhealthy, the program **fails loudly** — it never silently writes to internal storage.

### 2. Hermes UX unchanged

`hermes ssd` must feel identical to `hermes` from the user's perspective. Only storage location changes. No TUI modifications, no provider flow changes, no required manual exports.

### 3. Verified volume boundary

All SSD-scoped operations are confined to paths under the verified `Hermes-SSD-LLM/` directory on the registered volume. Reset, doctor, and path resolution enforce this boundary.

### 4. Fail closed on ambiguity

Path validation rejects: empty paths, `/`, home directory, `/Volumes` root, volume root, and symlink escapes. When in doubt, reject.

### 5. Preserve upstream Hermes

The launcher execs `hermes.real` (upstream Hermes). The Rust wrapper is never substituted as the real binary. Bootstrap seeds config; it does not replace Hermes itself.

### 6. Measured, not estimated

Performance claims in documentation and benchmarks must come from executed measurements on real hardware. No synthetic or guessed numbers.

### 7. Separation of concerns

| Layer | Responsibility |
|-------|----------------|
| `hermes` (dispatcher) | Route `hermes` vs `hermes ssd` |
| `cli/` | Subcommand parsing and orchestration |
| `device/` | Volume discovery and verification |
| `environment/` | Env var routing (HERMES_HOME, TMPDIR, caches) |
| `launcher/` | Exec upstream Hermes |
| `paths/` | SSD directory layout |
| `bootstrap/` | Seed SSD home from `~/.hermes` |
| `reset/` | Scoped safe cleanup |
| `diagnostics/` | Doctor command |
| `inference/` + `metal/` + `ssd/` | Optional local GGUF inference |

Modules must not leak responsibilities across these boundaries.

---

## Data flow principles

```text
User runs "hermes ssd"
    → device: discover + verify registered SSD
    → paths: resolve Hermes-SSD-LLM/ layout
    → bootstrap: seed missing config from ~/.hermes
    → environment: set HERMES_HOME, TMPDIR, HF_HOME, etc.
    → locks: acquire session lock
    → launcher: exec hermes.real with routed env
    → User interacts with Hermes normally
```

Inference (`hermes-ssd-llm`) is a **separate entry point** — it does not run during normal `hermes ssd` passthrough.

---

## Storage layout principle

Everything heavy lives on the SSD under `Hermes-SSD-LLM/`:

- Hermes home (config, skills, memories, cron)
- Models and caches
- Temporary workspace
- Logs and runtime state

Internal Mac storage is reserved for the OS, apps, and non-AI work.

---

## Error handling principle

- Errors must be actionable (tell the user what to fix)
- No partial state that looks successful
- Doctor command is the diagnostic surface
- Reset is scoped and previewable (`--dry-run`)

---

## Testing principle

- Unit tests for path safety, config migration, bootstrap logic
- Integration tests for CLI routing and reset safety
- Benchmarks as executable scripts, results committed as measured artifacts
- No tests that require a physical SSD in CI — mock or skip with clear markers

---

## Future scaling considerations

- Additional volume types or registration methods → new ADR required
- Linux port → separate ADR; SSD features are macOS-specific today
- Multi-SSD support → must preserve single-verified-volume invariant per session
- Cloud sync → out of scope; SSD is the portability mechanism
