# Agent instructions — Hermes SSD LLM

Read `CONSTITUTION.md` first. It is the absolute engineering authority for this repository and workstation.

Then read `.hermes/README.md` and the policy files it indexes. Repository standards live in `.hermes/` (not agent memory) so documentation and engineering behavior are reproducible across sessions.

## Immutable user workflow

```text
1. Connect the registered SanDisk Portable SSD.
2. Run: hermes ssd
3. Use Hermes normally.
```

- `hermes` — standard Hermes (internal paths, unchanged UX)
- `hermes ssd` — verified SSD-backed storage routing, then same Hermes UX
- `hermes ssd doctor` — diagnostics
- `hermes ssd reset --dry-run` / `hermes ssd reset` — scoped first-run reset

Never change the Hermes TUI, provider flow, or require manual env exports for normal use.

## Before editing

1. Read `ARCHITECTURE.md` and the relevant `src/` module.
2. Verify claims against code — do not trust README alone.
3. Design before implementing (see constitution lifecycle).
4. Match existing Rust style and module boundaries.

## Project layout

```text
src/
  bin/hermes.rs           # Dispatcher (hermes / hermes ssd)
  bin/hermes_ssd_llm.rs     # Doctor, register, inference CLI
  cli/                      # SSD subcommand routing
  config/                   # TOML config + migration
  device/                   # Volume discovery (diskutil)
  environment/              # SSD path routing (HERMES_HOME, caches, etc.)
  launcher/                 # exec real Hermes, resolve hermes.real
  locks/                    # Session lock, unclean shutdown
  reset/                    # Safe scoped reset
  paths/                    # SSD directory layout
  diagnostics/              # Doctor command
  ssd/, metal/, inference/  # Local GGUF inference engine
```

## Safety rules

- SSD mode must **never** silently fall back to internal storage.
- Reset must only delete paths under verified `Hermes-SSD-LLM/` on the registered volume.
- Reject empty paths, `/`, home, `/Volumes`, volume root, and symlink escapes.
- Do not commit secrets, UUIDs in public docs, personal paths, or model weights.
- Preserve `hermes.real` — never back up the Rust wrapper as real Hermes.

## Quality gates (required before merge)

```bash
cargo fmt --check
cargo test --lib --tests
cargo build --release
```

Run ShellCheck on `install.sh`, `uninstall.sh`, and `scripts/*.sh`, `benchmarks/scripts/*.sh`.

## Documentation

Follow `.hermes/DOCUMENTATION_STANDARD.md` for structure and quality. Keep in sync with code:

- `README.md` — beginner-facing (plain English, explain all jargon)
- `TECHNICAL.md` — senior-engineer reference (architecture, ADRs, diagrams)
- `ARCHITECTURE.md` — living component map (update when boundaries change)
- `CONTRIBUTING.md` — contributor workflow and quality gates
- `BENCHMARKS.md` — measured results only (no estimates)
- `CHANGELOG.md` — every release
- `MIGRATION.md` — naming and reset history

To bootstrap these standards into another repository: `./scripts/bootstrap-hermes-standards.sh /path/to/repo`

## Upstream

Local inference engine derived from [ssd-llm](https://github.com/redbasecap-buiss/ssd-llm). See `NOTICE`.
