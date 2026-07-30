# Hermes SSD LLM Benchmark Report
Generated: 2026-07-30T23:56:54Z
Version: 0.3.1
Commit: `4bc9a131cbfed46adc23571527e99e6e919a5e8a`
## Test system
- Mac: MacBook Air / Apple M2 / 8.0 GiB
- macOS: 26.2
- SSD: Extreme SSD / ExFAT / USB
- SSD available: 1832 GB

## Storage (measured)

| Test | Median |
|------|--------|
| sequential_write_mib_per_s | 1537.47 MiB/s |
| sequential_read_mib_per_s | 6741.26 MiB/s |
| small_file_create_100_ms | 73.28 ms |
| startup_validation_ms | 563.34 ms |

## Startup (measured)

| Test | Median |
|------|--------|
| hermes_version_ms | 250.0 ms |
| hermes_ssd_help_ms | 21.9 ms |
| hermes_ssd_doctor_ms | 556.2 ms |
| ssd_validation_overhead_ms | 0 ms |

## Memory (measured)

- hermes_version_max_rss_mib: 50.6
- hermes_ssd_doctor_max_rss_mib: 50.7
- swap_used_mb:   
- free_memory_mib_approx: 420.625

