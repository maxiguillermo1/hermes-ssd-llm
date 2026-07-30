#!/usr/bin/env bash
# Hermes SSD LLM uninstaller — restores original hermes wrapper, keeps user data.
set -euo pipefail

INSTALL_BIN="${HOME}/.local/bin"
REAL_BACKUP="${INSTALL_BIN}/hermes.real"
WRAPPER="${INSTALL_BIN}/hermes"

log() { printf 'Hermes SSD LLM: %s\n' "$*"; }
die() { printf 'Hermes SSD LLM error: %s\n' "$*" >&2; exit 1; }

if [[ ! -x "$REAL_BACKUP" ]]; then
  die "No preserved Hermes at ${REAL_BACKUP} — nothing to restore."
fi

log "Restoring original Hermes executable..."
cp -p "$REAL_BACKUP" "$WRAPPER"
chmod +x "$WRAPPER"

if [[ -x "${INSTALL_BIN}/hermes-ssd-llm" ]]; then
  rm -f "${INSTALL_BIN}/hermes-ssd-llm"
  log "Removed hermes-ssd-llm binary"
fi

# Remove legacy binary name if present
if [[ -x "${INSTALL_BIN}/hermes-ssd" ]]; then
  rm -f "${INSTALL_BIN}/hermes-ssd"
  log "Removed legacy hermes-ssd binary"
fi

log "Uninstall complete. User data preserved:"
log "  ${HOME}/.config/hermes-ssd-llm/"
log "  External SSD Hermes-SSD-LLM/ directory (if created)"
log "To remove config: rm -rf ${HOME}/.config/hermes-ssd-llm"
