#!/usr/bin/env bash
# Hermes SSD LLM installer — idempotent, user-local (no sudo).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$SCRIPT_DIR"
INSTALL_BIN="${HOME}/.local/bin"
CONFIG_DIR="${HOME}/.config/hermes-ssd-llm"
LEGACY_CONFIG_DIR="${HOME}/.config/hermes-ssd"

log() { printf 'Hermes SSD LLM: %s\n' "$*"; }
die() { printf 'Hermes SSD LLM error: %s\n' "$*" >&2; exit 1; }

if [[ "$(uname -s)" != "Darwin" ]]; then
  die "Hermes SSD LLM requires macOS."
fi

if ! command -v cargo >/dev/null 2>&1; then
  if [[ -f "${HOME}/.cargo/env" ]]; then
    # shellcheck source=/dev/null
    source "${HOME}/.cargo/env"
  fi
fi
command -v cargo >/dev/null 2>&1 || die "Rust/cargo not found. Install from https://rustup.rs"

log "Building release binaries..."
(cd "$REPO_ROOT" && cargo build --release)

HERMES_WRAPPER="${REPO_ROOT}/target/release/hermes"
HERMES_SSD_LLM_BIN="${REPO_ROOT}/target/release/hermes-ssd-llm"
[[ -x "$HERMES_WRAPPER" ]] || die "build missing hermes wrapper"
[[ -x "$HERMES_SSD_LLM_BIN" ]] || die "build missing hermes-ssd-llm binary"

mkdir -p "$INSTALL_BIN" "$CONFIG_DIR"

# Migrate legacy hermes-ssd config if present
if [[ ! -f "${CONFIG_DIR}/config.toml" && -f "${LEGACY_CONFIG_DIR}/config.toml" ]]; then
  log "Migrating config from ${LEGACY_CONFIG_DIR}"
  cp -p "${LEGACY_CONFIG_DIR}/config.toml" "${CONFIG_DIR}/config.toml"
  chmod 600 "${CONFIG_DIR}/config.toml"
fi

is_real_hermes() {
  local candidate="$1"
  [[ -x "$candidate" && "$candidate" != "$HERMES_WRAPPER" ]] || return 1
  if file "$candidate" 2>/dev/null | grep -q "Mach-O"; then
    return 1
  fi
  return 0
}

# Locate real Hermes — never back up our own Rust wrapper.
CURRENT_HERMES=""
for candidate in \
  "${INSTALL_BIN}/hermes-real" \
  "${HOME}/Desktop/Hermes/hermes-agent/.venv/bin/hermes" \
  "/opt/homebrew/bin/hermes" \
  "/usr/local/bin/hermes"; do
  if is_real_hermes "$candidate"; then
    CURRENT_HERMES="$candidate"
    break
  fi
done

[[ -n "$CURRENT_HERMES" && -x "$CURRENT_HERMES" ]] || die "Could not find existing Hermes executable. Install Hermes Agent first."

REAL_BACKUP="${INSTALL_BIN}/hermes.real"
if [[ ! -e "$REAL_BACKUP" ]] || file "$REAL_BACKUP" 2>/dev/null | grep -q "Mach-O"; then
  log "Preserving real Hermes at ${REAL_BACKUP}"
  cp -p "$CURRENT_HERMES" "$REAL_BACKUP"
fi

log "Installing wrapper to ${INSTALL_BIN}/hermes"
cp -p "$HERMES_WRAPPER" "${INSTALL_BIN}/hermes"
cp -p "$HERMES_SSD_LLM_BIN" "${INSTALL_BIN}/hermes-ssd-llm"
chmod +x "${INSTALL_BIN}/hermes" "${INSTALL_BIN}/hermes-ssd-llm"

# Register SSD — prefer 2TB external volume
MOUNT=""
if [[ -d "/Volumes/Extreme SSD" ]]; then
  MOUNT="/Volumes/Extreme SSD"
else
  while IFS= read -r vol; do
    [[ -n "$vol" && -d "$vol" ]] || continue
  if diskutil info "$vol" 2>/dev/null | grep -q "Protocol: *USB"; then
      SIZE=$(diskutil info "$vol" 2>/dev/null | awk -F': ' '/Total Size/ {print $2}' | head -1)
      if echo "$SIZE" | grep -qE 'TB|2\.0'; then
        MOUNT="$vol"
        break
      fi
    fi
  done < <(ls "/Volumes" 2>/dev/null || true)
fi

CONFIG_FILE="${CONFIG_DIR}/config.toml"
if [[ ! -f "$CONFIG_FILE" ]]; then
  UUID=""
  VNAME=""
  if [[ -n "$MOUNT" ]]; then
    UUID=$(diskutil info "$MOUNT" 2>/dev/null | awk -F': ' '/Volume UUID/ {print $2}' | xargs)
    VNAME=$(basename "$MOUNT")
  fi
  cat >"$CONFIG_FILE" <<EOF
version = 1
volume_uuid = "${UUID}"
expected_volume_name = "${VNAME}"
expected_model = "SanDisk Extreme Portable SSD"
minimum_capacity_gb = 1800
minimum_free_space_gb = 100
minimum_write_space_gb = 20
require_external_device = true
allow_internal_fallback = false
hermes_executable = "${REAL_BACKUP}"
real_hermes_backup = "${REAL_BACKUP}"
logging_level = "info"
debug_startup = false
layer_prefetch_depth = 2
max_ram_target_gb = 8
ssd_kv_swap = true
EOF
  chmod 600 "$CONFIG_FILE"
  log "Wrote ${CONFIG_FILE}"
fi

if [[ -n "$MOUNT" ]]; then
  log "Registering SSD at ${MOUNT}"
  "${INSTALL_BIN}/hermes-ssd-llm" register "$MOUNT" || die "SSD registration failed"
else
  log "No external SSD detected — edit ${CONFIG_FILE} and run: hermes ssd doctor"
fi

# Save install state
STATE_FILE="${CONFIG_DIR}/install-state.json"
cat >"$STATE_FILE" <<EOF
{
  "real_hermes_path": "${REAL_BACKUP}",
  "wrapper_path": "${INSTALL_BIN}/hermes",
  "installed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF

log "Testing normal hermes pass-through..."
if ! "${INSTALL_BIN}/hermes" --version >/dev/null 2>&1; then
  die "hermes --version failed after install"
fi

log "Testing hermes ssd doctor..."
if ! "${INSTALL_BIN}/hermes" ssd doctor >/dev/null 2>&1; then
  log "Warning: hermes ssd doctor reported issues (SSD may be disconnected)"
fi

log "Installation complete."
log "  hermes          — normal Hermes (unchanged)"
log "  hermes ssd      — SSD-backed Hermes"
log "  hermes ssd doctor — diagnostics"
log "  hermes-ssd-llm  — management and inference utilities"
