# Migration — Hermes SSD LLM

## Naming history

| Era | Name | Notes |
|-----|------|-------|
| Original prototype | SSD-PORT / ssd-llm | Local GGUF inference engine |
| v0.1–0.2 | hermes-ssd | Hermes dispatcher + SSD routing |
| **Current** | **hermes-ssd-llm** | Rust package `hermes-ssd-llm`, crate `hermes_ssd_llm` |

User-facing command remains **`hermes ssd`**. Normal **`hermes`** is unchanged.

## What was retained

- SSD volume UUID registration (`~/.config/hermes-ssd-llm/config.toml`)
- `hermes` dispatcher architecture
- SSD directory layout under `<mount>/Hermes-SSD-LLM/`
- Environment routing (`HERMES_HOME`, caches, `TMPDIR`, etc.)
- Local inference engine (`ssd/`, `metal/`, `inference/`)
- Install/uninstall scripts
- MIT license (Maxi Guillermo, 2026)

## What was removed or replaced

- Obsolete **BENCHMARKS.md** claims (M4 hardware, v1.39.0 estimates, llama.cpp comparison tables copied from upstream)
- Stale generated runtime state (via `hermes ssd reset`)
- Legacy config path `~/.config/hermes-ssd/` (auto-migrated to `hermes-ssd-llm`)
- Legacy SSD root `Hermes-SSD/` (still detected for backward compatibility on the same volume)

## Safe reset for existing users

Preview cleanup:

```bash
hermes ssd reset --dry-run
```

Default reset (preserves models, config, repositories, workspaces):

```bash
hermes ssd reset
```

Remove models too:

```bash
hermes ssd reset --include-models
```

Full project-managed SSD data:

```bash
hermes ssd reset --all-managed-data
```

**Always preserved on reset:** `~/.config/hermes-ssd-llm/config.toml` (volume UUID), Git history, source code.

**Note:** Credentials in SSD `HERMES_HOME` (`.env`, `auth.json`) are data on the external drive, not a separate Keychain-only store.

## Benchmark refresh (v0.3.0)

Run on your machine:

```bash
./scripts/capture-test-system.sh
./benchmarks/scripts/generate-report.sh
```

Committed results in `benchmarks/results/` are sanitized (no serial numbers, UUIDs, or home paths).

## Upstream attribution

The inference engine derives from the open-source [ssd-llm](https://github.com/redbasecap-buiss/ssd-llm) project. Hermes SSD LLM adds the Hermes dispatcher, SSD registration, environment routing, reset workflow, and benchmark tooling.
