#!/bin/bash
# Restore canister state from backup
# This recovers the canister ID and EVM address

set -e

BACKUP_DIR="${HOME}/.provenanceai-backups"

if [ -z "$1" ]; then
    echo "📦 Available backups:"
    ls -1t "${BACKUP_DIR}" 2>/dev/null || echo "No backups found"
    echo ""
    echo "Usage: bash scripts/restore_canister_state.sh <timestamp>"
    echo "Example: bash scripts/restore_canister_state.sh 20251023_014500"
    exit 1
fi

TIMESTAMP=$1
BACKUP_PATH="${BACKUP_DIR}/dfx-state-${TIMESTAMP}"

if [ ! -d "${BACKUP_PATH}" ]; then
    echo "❌ Backup not found: ${BACKUP_PATH}"
    exit 1
fi

echo "🔓 Restoring canister state from ${TIMESTAMP}..."

# Stop dfx if running
echo "Stopping dfx..."
dfx stop 2>/dev/null || true

# Remove current .dfx
if [ -d ".dfx" ]; then
    echo "Removing current .dfx..."
    rm -rf .dfx
fi

# Restore from backup
echo "Restoring .dfx from backup..."
cp -r "${BACKUP_PATH}" .dfx

# Show canister info
if [ -f "${BACKUP_PATH}/CANISTER_INFO.txt" ]; then
    echo ""
    echo "✅ Restored canister:"
    cat "${BACKUP_PATH}/CANISTER_INFO.txt"
fi

echo ""
echo "✅ Restore complete!"
echo "Next steps:"
echo "  1. dfx start --background"
echo "  2. dfx canister call brain_canister get_canister_evm_address"
