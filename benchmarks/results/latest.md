# Hermes SSD LLM Benchmark Report
Generated: 2026-07-30T23:42:30Z
Version: 0.3.0
Commit: `f96b6af8937315e2cd9720c2fd6bf5c46e7413f9`
## Test system
- Mac: MacBook Air / Apple M2 / 8.0 GiB
- macOS: 26.2
- SSD: Extreme SSD / ExFAT / USB
- SSD available: 1834 GB

## Storage (measured)

| Test | Median |
|------|--------|
| sequential_write_mib_per_s | 1539.1 MiB/s |
| sequential_read_mib_per_s | 6675.71 MiB/s |
| small_file_create_100_ms | 73.42 ms |
| startup_validation_ms | 568.58 ms |

## Startup (measured)

| Test | Median |
|------|--------|
| hermes_version_ms | 247.2 ms |
| hermes_ssd_help_ms | 21.4 ms |
| hermes_ssd_doctor_ms | 541.5 ms |
| ssd_validation_overhead_ms | 0 ms |

## Memory (measured)

- hermes_version_max_rss_mib: 50.3
- hermes_ssd_doctor_max_rss_mib: 50.1
- swap_used_mb:   
- free_memory_mib_approx: 969.203125

