# Benchmarks

All numbers on this page were **measured on the detected test system** (2026-07-30). They apply to that Mac and SanDisk SSD only.

> Hermes SSD LLM is primarily an SSD storage router for Hermes Agent. For **provider mode**, inference runs remotely; SSD mode moves Hermes state and caches off the internal drive. For **local LLM mode**, storing a GGUF on the SSD saves internal-drive space but does **not** reduce RAM required at inference time.

## Test system

| Field | Measured value |
|-------|----------------|
| Date | 2026-07-30 |
| Project version | 0.3.5 |
| Git commit | `4bc9a13` |
| Mac model | MacBook Air (Mac14,2) |
| Chip | Apple M2 (8 cores) |
| Unified memory | 8.0 GiB |
| macOS | 26.2 |
| Internal storage | 228.3 GiB total, 17.8 GiB free (at capture) |
| External SSD (reported name) | Extreme SSD |
| SSD filesystem | ExFAT |
| SSD connection | USB |
| SSD capacity | 1863 GB (decimal, formatted) |
| SSD available | 1832 GB (at capture) |
| Hermes | v0.19.0 |
| Hermes SSD LLM | 0.3.5 |
| Rust | 1.97.1 |

Full sanitized report: `benchmarks/results/current-system.md`  
Regenerate: `./scripts/capture-test-system.sh`

## Storage (sequential)

**Command:** `./benchmarks/scripts/benchmark-storage.sh`  
**Test file:** 256 MiB  
**Runs:** 3 (+ 1 warmup)

| Metric | Median | Min–max | Units |
|--------|--------|---------|-------|
| Sequential write | 1537.47 | — | MiB/s |
| Sequential read | 6741.26 | — | MiB/s |
| 100 small file creates | 73.28 | — | ms |
| Startup validation (doctor probe) | 563.34 | — | ms |

**Limitations:** ExFAT over USB; macOS cache may inflate read speeds on warm runs; single-threaded `dd`, not multi-queue peak.

## Hermes startup

**Command:** `./benchmarks/scripts/benchmark-startup.sh`  
**Runs:** 5 (+ 1 warmup)

| Test | Median | Units |
|------|--------|-------|
| `hermes --version` | 250.0 | ms |
| `hermes ssd --help` | 21.9 | ms |
| `hermes ssd doctor` | 556.2 | ms |

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
| `hermes --version` max RSS | ~50.6 MiB |
| `hermes ssd doctor` max RSS | ~50.7 MiB |
| Free memory (approx) | ~421 MiB |

Short-lived process RSS only; `powermetrics` not used (requires privileges).

## Local inference

Not run in this refresh (no GGUF model on SSD at benchmark time). The integrated `hermes-ssd-llm bench` CLI is **not shipped yet** (see [ROADMAP.md](ROADMAP.md)).

Developer micro-benchmarks:

```bash
cargo bench --bench inference_bench
```

For end-to-end local inference today, use `llama.cpp` or Ollama and document your own numbers.

## Remote providers

For Cursor, OpenAI, Anthropic, etc., only local startup and storage routing are relevant. Model computation remains remote.

## Full suite

```bash
./benchmarks/scripts/generate-report.sh
```

Outputs: `benchmarks/results/latest.json`, `latest.md`

## Removed claims

Inherited estimates from the upstream prototype (M4/16GB, v1.39.0 llama.cpp comparisons, ~9 t/s decode) were **removed**. This document lists only measurements from `benchmarks/results/*.json` on the test Mac.
