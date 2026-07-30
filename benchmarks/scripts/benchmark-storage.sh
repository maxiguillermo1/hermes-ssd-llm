#!/usr/bin/env bash
# Sequential read/write and small-file benchmarks on the registered SSD.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
RESULTS_DIR="${REPO_ROOT}/benchmarks/results"
FIXTURES_DIR="${REPO_ROOT}/benchmarks/fixtures"
mkdir -p "${RESULTS_DIR}" "${FIXTURES_DIR}"

TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
GIT_COMMIT="$(git -C "${REPO_ROOT}" rev-parse HEAD 2>/dev/null || echo unknown)"
PKG_VERSION="$(awk -F'"' '/^version = / {print $2; exit}' "${REPO_ROOT}/Cargo.toml")"

# Resolve SSD tmp from doctor
SSD_TMP=""
if command -v hermes >/dev/null 2>&1; then
  SSD_TMP="$(hermes ssd doctor 2>/dev/null | awk -F'=' '/TMPDIR/ {print $2; exit}' | tr -d ' ' || true)"
fi
if [[ -z "${SSD_TMP}" || ! -d "${SSD_TMP}" ]]; then
  # fallback: find Hermes-SSD-LLM tmp on external volume
  for vol in /Volumes/*; do
    [[ -d "${vol}/Hermes-SSD-LLM/tmp" ]] && SSD_TMP="${vol}/Hermes-SSD-LLM/tmp" && break
    [[ -d "${vol}/Hermes-SSD/tmp" ]] && SSD_TMP="${vol}/Hermes-SSD/tmp" && break
  done
fi
[[ -n "${SSD_TMP}" && -d "${SSD_TMP}" ]] || { echo "error: SSD tmp not found — connect SSD and run hermes ssd doctor" >&2; exit 1; }

BENCH_DIR="${SSD_TMP}/benchmark-$$"
mkdir -p "${BENCH_DIR}"
trap 'rm -rf "${BENCH_DIR}"' EXIT INT TERM

RUNS=3
WARMUP=1
LARGE_MB=256
LARGE_BYTES=$((LARGE_MB * 1024 * 1024))

median() {
  python3 - "$@" <<'PY'
import sys, statistics
vals = [float(x) for x in sys.argv[1:] if x]
print(f"{statistics.median(vals):.2f}" if vals else "0")
PY
}

measure_seq_write_mbps() {
  local file="${BENCH_DIR}/seq-write.bin"
  local start end elapsed mbps
  dd if=/dev/zero of="${file}" bs=1m count="${LARGE_MB}" conv=sync 2>/dev/null
  start=$(python3 -c 'import time; print(time.time())')
  dd if=/dev/zero of="${file}" bs=1m count="${LARGE_MB}" conv=sync 2>/dev/null
  end=$(python3 -c 'import time; print(time.time())')
  elapsed=$(python3 -c "print(${end}-${start})")
  mbps=$(python3 -c "print((${LARGE_MB}*2)/${elapsed})")
  echo "${mbps}"
}

measure_seq_read_mbps() {
  local file="${BENCH_DIR}/seq-write.bin"
  local start end elapsed mbps
  [[ -f "${file}" ]] || dd if=/dev/zero of="${file}" bs=1m count="${LARGE_MB}" 2>/dev/null
  start=$(python3 -c 'import time; print(time.time())')
  dd if="${file}" of=/dev/null bs=1m 2>/dev/null
  end=$(python3 -c 'import time; print(time.time())')
  elapsed=$(python3 -c "print(${end}-${start})")
  mbps=$(python3 -c "print(${LARGE_MB}/${elapsed})")
  echo "${mbps}"
}

measure_small_files_ms() {
  local n=100 start end elapsed
  start=$(python3 -c 'import time; print(time.time())')
  for i in $(seq 1 "${n}"); do
    echo "x" >"${BENCH_DIR}/small-${i}.txt"
  done
  end=$(python3 -c 'import time; print(time.time())')
  elapsed=$(python3 -c "print((${end}-${start})*1000)")
  echo "${elapsed}"
}

measure_validation_ms() {
  local start end elapsed
  start=$(python3 -c 'import time; print(time.time())')
  hermes ssd doctor >/dev/null 2>&1
  end=$(python3 -c 'import time; print(time.time())')
  elapsed=$(python3 -c "print((${end}-${start})*1000)")
  echo "${elapsed}"
}

write_vals=()
read_vals=()
small_vals=()
valid_vals=()

for ((i=0; i<RUNS+WARMUP; i++)); do
  w=$(measure_seq_write_mbps)
  r=$(measure_seq_read_mbps)
  s=$(measure_small_files_ms)
  v=$(measure_validation_ms)
  if (( i >= WARMUP )); then
    write_vals+=("${w}")
    read_vals+=("${r}")
    small_vals+=("${s}")
    valid_vals+=("${v}")
  fi
done

SEQ_WRITE_MED=$(median "${write_vals[@]}")
SEQ_READ_MED=$(median "${read_vals[@]}")
SMALL_FILES_MED=$(median "${small_vals[@]}")
VALIDATION_MED=$(median "${valid_vals[@]}")

OUT="${RESULTS_DIR}/storage-benchmark.json"
cat >"${OUT}" <<EOF
{
  "schema_version": 1,
  "captured_at": "${TS}",
  "project_version": "${PKG_VERSION}",
  "git_commit": "${GIT_COMMIT}",
  "benchmark": "storage",
  "test_file_size_mib": ${LARGE_MB},
  "runs": ${RUNS},
  "warmup": ${WARMUP},
  "results": {
    "sequential_write_mib_per_s": {"median": ${SEQ_WRITE_MED}, "unit": "MiB/s"},
    "sequential_read_mib_per_s": {"median": ${SEQ_READ_MED}, "unit": "MiB/s"},
    "small_file_create_100_ms": {"median": ${SMALL_FILES_MED}, "unit": "ms"},
    "startup_validation_ms": {"median": ${VALIDATION_MED}, "unit": "ms"}
  },
  "limitations": [
    "ExFAT USB SSD; OS cache may affect warm runs",
    "Single-process dd; not multi-queue NVMe peak"
  ]
}
EOF

echo "Storage benchmark complete → ${OUT}"
echo "  sequential write: ${SEQ_WRITE_MED} MiB/s (median)"
echo "  sequential read:  ${SEQ_READ_MED} MiB/s (median)"
echo "  100 small files:  ${SMALL_FILES_MED} ms (median)"
echo "  doctor overhead:  ${VALIDATION_MED} ms (median)"
