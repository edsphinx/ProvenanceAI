# ICP Canister - Brain Canister

**Purpose**: AI content generation, NFT minting, IP registration, and cross-chain orchestration
**Network**: Internet Computer (Local testnet for development, IC mainnet for production)

---

## 📁 Directory Structure

```
icp-canister/
├── README.md                    # This file
└── ../src/brain_canister/       # Canister source code
    ├── Cargo.toml              # Dependencies
    ├── brain_canister.did      # Candid interface
    ├── src/
    │   ├── lib.rs              # Main orchestrator
    │   ├── ai_util.rs          # DeepSeek AI integration
    │   ├── evm_util.rs         # EVM transaction utilities
    │   ├── nft_deployment.rs   # NFT deployment & minting
    │   ├── story_util.rs       # Story Protocol integration
    │   └── constellation_util.rs # Constellation DAG logging
    └── .env.dfx                # Local configuration
```

---

## 🎯 What This Does

The Brain Canister is the **orchestration layer** that:

1. **Generates AI content** using DeepSeek API
2. **Mints NFTs** on Story Protocol with content hash
3. **Registers IP assets** on Story Protocol
4. **Logs proofs** on Constellation Network
5. **Uses Chain-Key ECDSA** for EVM transactions (no private keys!)

---

## 🔧 Key Components

### Main Orchestrator (`lib.rs`)

5-step flow:
1. Generate AI content (DeepSeek)
2. Check/deploy NFT contract
3. Mint NFT with content hash
4. Register IP on Story Protocol
5. Log proof on Constellation

### AI Integration (`ai_util.rs`)

- DeepSeek API integration
- Image generation
- Content hashing (MD5)

### EVM Utilities (`evm_util.rs`)

- Chain-Key ECDSA signing
- EIP-155 transaction encoding
- RLP serialization
- Nonce management

### Story Protocol (`story_util.rs`)

- IPAssetRegistry integration
- NFT minting
- IP registration
- Transaction building

### Constellation (`constellation_util.rs`)

- HTTP POST to metagraph
- ProofOfGeneration data structure
- Currently: Simulated (Phase 2.2)
- Phase 3: Real HTTP to `http://198.144.183.32:9400/data`

---

## 🚀 Deployment

### Local Development

```bash
# Start dfx
dfx start --clean --background

# Deploy canister
dfx deploy brain_canister --argument '(record {
  deepseek_api_key = "YOUR_KEY";
  constellation_metagraph_url = "http://198.144.183.32:9400";
  replicate_api_key = opt "";
})'

# Deploy NFT contract (one-time)
dfx canister call brain_canister deploy_nft_contract '("ProvenanceAI NFT", "PROV")'

# Test generation
dfx canister call brain_canister generate_and_register_ip '(record {
  prompt = "A futuristic cityscape";
  metadata = record {
    title = "Test";
    description = "Test generation";
    tags = vec { "test" }
  }
})'
```

### IC Mainnet

```bash
# Deploy to IC mainnet
dfx deploy brain_canister --network ic --argument '(record {
  deepseek_api_key = "YOUR_KEY";
  constellation_metagraph_url = "http://198.144.183.32:9400";
  replicate_api_key = opt "";
})'
```

---

## 🔗 Integration Points

### Story Protocol (Aeneid Testnet)
- **Chain ID**: 1315
- **RPC**: https://aeneid.storyrpc.io
- **IPAssetRegistry**: 0x77319B4031e6eF1250907aa00018B8B1c67a244b
- **SimpleNFT**: 0x6853943248910243cDCE66C21C12398ebbC1642D

### Constellation Network (IntegrationNet)
- **Metagraph Endpoint**: http://198.144.183.32:9400/data
- **Explorer**: https://integrationnet.dagexplorer.io/

---

## 📊 Current Status

- ✅ Phase 1: ICP infrastructure complete
- ✅ Phase 2.1: Story Protocol integration complete
- ✅ Phase 2.2: Constellation integration (simulated) complete
- ⏳ Phase 3: Real Constellation integration (in progress)

---

## 🔐 Security

- **No private keys stored** - Chain-Key ECDSA threshold signatures
- **Nonce management** - Atomic increment for transaction ordering
- **Environment variables** - API keys in `.env.dfx` (not committed)

---

## 📚 Dependencies

```toml
[dependencies]
candid = "0.10"
ic-cdk = "0.13"
serde = "1.0"
serde_json = "1.0"
hex = "0.4"
rlp = "0.5"
k256 = "0.13"
sha2 = "0.10"
md5 = "0.7"
```

---

**Next**: Update `constellation_util.rs` for Phase 3 real integration
