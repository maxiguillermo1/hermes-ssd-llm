#!/usr/bin/env bash
# Measure Hermes startup overhead: normal vs SSD mode vs doctor.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
RESULTS_DIR="${REPO_ROOT}/benchmarks/results"
mkdir -p "${RESULTS_DIR}"

TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
GIT_COMMIT="$(git -C "${REPO_ROOT}" rev-parse HEAD 2>/dev/null || echo unknown)"
PKG_VERSION="$(awk -F'"' '/^version = / {print $2; exit}' "${REPO_ROOT}/Cargo.toml")"

RUNS=5
WARMUP=1

median() {
  python3 - "$@" <<'PY'
import sys, statistics
vals = [float(x) for x in sys.argv[1:] if x]
print(f"{statistics.median(vals):.1f}" if vals else "0")
PY
}

time_ms() {
  local start end
  start=$(python3 -c 'import time; print(time.time())')
  "$@" >/dev/null 2>&1
  end=$(python3 -c 'import time; print(time.time())')
  python3 -c "print((${end}-${start})*1000)"
}

hermes_vals=()
ssd_help_vals=()
doctor_vals=()

for ((i=0; i<RUNS+WARMUP; i++)); do
  h=$(time_ms hermes --version)
  s=$(time_ms hermes ssd --help)
  d=$(time_ms hermes ssd doctor)
  if (( i >= WARMUP )); then
    hermes_vals+=("${h}")
    ssd_help_vals+=("${s}")
    doctor_vals+=("${d}")
  fi
done

HERMES_MED=$(median "${hermes_vals[@]}")
SSD_HELP_MED=$(median "${ssd_help_vals[@]}")
DOCTOR_MED=$(median "${doctor_vals[@]}")
OVERHEAD=$(python3 -c "print(max(0, ${SSD_HELP_MED}-${HERMES_MED}))")

OUT="${RESULTS_DIR}/startup-benchmark.json"
cat >"${OUT}" <<EOF
{
  "schema_version": 1,
  "captured_at": "${TS}",
  "project_version": "${PKG_VERSION}",
  "git_commit": "${GIT_COMMIT}",
  "benchmark": "startup",
  "runs": ${RUNS},
  "warmup": ${WARMUP},
  "results": {
    "hermes_version_ms": {"median": ${HERMES_MED}, "unit": "ms"},
    "hermes_ssd_help_ms": {"median": ${SSD_HELP_MED}, "unit": "ms"},
    "hermes_ssd_doctor_ms": {"median": ${DOCTOR_MED}, "unit": "ms"},
    "ssd_validation_overhead_ms": {"median": ${OVERHEAD}, "unit": "ms"}
  },
  "notes": "hermes ssd --help includes SSD verify + prints Hermes help; not full TUI launch"
}
EOF

echo "Startup benchmark → ${OUT}"
echo "  hermes --version:     ${HERMES_MED} ms"
echo "  hermes ssd --help:    ${SSD_HELP_MED} ms"
echo "  hermes ssd doctor:    ${DOCTOR_MED} ms"
echo "  SSD overhead (est.):  ${OVERHEAD} ms"
