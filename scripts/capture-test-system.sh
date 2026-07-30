#!/usr/bin/env bash
# Capture sanitized hardware + toolchain report for Hermes SSD LLM benchmarks.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
RESULTS_DIR="${REPO_ROOT}/benchmarks/results"
PRIVATE_DIR="${RESULTS_DIR}/private"
mkdir -p "${RESULTS_DIR}" "${PRIVATE_DIR}"

TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
GIT_COMMIT="$(git -C "${REPO_ROOT}" rev-parse HEAD 2>/dev/null || echo unknown)"
PKG_VERSION="$(awk -F'"' '/^version = / {print $2; exit}' "${REPO_ROOT}/Cargo.toml")"

# Raw capture (gitignored)
RAW="${PRIVATE_DIR}/raw-$(date -u +%Y%m%d-%H%M%S).txt"
{
  echo "timestamp=${TS}"
  system_profiler SPHardwareDataType 2>/dev/null || true
  system_profiler SPStorageDataType 2>/dev/null || true
  system_profiler SPUSBDataType 2>/dev/null || true
  sw_vers 2>/dev/null || true
  uname -m
  sysctl -n hw.memsize 2>/dev/null || true
  sysctl -n machdep.cpu.brand_string 2>/dev/null || true
  rustc --version 2>/dev/null || true
  cargo --version 2>/dev/null || true
  hermes --version 2>/dev/null || true
  diskutil list external 2>/dev/null || true
} >"${RAW}"

# Helpers
gb_from_bytes() {
  awk -v b="$1" 'BEGIN { printf "%.1f", b/1000000000 }'
}
gib_from_bytes() {
  awk -v b="$1" 'BEGIN { printf "%.1f", b/1073741824 }'
}

MEM_BYTES="$(sysctl -n hw.memsize 2>/dev/null || echo 0)"
MAC_MODEL="$(system_profiler SPHardwareDataType 2>/dev/null | awk -F': ' '/Model Name/ {print $2; exit}')"
MAC_ID="$(system_profiler SPHardwareDataType 2>/dev/null | awk -F': ' '/Model Identifier/ {print $2; exit}')"
CHIP="$(system_profiler SPHardwareDataType 2>/dev/null | awk -F': ' '/Chip/ {print $2; exit}')"
CPU_CORES="$(system_profiler SPHardwareDataType 2>/dev/null | awk -F': ' '/Total Number of Cores/ {print $2; exit}')"
MACOS_VER="$(sw_vers -productVersion 2>/dev/null || echo unknown)"
ARCH="$(uname -m)"

# Internal disk (boot volume)
INTERNAL_TOTAL="$(df -k / 2>/dev/null | awk 'NR==2 {print $2*1024}')"
INTERNAL_FREE="$(df -k / 2>/dev/null | awk 'NR==2 {print $4*1024}')"
INTERNAL_FS="$(df -T / 2>/dev/null | awk 'NR==2 {print $2}' || diskutil info / 2>/dev/null | awk -F': ' '/File System Personality/ {print $2; exit}')"

# External SSD from hermes ssd doctor if available
SSD_MODEL="unavailable"
SSD_FS="unavailable"
SSD_PROTOCOL="unavailable"
SSD_TOTAL="unavailable"
SSD_FREE="unavailable"
SSD_CAPACITY_GB="unavailable"

if command -v hermes >/dev/null 2>&1; then
  DOCTOR="$(hermes ssd doctor 2>/dev/null || true)"
  SSD_FS="$(echo "${DOCTOR}" | awk -F': ' '/^Filesystem:/ {gsub(/^ +/,"",$2); print $2; exit}')"
  SSD_PROTOCOL="$(echo "${DOCTOR}" | awk -F': ' '/^Protocol:/ {gsub(/^ +/,"",$2); print $2; exit}')"
  SSD_TOTAL="$(echo "${DOCTOR}" | awk -F': ' '/^Total capacity:/ {gsub(/[^0-9.]/,"",$2); print $2; exit}')"
  SSD_FREE="$(echo "${DOCTOR}" | awk -F': ' '/^Available:/ {gsub(/[^0-9.]/,"",$2); print $2; exit}')"
  VOL_NAME="$(echo "${DOCTOR}" | awk -F': ' '/^Volume name:/ {gsub(/^ +/,"",$2); print $2; exit}')"
  if [[ -n "${VOL_NAME}" ]]; then
  SSD_MODEL="${VOL_NAME}"
  fi
fi

# Try USB product name
USB_PRODUCT="$(system_profiler SPUSBDataType 2>/dev/null | awk '/SanDisk|Portable SSD|Extreme/ {found=1} found && /Product ID/ {getline; print; exit}' || true)"
if [[ -z "${USB_PRODUCT}" ]]; then
  USB_PRODUCT="$(system_profiler SPUSBDataType 2>/dev/null | grep -A2 'SanDisk' | grep 'Manufacturer' | head -1 | awk -F': ' '{print $2}' || true)"
fi
if [[ "${SSD_MODEL}" == "unavailable" && -n "${USB_PRODUCT}" ]]; then
  SSD_MODEL="SanDisk Portable SSD"
fi

HERMES_VER="$(hermes --version 2>/dev/null | head -1 || echo unavailable)"
RUST_VER="$(rustc --version 2>/dev/null || echo unavailable)"
CARGO_VER="$(cargo --version 2>/dev/null || echo unavailable)"
POWER="$(pmset -g batt 2>/dev/null | awk '/AC Power/ {print "AC"; exit} /Battery Power/ {print "battery"; exit}' || echo unavailable)"

JSON_OUT="${RESULTS_DIR}/current-system.json"
MD_OUT="${RESULTS_DIR}/current-system.md"

cat >"${JSON_OUT}" <<EOF
{
  "schema_version": 1,
  "captured_at": "${TS}",
  "project": "hermes-ssd-llm",
  "project_version": "${PKG_VERSION}",
  "git_commit": "${GIT_COMMIT}",
  "mac": {
    "model_name": "${MAC_MODEL:-unavailable}",
    "model_identifier": "${MAC_ID:-unavailable}",
    "chip": "${CHIP:-unavailable}",
    "cpu_cores": "${CPU_CORES:-unavailable}",
    "unified_memory_gib": $(gib_from_bytes "${MEM_BYTES}"),
    "architecture": "${ARCH}",
    "macos_version": "${MACOS_VER}"
  },
  "internal_storage": {
    "filesystem": "${INTERNAL_FS:-unavailable}",
    "total_gib": $(gib_from_bytes "${INTERNAL_TOTAL:-0}"),
    "available_gib": $(gib_from_bytes "${INTERNAL_FREE:-0}")
  },
  "external_ssd": {
    "manufacturer": "SanDisk",
    "reported_model": "${SSD_MODEL}",
    "filesystem_type": "${SSD_FS:-unavailable}",
    "connection_protocol": "${SSD_PROTOCOL:-unavailable}",
    "total_gb_decimal": "${SSD_TOTAL}",
    "available_gb_decimal": "${SSD_FREE}",
    "volume_uuid": "REDACTED"
  },
  "toolchain": {
    "hermes_version": $(printf '%s' "${HERMES_VER}" | python3 -c 'import json,sys; print(json.dumps(sys.stdin.read().strip()))'),
    "hermes_ssd_llm_version": "${PKG_VERSION}",
    "rust_version": $(printf '%s' "${RUST_VER}" | python3 -c 'import json,sys; print(json.dumps(sys.stdin.read().strip()))'),
    "cargo_version": $(printf '%s' "${CARGO_VER}" | python3 -c 'import json,sys; print(json.dumps(sys.stdin.read().strip()))')
  },
  "test_conditions": {
    "power_source": "${POWER}",
    "thermal_state": "unavailable"
  }
}
EOF

cat >"${MD_OUT}" <<EOF
# Hermes SSD LLM — Test System Report

Captured: ${TS}
Project version: ${PKG_VERSION}
Git commit: \`${GIT_COMMIT}\`

> Results apply to this detected test system only.

## Mac

| Field | Value |
|-------|-------|
| Model | ${MAC_MODEL:-unavailable} |
| Identifier | ${MAC_ID:-unavailable} |
| Chip | ${CHIP:-unavailable} |
| CPU cores | ${CPU_CORES:-unavailable} |
| Unified memory | $(gib_from_bytes "${MEM_BYTES}") GiB |
| macOS | ${MACOS_VER} |
| Architecture | ${ARCH} |

## Internal storage

| Field | Value |
|-------|-------|
| Filesystem | ${INTERNAL_FS:-unavailable} |
| Total | $(gib_from_bytes "${INTERNAL_TOTAL:-0}") GiB |
| Available (at capture) | $(gib_from_bytes "${INTERNAL_FREE:-0}") GiB |

## External SSD

| Field | Value |
|-------|-------|
| Manufacturer | SanDisk |
| Reported model | ${SSD_MODEL} |
| Filesystem | ${SSD_FS:-unavailable} |
| Connection | ${SSD_PROTOCOL:-unavailable} |
| Total capacity | ${SSD_TOTAL} GB (decimal, from doctor) |
| Available (at capture) | ${SSD_FREE} GB (decimal, from doctor) |
| Volume UUID | REDACTED |

## Toolchain

| Tool | Version |
|------|---------|
| Hermes | ${HERMES_VER} |
| Hermes SSD LLM | ${PKG_VERSION} |
| Rust | ${RUST_VER} |
| Cargo | ${CARGO_VER} |

## Conditions

- Power: ${POWER}
- Thermal state: unavailable (not measured)
EOF

echo "Wrote ${JSON_OUT}"
echo "Wrote ${MD_OUT}"
echo "Raw capture (private): ${RAW}"
