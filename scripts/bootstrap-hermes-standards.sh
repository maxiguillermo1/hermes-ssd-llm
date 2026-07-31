#!/usr/bin/env bash
# bootstrap-hermes-standards.sh — Install .hermes/ engineering standards into a Git repository.
#
# Usage:
#   ./scripts/bootstrap-hermes-standards.sh /path/to/repo
#   ./scripts/bootstrap-hermes-standards.sh .          # current repo
#
# Copies policy files from this repository's .hermes/ directory.
# Skips files that already exist (use --force to overwrite).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SOURCE_HERMES="${SOURCE_ROOT}/.hermes"

FORCE=false
if [[ "${1:-}" == "--force" ]]; then
    FORCE=true
    shift
fi

TARGET="${1:-.}"
if [[ ! -d "${TARGET}" ]]; then
    echo "error: target directory does not exist: ${TARGET}" >&2
    exit 1
fi

TARGET="$(cd "${TARGET}" && pwd)"
TARGET_HERMES="${TARGET}/.hermes"

if [[ ! -d "${SOURCE_HERMES}" ]]; then
    echo "error: source .hermes/ not found at ${SOURCE_HERMES}" >&2
    exit 1
fi

echo "Bootstrapping Hermes standards into: ${TARGET}"

mkdir -p "${TARGET_HERMES}/templates"

copy_file() {
    local rel="$1"
    local src="${SOURCE_HERMES}/${rel}"
    local dest="${TARGET_HERMES}/${rel}"

    if [[ ! -f "${src}" ]]; then
        echo "  skip (missing source): ${rel}"
        return
    fi

    if [[ -f "${dest}" && "${FORCE}" != "true" ]]; then
        echo "  skip (exists): ${rel}"
        return
    fi

    mkdir -p "$(dirname "${dest}")"
    cp "${src}" "${dest}"
    echo "  installed: ${rel}"
}

# Core policy files
for f in README.md ENGINEERING_CONSTITUTION.md DOCUMENTATION_STANDARD.md PROJECT_STANDARDS.md ARCHITECTURE_PRINCIPLES.md; do
    copy_file "${f}"
done

# Templates
for f in templates/README_TEMPLATE.md templates/TECHNICAL_TEMPLATE.md; do
    copy_file "${f}"
done

# Customize PROJECT_STANDARDS.md placeholder if freshly installed
if [[ -f "${TARGET_HERMES}/PROJECT_STANDARDS.md" ]]; then
  repo_name="$(basename "${TARGET}")"
  if grep -q "Repository: hermes-ssd-llm" "${TARGET_HERMES}/PROJECT_STANDARDS.md" 2>/dev/null; then
    if [[ "${repo_name}" != "hermes-ssd-llm" ]]; then
      sed -i '' "s/Repository: hermes-ssd-llm/Repository: ${repo_name}/" "${TARGET_HERMES}/PROJECT_STANDARDS.md" 2>/dev/null \
        || sed -i "s/Repository: hermes-ssd-llm/Repository: ${repo_name}/" "${TARGET_HERMES}/PROJECT_STANDARDS.md"
      echo "  customized: PROJECT_STANDARDS.md (repo name → ${repo_name})"
    fi
  fi
fi

echo ""
echo "Done. Next steps for ${TARGET}:"
echo "  1. Customize .hermes/PROJECT_STANDARDS.md for this repository"
echo "  2. Customize .hermes/ARCHITECTURE_PRINCIPLES.md for this repository"
echo "  3. Audit and rewrite README.md per .hermes/DOCUMENTATION_STANDARD.md"
echo "  4. Create TECHNICAL.md if the project is non-trivial"
echo "  5. Reference .hermes/ in AGENTS.md or CONTRIBUTING.md"
