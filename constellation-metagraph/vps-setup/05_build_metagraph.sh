#!/bin/bash
set -e

# ProvenanceAI Metagraph Deployment - Script 5: Build Metagraph
# Purpose: Build Scala code and create Docker containers

echo "=========================================="
echo "Script 5: Build ProvenanceAI Metagraph"
echo "=========================================="
echo ""

LOG_FILE="/root/provenanceai-metagraph/logs/05_build_metagraph.log"
exec > >(tee -a "$LOG_FILE") 2>&1

echo "Log file: $LOG_FILE"
echo "Started at: $(date)"
echo ""

cd /root/provenanceai-metagraph/euclid-development-environment

# Build metagraph
echo "[1/2] Building metagraph (this may take 10-20 minutes)..."
echo "Building Scala code and Docker images..."
echo ""

# Run Hydra build
scripts/hydra build

echo ""
echo "Build complete!"
echo ""

# Verify Docker images
echo "[2/2] Verifying Docker images..."
docker images | grep provenanceai || docker images | head -10

echo ""
echo "=========================================="
echo "✅ Metagraph Build Complete!"
echo "=========================================="
echo ""
echo "Docker containers are ready for deployment"
echo ""
echo "Next step: Run ./06_deploy_integrationnet.sh"
echo ""
