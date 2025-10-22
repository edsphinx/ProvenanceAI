#!/bin/bash
set -e

# ProvenanceAI Metagraph Deployment - Script 2: Install Tools
# Purpose: Install Docker, Node.js, Rust, Scala, and other development tools

echo "=========================================="
echo "Script 2: Install Development Tools"
echo "=========================================="
echo ""

LOG_FILE="/root/provenanceai-metagraph/logs/02_install_tools.log"
exec > >(tee -a "$LOG_FILE") 2>&1

echo "Log file: $LOG_FILE"
echo "Started at: $(date)"
echo ""

# Install Docker
echo "[1/7] Installing Docker..."
if command -v docker &> /dev/null; then
    echo "Docker already installed: $(docker --version)"
else
    curl -fsSL https://get.docker.com -o get-docker.sh
    sh get-docker.sh
    rm get-docker.sh

    # Add root to docker group
    usermod -aG docker root

    # Start and enable Docker
    systemctl start docker
    systemctl enable docker

    echo "Docker installed: $(docker --version)"
fi

# Configure Docker memory
echo "Configuring Docker memory limits..."
mkdir -p /etc/docker
cat > /etc/docker/daemon.json <<EOF
{
  "default-ulimits": {
    "memlock": {
      "hard": 8589934592,
      "soft": 8589934592
    }
  }
}
EOF
systemctl restart docker
sleep 5
echo ""

# Install Node.js 18+
echo "[2/7] Installing Node.js 18..."
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    echo "Node.js already installed: $NODE_VERSION"
else
    curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
    apt install -y nodejs
    echo "Node.js installed: $(node --version)"
fi
echo ""

# Install Yarn
echo "[3/7] Installing Yarn..."
if command -v yarn &> /dev/null; then
    echo "Yarn already installed: $(yarn --version)"
else
    npm install -g yarn
    echo "Yarn installed: $(yarn --version)"
fi
echo ""

# Install JQ and YQ
echo "[4/7] Installing JQ and YQ..."
if command -v jq &> /dev/null; then
    echo "JQ already installed: $(jq --version)"
else
    apt install -y jq
fi

if command -v yq &> /dev/null; then
    echo "YQ already installed: $(yq --version)"
else
    wget https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 -O /usr/bin/yq
    chmod +x /usr/bin/yq
    echo "YQ installed: $(yq --version)"
fi
echo ""

# Install Rust and Cargo
echo "[5/7] Installing Rust and Cargo..."
if command -v cargo &> /dev/null; then
    echo "Rust already installed: $(rustc --version)"
else
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo 'source "$HOME/.cargo/env"' >> ~/.bashrc
    echo "Rust installed: $(rustc --version)"
fi
echo ""

# Install argc
echo "[6/7] Installing argc..."
source "$HOME/.cargo/env"
if command -v argc &> /dev/null; then
    echo "argc already installed: $(argc --version)"
else
    cargo install argc
    echo "argc installed: $(argc --version)"
fi
echo ""

# Install Scala and SBT
echo "[7/7] Installing Scala and SBT..."
if command -v scala &> /dev/null; then
    echo "Scala already installed: $(scala -version 2>&1 | head -1)"
else
    # Install Java (required for Scala)
    apt install -y openjdk-11-jdk

    # Install Scala
    wget https://downloads.lightbend.com/scala/2.13.12/scala-2.13.12.deb
    dpkg -i scala-2.13.12.deb
    rm scala-2.13.12.deb

    # Install SBT
    echo "deb https://repo.scala-sbt.org/scalasbt/debian all main" | tee /etc/apt/sources.list.d/sbt.list
    curl -sL "https://keyserver.ubuntu.com/pks/lookup?op=get&search=0x2EE0EA64E40A89B84B2DF73499E82A75642AC823" | apt-key add
    apt update
    apt install -y sbt

    echo "Scala installed: $(scala -version 2>&1 | head -1)"
    echo "SBT installed: $(sbt --version | head -1)"
fi
echo ""

# Verify all installations
echo "=========================================="
echo "✅ All Tools Installed!"
echo "=========================================="
echo ""
echo "Installed versions:"
echo "- Docker: $(docker --version)"
echo "- Node.js: $(node --version)"
echo "- Yarn: $(yarn --version)"
echo "- JQ: $(jq --version)"
echo "- YQ: $(yq --version)"
echo "- Rust: $(rustc --version)"
echo "- Cargo: $(cargo --version)"
echo "- argc: $(argc --version 2>&1 || echo 'installed')"
echo "- Java: $(java -version 2>&1 | head -1)"
echo "- Scala: $(scala -version 2>&1 | head -1)"
echo "- SBT: $(sbt --version | head -1)"
echo ""
echo "⚠️  IMPORTANT: You may need to run 'newgrp docker' to use Docker without sudo"
echo ""
echo "Next step: Run ./03_clone_euclid.sh"
echo ""
