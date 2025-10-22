# Constellation Network Deployment Guide

**Project**: ProvenanceAI - Metagraph Deployment
**Network**: Constellation IntegrationNet
**Last Updated**: 2025-10-22

---

## 🎯 Overview

This directory contains automated deployment scripts for deploying the ProvenanceAI metagraph to Constellation Network's IntegrationNet. The metagraph implements a custom ProofOfGeneration data structure for tracking AI-generated content with blockchain provenance.

### What is a Metagraph?

A metagraph is a custom Layer 1 blockchain on Constellation Network that:
- Defines custom data structures (in our case: ProofOfGeneration)
- Implements custom validation logic
- Connects to Constellation's Global L0 consensus layer
- Provides custom API endpoints for data submission

### ProvenanceAI Architecture

Our metagraph has two layers:
1. **Data L1** (Port 9400): Accepts ProofOfGeneration submissions from ICP canister
2. **Metagraph L0** (Port 9200): Validates and snapshots data to Global L0

## 📋 Prerequisites

Before deploying, ensure you have:
- Ubuntu 20.04+ or Debian-based Linux server
- Root or sudo access
- Minimum 16GB RAM (recommended 32GB)
- 100GB+ available disk space
- Open ports: 9200, 9400
- Stable internet connection

## 🚀 Deployment Scripts

The deployment is automated through 7 sequential scripts. Each script is idempotent (safe to re-run) and creates detailed logs.

### Execution Order

Execute these scripts **in order**:

1. **01_system_setup.sh** - System dependencies and firewall
2. **02_install_tools.sh** - Development tools (Docker, Node.js, Scala, Rust)
3. **03_clone_euclid.sh** - Euclid Development Environment
4. **04_create_metagraph.sh** - ProvenanceAI metagraph structure
5. **05_build_metagraph.sh** - Scala compilation and Docker images
6. **06_deploy_integrationnet.sh** - Deploy to IntegrationNet
7. **07_verify_deployment.sh** - Test endpoints and submit data

### Quick Start

```bash
# Clone or navigate to deployment directory
cd packages/constellation/deployment/

# Make all scripts executable
chmod +x *.sh

# Run scripts one by one (check logs after each)
./01_system_setup.sh          # ~10 minutes
./02_install_tools.sh         # ~15 minutes
./03_clone_euclid.sh          # ~3 minutes
./04_create_metagraph.sh      # ~10 minutes
./05_build_metagraph.sh       # ~20 minutes
./06_deploy_integrationnet.sh # ~10 minutes
./07_verify_deployment.sh     # ~5 minutes
```

**Total deployment time**: ~1.5 hours

### Important Notes

1. **Sequential execution**: Each script depends on the previous one
2. **Check for errors**: Review logs before proceeding to next script
3. **Idempotent**: Safe to re-run scripts if they fail
4. **Logs**: All output saved to `/root/provenanceai-metagraph/logs/`

## 📊 What Each Script Does

### 01_system_setup.sh
- Updates system packages
- Installs base dependencies
- Configures firewall for ports 9200, 9400
- **Duration**: 5-10 minutes

### 02_install_tools.sh
- Installs Docker
- Installs Node.js 18+ and Yarn
- Installs JQ and YQ
- Installs Rust and Cargo
- Installs Scala and SBT
- **Duration**: 10-15 minutes

### 03_clone_euclid.sh
- Clones Euclid Development Environment
- Installs Hydra CLI
- Lists available templates
- **Duration**: 2-3 minutes

### 04_create_metagraph.sh
- Installs Custom NFT template
- Creates ProofOfGeneration data structure
- Implements validation logic
- Configures euclid.json
- **Duration**: 5-10 minutes

### 05_build_metagraph.sh
- Builds Scala code
- Creates Docker images
- **Duration**: 10-20 minutes (depending on VPS speed)

### 06_deploy_integrationnet.sh
- Configures remote deployment
- Deploys to IntegrationNet
- Starts metagraph nodes
- **Duration**: 5-10 minutes

### 07_verify_deployment.sh
- Tests Data L1 endpoint
- Tests Metagraph L0 endpoint
- Verifies IntegrationNet connection
- Submits test data
- **Duration**: 2-5 minutes

## 📝 Logging

Each script will create logs in `/root/provenanceai-metagraph/logs/`:

```
logs/
├── 01_system_setup.log
├── 02_install_tools.log
├── 03_clone_euclid.log
├── 04_create_metagraph.log
├── 05_build_metagraph.log
├── 06_deploy_integrationnet.log
└── 07_verify_deployment.log
```

## 🐛 Troubleshooting

If something fails:

1. **Check the log file** for the failed script
2. **Fix the error** mentioned in the log
3. **Re-run the script** - they're idempotent (safe to run multiple times)
4. **Ask for help** if you're stuck

Common issues:
- **Docker permission denied**: Run `newgrp docker` and retry
- **Port already in use**: Stop conflicting service or use different port
- **Out of memory**: VPS needs minimum 16GB RAM
- **Build timeout**: Increase timeout in script

## ✅ Success Criteria

After running all scripts, you should have:

- [ ] Docker running and accessible
- [ ] Euclid Development Environment cloned
- [ ] ProvenanceAI metagraph created
- [ ] Docker containers built successfully
- [ ] Metagraph deployed to IntegrationNet
- [ ] Data L1 endpoint responding at http://198.144.183.32:9400
- [ ] Metagraph L0 endpoint responding at http://198.144.183.32:9200
- [ ] Test data successfully submitted and verified

## 🔗 Important Endpoints

After successful deployment:

- **Data L1**: http://198.144.183.32:9400
  - POST endpoint: http://198.144.183.32:9400/data
  - Cluster info: http://198.144.183.32:9400/cluster/info

- **Metagraph L0**: http://198.144.183.32:9200
  - Cluster info: http://198.144.183.32:9200/cluster/info

- **IntegrationNet Explorer**: https://integrationnet.dagexplorer.io/

## 🎯 Deployment Success Criteria

After successful deployment, you should have:

- [ ] Docker running and accessible
- [ ] Euclid Development Environment installed
- [ ] ProvenanceAI metagraph compiled and containerized
- [ ] Data L1 endpoint responding: `http://<YOUR_IP>:9400`
- [ ] Metagraph L0 endpoint responding: `http://<YOUR_IP>:9200`
- [ ] Test ProofOfGeneration data submitted successfully
- [ ] Metagraph visible on [IntegrationNet Explorer](https://integrationnet.dagexplorer.io/)

## 📡 API Endpoints

After deployment, your metagraph will expose these endpoints:

### Data L1 (Port 9400)

```bash
# Submit ProofOfGeneration data
POST http://<YOUR_IP>:9400/data
Content-Type: application/json

{
  "value": {
    "ProofOfGeneration": {
      "contentHash": "0x...",
      "modelName": "deepseek-chat",
      "timestamp": 1729612800000,
      "storyIpId": "0x...",
      "nftContract": "0x...",
      "nftTokenId": 123,
      "generatorAddress": "ICP-Canister-xyz"
    }
  },
  "proofs": []
}

# Check cluster info
GET http://<YOUR_IP>:9400/cluster/info
```

### Metagraph L0 (Port 9200)

```bash
# Check cluster info
GET http://<YOUR_IP>:9200/cluster/info
```

## 🔗 Integration with ICP Canister

To integrate with the ICP canister (`packages/icp/src/constellation_util.rs`):

1. Update the `CONSTELLATION_DATA_L1_URL` constant to your Data L1 endpoint
2. The canister will automatically POST ProofOfGeneration data after each AI generation
3. Verify submissions on IntegrationNet Explorer

## 📚 Additional Documentation

- **Configuration files**: See `packages/constellation/config/`
- **Metagraph source code**: See `packages/constellation/metagraph/`
- **Euclid Documentation**: https://docs.constellationnetwork.io/euclid
- **IntegrationNet Explorer**: https://integrationnet.dagexplorer.io/

## 🐛 Common Issues & Solutions

See the troubleshooting section above for common deployment issues. For persistent problems:

1. Check logs in `/root/provenanceai-metagraph/logs/`
2. Verify all prerequisites are met (RAM, disk space, ports)
3. Ensure Docker has sufficient resources allocated
4. Check Constellation Network status (IntegrationNet may have maintenance windows)

---

**Deployment scripts tested on**: Ubuntu 22.04 LTS, 32GB RAM, 200GB SSD
