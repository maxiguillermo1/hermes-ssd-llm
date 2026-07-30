#!/usr/bin/env bash
# Run all benchmarks and merge into latest.json / latest.md
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
RESULTS_DIR="${REPO_ROOT}/benchmarks/results"

"${REPO_ROOT}/scripts/capture-test-system.sh"
"${SCRIPT_DIR}/benchmark-storage.sh"
"${SCRIPT_DIR}/benchmark-startup.sh"
"${SCRIPT_DIR}/benchmark-routing.sh"
"${SCRIPT_DIR}/benchmark-memory.sh"

TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
GIT_COMMIT="$(git -C "${REPO_ROOT}" rev-parse HEAD 2>/dev/null || echo unknown)"
PKG_VERSION="$(awk -F'"' '/^version = / {print $2; exit}' "${REPO_ROOT}/Cargo.toml")"

python3 - <<'PY' "${RESULTS_DIR}" "${TS}" "${PKG_VERSION}" "${GIT_COMMIT}"
import json, sys, pathlib
results_dir = pathlib.Path(sys.argv[1])
ts, ver, commit = sys.argv[2:5]
parts = {}
for name in ["current-system", "storage-benchmark", "startup-benchmark", "routing-benchmark", "memory-benchmark"]:
    p = results_dir / f"{name}.json"
    if p.exists():
        parts[name] = json.loads(p.read_text())
merged = {
    "schema_version": 1,
    "generated_at": ts,
    "project_version": ver,
    "git_commit": commit,
    "reports": parts,
}
(results_dir / "latest.json").write_text(json.dumps(merged, indent=2) + "\n")

md = [f"# Hermes SSD LLM Benchmark Report\n", f"Generated: {ts}\n", f"Version: {ver}\n", f"Commit: `{commit}`\n"]
sys_info = parts.get("current-system", {})
mac = sys_info.get("mac", {})
ext = sys_info.get("external_ssd", {})
md.append("## Test system\n")
md.append(f"- Mac: {mac.get('model_name','?')} / {mac.get('chip','?')} / {mac.get('unified_memory_gib','?')} GiB\n")
md.append(f"- macOS: {mac.get('macos_version','?')}\n")
md.append(f"- SSD: {ext.get('reported_model','?')} / {ext.get('filesystem_type','?')} / {ext.get('connection_protocol','?')}\n")
md.append(f"- SSD available: {ext.get('available_gb_decimal','?')} GB\n\n")

stor = parts.get("storage-benchmark", {}).get("results", {})
if stor:
    md.append("## Storage (measured)\n\n")
    md.append(f"| Test | Median |\n|------|--------|\n")
    for k,v in stor.items():
        md.append(f"| {k} | {v.get('median')} {v.get('unit','')} |\n")
    md.append("\n")

start = parts.get("startup-benchmark", {}).get("results", {})
if start:
    md.append("## Startup (measured)\n\n")
    md.append(f"| Test | Median |\n|------|--------|\n")
    for k,v in start.items():
        md.append(f"| {k} | {v.get('median')} {v.get('unit','')} |\n")
    md.append("\n")

mem = parts.get("memory-benchmark", {}).get("results", {})
if mem:
    md.append("## Memory (measured)\n\n")
    for k,v in mem.items():
        md.append(f"- {k}: {v}\n")
    md.append("\n")

(results_dir / "latest.md").write_text("".join(md))
print(f"Merged → {results_dir / 'latest.json'}")
print(f"Merged → {results_dir / 'latest.md'}")
PY
