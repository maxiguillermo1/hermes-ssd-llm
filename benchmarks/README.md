# Hermes SSD LLM — Benchmarks

Measured benchmarks for the **detected test system only**. Do not treat these numbers as universal.

## Run all benchmarks

```bash
./benchmarks/scripts/generate-report.sh
```

This produces:

- `benchmarks/results/current-system.json` — sanitized hardware report (committed)
- `benchmarks/results/current-system.md` — human-readable hardware report (committed)
- `benchmarks/results/latest.json` — merged benchmark results (committed)
- `benchmarks/results/latest.md` — summary markdown (committed)
- `benchmarks/results/private/` — raw captures with redacted identifiers (gitignored)

## Individual scripts

| Script | Measures |
|--------|----------|
| `scripts/capture-test-system.sh` | Mac + SSD + toolchain (sanitized) |
| `benchmarks/scripts/benchmark-storage.sh` | Sequential read/write, small files, doctor overhead |
| `benchmarks/scripts/benchmark-startup.sh` | `hermes` vs `hermes ssd` startup |
| `benchmarks/scripts/benchmark-routing.sh` | Env path routing to external SSD |
| `benchmarks/scripts/benchmark-memory.sh` | Launcher RSS, swap, free memory |

## Requirements

- Registered external SSD connected
- `hermes` and `hermes ssd doctor` working
- macOS with `dd`, `python3`, `time`

## Local inference benchmarks

The integrated `hermes-ssd-llm bench` CLI is **not shipped yet** (see [ROADMAP.md](../ROADMAP.md)).

Developer Criterion bench:

```bash
cargo bench --bench inference_bench
```

Local inference throughput numbers are **not** included in committed `BENCHMARKS.md` unless you run and document them separately with model file, quantization, and settings.
