# Benchmarks

All numbers on this page were **measured on the detected test system** (2026-07-30). They apply to that Mac and SanDisk SSD only.

> Hermes SSD LLM is not designed to beat in-RAM inference on tiny models. SSD mode reduces **internal-drive use** and **memory pressure** for large local models. Remote providers still run inference in the cloud.

## Test system

| Field | Measured value |
|-------|----------------|
| Date | 2026-07-30 |
| Project version | 0.3.0 |
| Git commit | `f96b6af` |
| Mac model | MacBook Air (Mac14,2) |
| Chip | Apple M2 (8 cores) |
| Unified memory | 8.0 GiB |
| macOS | 26.2 |
| Internal storage | 228.3 GiB total, 16.1 GiB free (at capture) |
| External SSD (reported name) | Extreme SSD |
| SSD filesystem | ExFAT |
| SSD connection | USB |
| SSD capacity | 1863 GB (decimal, formatted) |
| SSD available | 1834 GB (at capture) |
| Hermes | v0.19.0 |
| Hermes SSD LLM | 0.3.0 |
| Rust | 1.97.1 |

Full sanitized report: `benchmarks/results/current-system.md`  
Regenerate: `./scripts/capture-test-system.sh`

## Storage (sequential)

**Command:** `./benchmarks/scripts/benchmark-storage.sh`  
**Test file:** 256 MiB  
**Runs:** 3 (+ 1 warmup)

| Metric | Median | Min–max | Units |
|--------|--------|---------|-------|
| Sequential write | 1379.30 | — | MiB/s |
| Sequential read | 6710.34 | — | MiB/s |
| 100 small file creates | 75.22 | — | ms |
| Startup validation (doctor probe) | 531.29 | — | ms |

**Limitations:** ExFAT over USB; macOS cache may inflate read speeds on warm runs; single-threaded `dd`, not multi-queue peak.

## Hermes startup

**Command:** `./benchmarks/scripts/benchmark-startup.sh`  
**Runs:** 5 (+ 1 warmup)

| Test | Median | Units |
|------|--------|-------|
| `hermes --version` | 259.0 | ms |
| `hermes ssd --help` | 22.1 | ms |
| `hermes ssd doctor` | 532.7 | ms |

`hermes ssd` adds SSD verification before launching Hermes. Doctor includes full validation plus routing report.

## Storage routing (verified)

**Command:** `./benchmarks/scripts/benchmark-routing.sh`

All heavy paths resolved under `/Volumes/.../Hermes-SSD-LLM/`:

| Variable | On external SSD |
|----------|-----------------|
| `HERMES_HOME` | yes |
| `TMPDIR` | yes |
| `HF_HOME` | yes |
| `CARGO_TARGET_DIR` | yes |
| `HERMES_SSD_LLM_MODELS` | yes |
| `HERMES_SSD_LLM_LOG_DIR` | yes |

## Memory snapshot

**Command:** `./benchmarks/scripts/benchmark-memory.sh`

| Metric | Value |
|--------|-------|
| `hermes --version` max RSS | ~50.5 MiB |
| `hermes ssd doctor` max RSS | ~49.9 MiB |
| Free memory (approx) | ~798 MiB |

Short-lived process RSS only; `powermetrics` not used (requires privileges).

## Local inference

Not run in this refresh (no GGUF model on SSD at benchmark time). When a model is available:

```bash
hermes-ssd-llm bench /path/to/model.gguf --json
```

## Remote providers

For Cursor, OpenAI, Anthropic, etc., only local startup and storage routing are relevant. Model computation remains remote.

## Full suite

```bash
./benchmarks/scripts/generate-report.sh
```

Outputs: `benchmarks/results/latest.json`, `latest.md`

## Removed claims

Inherited estimates from the upstream prototype (M4/16GB, v1.39.0 llama.cpp comparisons, ~9 t/s decode) were **removed**. This document lists only measurements from `benchmarks/results/*.json` on the test Mac.
