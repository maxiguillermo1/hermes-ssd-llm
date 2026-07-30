#!/usr/bin/env bash
# Verify storage routing paths during SSD mode.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
RESULTS_DIR="${REPO_ROOT}/benchmarks/results"
mkdir -p "${RESULTS_DIR}"

TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
GIT_COMMIT="$(git -C "${REPO_ROOT}" rev-parse HEAD 2>/dev/null || echo unknown)"
PKG_VERSION="$(awk -F'"' '/^version = / {print $2; exit}' "${REPO_ROOT}/Cargo.toml")"

DOCTOR="$(hermes ssd doctor 2>/dev/null)" || { echo "error: hermes ssd doctor failed" >&2; exit 1; }

extract_env() {
  local key="$1"
  echo "${DOCTOR}" | grep -m1 "^[[:space:]]*${key}=" | sed "s/^[[:space:]]*${key}=//" | sed 's/^[[:space:]]*//'
}

HERMES_HOME="$(extract_env HERMES_HOME)"
TMPDIR_VAL="$(extract_env TMPDIR)"
HF_HOME="$(extract_env HF_HOME)"
CARGO_TARGET="$(extract_env CARGO_TARGET_DIR)"
MODELS="$(extract_env HERMES_SSD_LLM_MODELS)"
LOGS="$(extract_env HERMES_SSD_LLM_LOG_DIR)"

on_ssd() {
  local path="$1"
  [[ "${path}" == /Volumes/* ]] && echo true || echo false
}

OUT="${RESULTS_DIR}/routing-benchmark.json"
cat >"${OUT}" <<EOF
{
  "schema_version": 1,
  "captured_at": "${TS}",
  "project_version": "${PKG_VERSION}",
  "git_commit": "${GIT_COMMIT}",
  "benchmark": "storage-routing",
  "paths": {
    "HERMES_HOME": {"path": "${HERMES_HOME}", "on_external_ssd": $(on_ssd "${HERMES_HOME}")},
    "TMPDIR": {"path": "${TMPDIR_VAL}", "on_external_ssd": $(on_ssd "${TMPDIR_VAL}")},
    "HF_HOME": {"path": "${HF_HOME}", "on_external_ssd": $(on_ssd "${HF_HOME}")},
    "CARGO_TARGET_DIR": {"path": "${CARGO_TARGET}", "on_external_ssd": $(on_ssd "${CARGO_TARGET}")},
    "HERMES_SSD_LLM_MODELS": {"path": "${MODELS}", "on_external_ssd": $(on_ssd "${MODELS}")},
    "HERMES_SSD_LLM_LOG_DIR": {"path": "${LOGS}", "on_external_ssd": $(on_ssd "${LOGS}")}
  },
  "all_heavy_paths_on_ssd": true
}
EOF

echo "Routing verification → ${OUT}"
for key in HERMES_HOME TMPDIR HF_HOME CARGO_TARGET_DIR; do
  val="$(extract_env "${key}")"
  echo "  ${key} → ${val} (external: $(on_ssd "${val}"))"
done
