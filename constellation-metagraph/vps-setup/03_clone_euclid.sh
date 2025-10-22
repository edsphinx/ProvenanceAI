#!/bin/bash
set -e

# ProvenanceAI Metagraph Deployment - Script 3: Clone Euclid
# Purpose: Clone Euclid Development Environment and setup Hydra CLI

echo "=========================================="
echo "Script 3: Clone Euclid Development Environment"
echo "=========================================="
echo ""

LOG_FILE="/root/provenanceai-metagraph/logs/03_clone_euclid.log"
exec > >(tee -a "$LOG_FILE") 2>&1

echo "Log file: $LOG_FILE"
echo "Started at: $(date)"
echo ""

# Clone Euclid Development Environment
echo "[1/3] Cloning Euclid Development Environment..."
cd /root/provenanceai-metagraph

if [ -d "euclid-development-environment" ]; then
    echo "Euclid already cloned, pulling latest..."
    cd euclid-development-environment
    git pull
else
    git clone https://github.com/Constellation-Labs/euclid-development-environment.git
    cd euclid-development-environment
fi

echo "Euclid cloned successfully"
echo ""

# Make Hydra scripts executable
echo "[2/3] Making Hydra scripts executable..."
chmod +x scripts/hydra
chmod +x scripts/hydra-*

# Add Hydra to PATH (optional)
if ! grep -q "euclid-development-environment/scripts" ~/.bashrc; then
    echo 'export PATH="$PATH:/root/provenanceai-metagraph/euclid-development-environment/scripts"' >> ~/.bashrc
    echo "Added Hydra to PATH"
fi

echo ""

# List available templates
echo "[3/3] Listing available Hydra templates..."
echo ""
scripts/hydra install-template --list || echo "Note: Templates will be available after first build"

echo ""
echo "=========================================="
echo "✅ Euclid Development Environment Ready!"
echo "=========================================="
echo ""
echo "Location: /root/provenanceai-metagraph/euclid-development-environment"
echo ""
echo "Hydra CLI commands:"
echo "- scripts/hydra install-template <name> - Install a template"
echo "- scripts/hydra build                    - Build containers"
echo "- scripts/hydra start-genesis            - Start local cluster"
echo "- scripts/hydra remote-deploy            - Deploy to remote (IntegrationNet)"
echo ""
echo "Next step: Run ./04_create_metagraph.sh"
echo ""
