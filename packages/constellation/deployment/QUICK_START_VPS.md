# Quick Start Guide - VPS Deployment

**Project**: ProvenanceAI Constellation Metagraph
**Target**: IntegrationNet
**Last Updated**: 2025-10-22

---

## 🎯 Purpose

This guide provides a rapid deployment path for setting up the ProvenanceAI metagraph on a fresh VPS or server.

---

## 📁 Deployment Files

The deployment scripts are located in this directory:

```
packages/constellation/deployment/
├── README.md                       # Full deployment guide
├── QUICK_START_VPS.md              # This file
├── 01_system_setup.sh              # System dependencies
├── 02_install_tools.sh             # Development tools
├── 03_clone_euclid.sh              # Euclid framework
├── 04_create_metagraph.sh          # Metagraph structure
├── 05_build_metagraph.sh           # Build containers
├── 06_deploy_integrationnet.sh     # Deploy to network
└── 07_verify_deployment.sh         # Verify deployment
```

---

## 🚀 Quick Execution

Copy these scripts to your VPS and execute in order:

```bash
# Navigate to deployment directory
cd packages/constellation/deployment/

# Make scripts executable
chmod +x *.sh

# Execute in order (one at a time, check logs after each)
./01_system_setup.sh          # ~10 mins - System setup
./02_install_tools.sh         # ~15 mins - Install Docker, Node, Scala, Rust
./03_clone_euclid.sh          # ~3 mins  - Clone Euclid framework
./04_create_metagraph.sh      # ~10 mins - Create metagraph structure
./05_build_metagraph.sh       # ~20 mins - Build Docker images
./06_deploy_integrationnet.sh # ~10 mins - Deploy to IntegrationNet
./07_verify_deployment.sh     # ~5 mins  - Test and verify
```

**Total Time**: ~1.5 hours

---

## 📊 Monitoring Progress

Each script creates a log file in `/root/provenanceai-metagraph/logs/`:

```bash
# View logs in real-time
tail -f /root/provenanceai-metagraph/logs/XX_scriptname.log

# Check last 50 lines of most recent log
ls -t /root/provenanceai-metagraph/logs/*.log | head -1 | xargs tail -50
```

---

## ✅ Success Criteria

When deployment is complete, you should have:

- ✅ Metagraph running on IntegrationNet
- ✅ Data L1 endpoint: `http://<YOUR_IP>:9400`
- ✅ Metagraph L0 endpoint: `http://<YOUR_IP>:9200`
- ✅ Test ProofOfGeneration data submitted successfully
- ✅ Metagraph visible on [IntegrationNet Explorer](https://integrationnet.dagexplorer.io/)

### Quick Health Check

```bash
# Test Data L1
curl http://localhost:9400/cluster/info | jq '.'

# Test Metagraph L0
curl http://localhost:9200/cluster/info | jq '.'
```

---

## 🐛 Troubleshooting

If a script fails:

1. **Check the log file**: `/root/provenanceai-metagraph/logs/XX_scriptname.log`
2. **Review the error**: Look for ERROR or FAILED messages
3. **Fix the issue**: Common issues listed in main README.md
4. **Re-run the script**: All scripts are idempotent (safe to re-run)

Common issues:
- **Docker permission denied**: Run `newgrp docker` and retry
- **Port already in use**: Stop conflicting service or use different port
- **Out of memory**: VPS needs minimum 16GB RAM
- **Build timeout**: Increase timeout in script or allocate more CPU

---

## 📞 Key Information

- **Network**: Constellation IntegrationNet
- **Required Ports**: 9200 (Metagraph L0), 9400 (Data L1)
- **Minimum Resources**: 16GB RAM, 100GB disk
- **Estimated Duration**: 1.5 hours
- **Explorer**: https://integrationnet.dagexplorer.io/

---

## 🔗 Integration

After successful deployment:

1. **Update ICP Canister**: Modify `packages/icp/src/constellation_util.rs`
   - Set `CONSTELLATION_DATA_L1_URL` to `http://<YOUR_IP>:9400/data`

2. **Test Integration**: Deploy ICP canister and generate content
   - Canister will POST ProofOfGeneration data to your Data L1

3. **Verify on Explorer**: Check IntegrationNet Explorer for your submissions

---

**For detailed information, see [README.md](./README.md)**
