#!/usr/bin/env bash
# Memory observations for Hermes launcher processes.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
RESULTS_DIR="${REPO_ROOT}/benchmarks/results"
mkdir -p "${RESULTS_DIR}"

TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
GIT_COMMIT="$(git -C "${REPO_ROOT}" rev-parse HEAD 2>/dev/null || echo unknown)"
PKG_VERSION="$(awk -F'"' '/^version = / {print $2; exit}' "${REPO_ROOT}/Cargo.toml")"

rss_kb_for() {
  local cmd="$1"
  /usr/bin/time -l ${cmd} >/dev/null 2>&1 || true
}

# time -l prints to stderr
measure_rss_bytes() {
  local cmd=("$@")
  local stderr
  stderr=$(/usr/bin/time -l "${cmd[@]}" 2>&1 >/dev/null || true)
  echo "${stderr}" | awk '/maximum resident set size/ {print $1; exit}'
}

HERMES_RSS_BYTES=$(measure_rss_bytes hermes --version)
SSD_DOCTOR_RSS_BYTES=$(measure_rss_bytes hermes ssd doctor)
HERMES_RSS_MIB=$(python3 -c "print(round(${HERMES_RSS_BYTES:-0}/1048576, 1))")
SSD_DOCTOR_RSS_MIB=$(python3 -c "print(round(${SSD_DOCTOR_RSS_BYTES:-0}/1048576, 1))")

SWAP_USED=$(sysctl -n vm.swapusage 2>/dev/null | awk -F'used = |M' '{print $2}' | head -1 || echo unavailable)
MEM_FREE_PAGES=$(vm_stat 2>/dev/null | awk '/Pages free/ {gsub(/\./,"",$3); print $3}' || echo unavailable)
PAGE_SIZE=$(sysctl -n hw.pagesize 2>/dev/null || echo 4096)
FREE_MIB=$(python3 -c "print(int(${MEM_FREE_PAGES:-0})*${PAGE_SIZE}/1048576)" 2>/dev/null || echo unavailable)

OUT="${RESULTS_DIR}/memory-benchmark.json"
cat >"${OUT}" <<EOF
{
  "schema_version": 1,
  "captured_at": "${TS}",
  "project_version": "${PKG_VERSION}",
  "git_commit": "${GIT_COMMIT}",
  "benchmark": "memory",
  "results": {
    "hermes_version_max_rss_mib": ${HERMES_RSS_MIB:-0},
    "hermes_ssd_doctor_max_rss_mib": ${SSD_DOCTOR_RSS_MIB:-0},
    "swap_used_mb": "${SWAP_USED}",
    "free_memory_mib_approx": "${FREE_MIB}"
  },
  "limitations": [
    "RSS from /usr/bin/time -l; short-lived processes only",
    "powermetrics not used (requires elevated permissions)"
  ]
}
EOF

echo "Memory benchmark → ${OUT}"
echo "  hermes --version RSS:      ${HERMES_RSS_MIB:-unavailable} MiB"
echo "  hermes ssd doctor RSS:     ${SSD_DOCTOR_RSS_MIB:-unavailable} MiB"
echo "  swap used:                 ${SWAP_USED} MB"
echo "  free memory (approx):      ${FREE_MIB} MiB"
