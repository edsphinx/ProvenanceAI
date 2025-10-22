# Constellation Metagraph - ProvenanceAI

**Purpose**: Deploy ProvenanceAI metagraph to Constellation Network IntegrationNet
**VPS**: 198.144.183.32
**Network**: IntegrationNet (Testnet)

---

## 📁 Directory Structure

```
constellation-metagraph/
├── README.md                          # This file
├── .env.vps                           # VPS credentials (DO NOT COMMIT)
├── vps-setup/                         # Deployment scripts for VPS
│   ├── 00_CLAUDE_VPS_INSTRUCTIONS.md  # Instructions for Claude on VPS
│   ├── 01_system_setup.sh            # System dependencies
│   ├── 02_install_tools.sh           # Development tools
│   ├── 03_clone_euclid.sh            # Euclid SDK
│   ├── 04_create_metagraph.sh        # Create ProvenanceAI metagraph
│   ├── 05_build_metagraph.sh         # Build containers
│   ├── 06_deploy_integrationnet.sh   # Deploy to IntegrationNet
│   └── 07_verify_deployment.sh       # Verify deployment
└── (future: source code will be on VPS)
```

---

## 🎯 What This Does

This directory contains everything needed to deploy the **ProvenanceAI Constellation Metagraph** to IntegrationNet.

### ProofOfGeneration Data Structure

The metagraph tracks AI-generated content with the following fields:

```scala
case class ProofOfGeneration(
  contentHash: String,      // Hash of AI-generated content
  modelName: String,        // AI model used (e.g., "deepseek-chat")
  timestamp: Long,          // Generation timestamp
  storyIpId: String,       // Story Protocol IP ID
  nftContract: String,     // NFT contract address
  nftTokenId: Long,        // NFT token ID
  generatorAddress: String // ICP canister identifier
)
```

---

## 🚀 Deployment Process

### For Claude on VPS (198.144.183.32)

Run scripts in order:

```bash
cd /root/provenanceai-metagraph/vps-setup

./01_system_setup.sh              # ~10 mins
./02_install_tools.sh             # ~15 mins
./03_clone_euclid.sh              # ~3 mins
./04_create_metagraph.sh          # ~10 mins
./05_build_metagraph.sh           # ~20 mins
./06_deploy_integrationnet.sh     # ~10 mins
./07_verify_deployment.sh         # ~5 mins
```

**Total Time**: ~1.5 hours

### For Manual Deployment

See `vps-setup/00_CLAUDE_VPS_INSTRUCTIONS.md` for detailed instructions.

---

## 🔗 Endpoints

After deployment, the metagraph will be accessible at:

- **Data L1**: http://198.144.183.32:9400
  - POST endpoint: `http://198.144.183.32:9400/data`
- **Metagraph L0**: http://198.144.183.32:9200
- **IntegrationNet Explorer**: https://integrationnet.dagexplorer.io/

---

## 🔄 Integration with ICP Canister

The ICP canister (in `../icp-canister/`) will POST ProofOfGeneration data to:

```
POST http://198.144.183.32:9400/data
Content-Type: application/json

{
  "value": {
    "ProofOfGeneration": {
      "contentHash": "0xabc123...",
      "modelName": "deepseek-chat",
      "timestamp": 1729512000000,
      "storyIpId": "0x0523...",
      "nftContract": "0x6853...",
      "nftTokenId": 1,
      "generatorAddress": "ICP-Canister"
    }
  },
  "proofs": []
}
```

---

## 📚 Documentation

- **Constellation Docs**: https://docs.constellationnetwork.io/
- **Euclid SDK**: https://github.com/Constellation-Labs/euclid-development-environment
- **Metagraph Examples**: https://github.com/Constellation-Labs/metagraph-examples

---

## 🔐 Security

- `.env.vps` contains VPS credentials - **NEVER commit to Git**
- File is automatically added to `.gitignore`
- All scripts run on the VPS itself (local execution)

---

## ✅ Status

- [x] Scripts created
- [ ] VPS setup complete
- [ ] Metagraph deployed
- [ ] Verified on IntegrationNet
- [ ] ICP canister integrated

---

**Next**: Execute scripts on VPS via Claude
