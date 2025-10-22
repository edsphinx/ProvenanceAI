#!/bin/bash
set -e  # Exit on error

# ProvenanceAI Metagraph Deployment - Script 1: System Setup
# VPS: 198.144.183.32
# Purpose: Update system and install base dependencies

echo "=========================================="
echo "ProvenanceAI Metagraph Deployment"
echo "Script 1: System Setup"
echo "=========================================="
echo ""

# Create directories
echo "[1/6] Creating project directories..."
mkdir -p /root/provenanceai-metagraph/logs
cd /root/provenanceai-metagraph

# Log file
LOG_FILE="/root/provenanceai-metagraph/logs/01_system_setup.log"
exec > >(tee -a "$LOG_FILE") 2>&1

echo "Log file: $LOG_FILE"
echo "Started at: $(date)"
echo ""

# Check system info
echo "[2/6] Checking system information..."
echo "OS: $(cat /etc/os-release | grep PRETTY_NAME | cut -d'"' -f2)"
echo "Kernel: $(uname -r)"
echo "RAM: $(free -h | grep Mem | awk '{print $2}')"
echo "CPUs: $(nproc)"
echo "Disk: $(df -h / | tail -1 | awk '{print $2}')"
echo ""

# Check if we have enough resources
TOTAL_RAM=$(free -m | grep Mem | awk '{print $2}')
if [ "$TOTAL_RAM" -lt 15000 ]; then
    echo "⚠️  WARNING: VPS has less than 16GB RAM ($TOTAL_RAM MB)"
    echo "   Metagraph may have performance issues"
    echo ""
fi

# Update system
echo "[3/6] Updating system packages..."
apt update
apt upgrade -y

# Install base dependencies
echo "[4/6] Installing base dependencies..."
apt install -y \
    curl \
    wget \
    git \
    build-essential \
    ca-certificates \
    gnupg \
    lsb-release \
    software-properties-common \
    apt-transport-https \
    unzip \
    jq

# Configure firewall
echo "[5/6] Configuring firewall..."
if command -v ufw &> /dev/null; then
    echo "UFW firewall detected"
    ufw allow 22/tcp    # SSH
    ufw allow 9200/tcp  # Metagraph L0
    ufw allow 9400/tcp  # Data L1
    ufw allow 9300/tcp  # Currency L1 (optional)
    ufw allow 9000/tcp  # Global L0 (optional)
    echo "Firewall rules added (not enabling UFW to avoid SSH lockout)"
    echo "To enable: ufw enable"
else
    echo "UFW not found, skipping firewall configuration"
    echo "Manual firewall configuration may be needed"
fi
echo ""

# Set timezone
echo "[6/6] Setting timezone to UTC..."
timedatectl set-timezone UTC

echo ""
echo "=========================================="
echo "✅ System Setup Complete!"
echo "=========================================="
echo ""
echo "Summary:"
echo "- System packages updated"
echo "- Base dependencies installed"
echo "- Firewall configured (ports 9200, 9400 opened)"
echo "- Timezone set to UTC"
echo ""
echo "Next step: Run ./02_install_tools.sh"
echo ""
