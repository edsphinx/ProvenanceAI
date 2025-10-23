#!/bin/bash
# Backup canister state to preserve canister ID and EVM address
# Run this BEFORE doing any 'dfx stop' or 'make clean'

set -e

BACKUP_DIR="${HOME}/.provenanceai-backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_PATH="${BACKUP_DIR}/dfx-state-${TIMESTAMP}"

echo "🔒 Backing up canister state..."
echo "Destination: ${BACKUP_PATH}"

# Create backup directory
mkdir -p "${BACKUP_DIR}"

# Backup .dfx directory
if [ -d ".dfx" ]; then
    cp -r .dfx "${BACKUP_PATH}"
    echo "✅ Backed up .dfx directory"
else
    echo "⚠️  No .dfx directory found"
    exit 1
fi

# Save canister ID to a text file for easy reference
if [ -f ".dfx/local/canister_ids.json" ]; then
    CANISTER_ID=$(cat .dfx/local/canister_ids.json | grep brain_canister -A 2 | grep local | cut -d'"' -f4)
    echo "Canister ID: ${CANISTER_ID}" > "${BACKUP_PATH}/CANISTER_INFO.txt"

    # Get EVM address if dfx is running
    EVM_ADDRESS=$(dfx canister call brain_canister get_canister_evm_address 2>/dev/null | tr -d '()\"' || echo "N/A")
    echo "EVM Address: ${EVM_ADDRESS}" >> "${BACKUP_PATH}/CANISTER_INFO.txt"
    echo "Backup Date: ${TIMESTAMP}" >> "${BACKUP_PATH}/CANISTER_INFO.txt"

    echo "✅ Saved canister info:"
    cat "${BACKUP_PATH}/CANISTER_INFO.txt"
fi

# Keep only last 10 backups
cd "${BACKUP_DIR}"
ls -t | tail -n +11 | xargs -r rm -rf
echo "🧹 Cleaned old backups (keeping last 10)"

echo ""
echo "✅ Backup complete!"
echo "To restore: bash scripts/restore_canister_state.sh ${TIMESTAMP}"
