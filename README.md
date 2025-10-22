# ProvenanceAI 🤖⚡

**Cross-Chain AI-Generated Content Registration System**

[![ICP](https://img.shields.io/badge/ICP-Internet_Computer-blue)](https://internetcomputer.org/)
[![Story Protocol](https://img.shields.io/badge/Story-Protocol-purple)](https://story.foundation/)
[![Constellation](https://img.shields.io/badge/Constellation-Network-orange)](https://constellationnetwork.io/)

ProvenanceAI is a decentralized system for generating, minting, and registering AI-generated content as Intellectual Property (IP) assets across multiple blockchain networks.

## 🎯 Overview

ProvenanceAI combines three powerful blockchain ecosystems:

1. **Internet Computer (ICP)** - AI generation & orchestration
2. **Story Protocol** - IP asset registration & licensing
3. **Constellation Network** - DAG-based proof logging

## ✨ Features

- 🎨 **AI Content Generation** - DeepSeek-powered image generation
- 🔐 **Chain-Key ECDSA** - Threshold signatures for EVM transactions (no private keys!)
- 📜 **NFT Minting** - ERC721 NFTs with content hash tracking
- 🏛️ **IP Registration** - Automated Story Protocol registration
- 🌌 **Proof Logging** - Constellation DAG immutable records (simulated for MVP)

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────┐
│  ProvenanceAI Canister (ICP)                        │
│  ├─ DeepSeek AI Integration                         │
│  ├─ SimpleNFT Deployment & Minting                  │
│  ├─ Story Protocol IP Registration                  │
│  └─ Constellation DAG Logging (Phase 3)             │
└─────────────────────────────────────────────────────┘
           ↓                    ↓                 ↓
    Story Protocol      ICP Canister      Constellation
    (Aeneid Testnet)    (Chain-Key)       (DAG Layer)
```

## 🚀 Phase 2.2 Status - COMPLETE ✅

### Deployed Contracts

**Story Protocol Aeneid Testnet (Chain ID: 1315)**

- **SimpleNFT Contract**: [`0x6853943248910243cDCE66C21C12398ebbC1642D`](https://aeneid.storyscan.io/address/0x6853943248910243cDCE66C21C12398ebbC1642D)
- **IPAssetRegistry**: `0x77319B4031e6eF1250907aa00018B8B1c67a244b`
- **First NFT Minted**: [Token ID #1](https://aeneid.storyscan.io/tx/0xa1b0eea1bedf69c9c2725daae560ae2b5df1cc26e30b3e131bbb58db4196ac03)

**ICP Canister**

- **Canister ID**: `uxrrr-q7777-77774-qaaaq-cai` (local)
- **EVM Address**: `0xDa824f554C42ecd28a74A037c70FA0b5bf447bB0`

### Verified Transactions

1. ✅ **SimpleNFT Deployment**: [`0x5b2b1b0c...762209`](https://aeneid.storyscan.io/tx/0x5b2b1b0cffbbc297ccd4d4ef74d81285ce59291a657c92eb8990c6a8e5762209)
2. ✅ **NFT Mint #1**: [`0xa1b0eea1...96ac03`](https://aeneid.storyscan.io/tx/0xa1b0eea1bedf69c9c2725daae560ae2b5df1cc26e30b3e131bbb58db4196ac03)
3. ✅ **IP Asset Registration**: [`0x96f06d91...4e3315`](https://aeneid.storyscan.io/tx/0x96f06d916cdd14ab94e63442df662ae164f12fac1b7f226d0cef4800984e3315)

## 📦 Tech Stack

### Blockchain Platforms

- **Internet Computer (ICP)** - Decentralized compute & AI orchestration
- **Story Protocol** - IP asset management
- **Constellation Network** - DAG-based data integrity

### Smart Contracts

- **Solidity 0.8.26** - SimpleNFT (ERC721)
- **Foundry** - Solidity development framework
- **OpenZeppelin** - Secure contract libraries

### Backend

- **Rust** - ICP canister implementation
- **ic-cdk** - ICP Canister Development Kit
- **Chain-Key ECDSA** - Threshold signature scheme

### AI Integration

- **DeepSeek AI** - Content generation
- **MD5 Hashing** - Content fingerprinting

## 🛠️ Development

### Prerequisites

```bash
# ICP Development
dfx --version  # 0.17.0 or higher

# Rust
cargo --version  # 1.75.0 or higher

# Solidity
forge --version  # Foundry
```

### Installation

```bash
# Clone repository
git clone <repository-url>
cd ProvenanceAI

# Install dependencies
cargo build --target wasm32-unknown-unknown --release --package brain_canister

# Compile Solidity contracts
forge build
```

### Local Deployment

```bash
# Start dfx
dfx start --clean --background

# Deploy canister
dfx deploy brain_canister --argument '(record {
  deepseek_api_key = "your-api-key";
  constellation_metagraph_url = "https://l0-lb-testnet.constellationnetwork.io";
  replicate_api_key = opt "";
})'

# Deploy SimpleNFT (one-time setup)
dfx canister call brain_canister deploy_nft_contract '("ProvenanceAI NFT", "PROV")'
```

### Testing

```bash
# Generate and register AI content
dfx canister call brain_canister generate_and_register_ip '(record {
  prompt = "A futuristic cityscape";
  metadata = record {
    title = "Futuristic City";
    description = "AI-generated artwork";
    tags = vec { "ai"; "city"; "futuristic" }
  }
})'
```

## 📚 Documentation

### Core Modules

- **`src/brain_canister/src/lib.rs`** - Main orchestrator
- **`src/brain_canister/src/nft_deployment.rs`** - NFT deployment & minting
- **`src/brain_canister/src/story_util.rs`** - Story Protocol integration
- **`src/brain_canister/src/constellation_util.rs`** - Constellation DAG logging
- **`src/brain_canister/src/evm_util.rs`** - EVM transaction utilities
- **`src/brain_canister/src/ai_util.rs`** - DeepSeek AI integration
- **`src/SimpleNFT.sol`** - ERC721 NFT contract

### Key Functions

```rust
// Deploy SimpleNFT contract
deploy_nft_contract(name: String, symbol: String) -> Result<String, String>

// Mint NFT with content hash
mint_nft(contract: String, hash: String, uri: String) -> Result<u64, String>

// Register NFT as IP Asset
register_nft_as_ip(contract: String, token_id: u64) -> Result<String, String>

// Complete end-to-end flow
generate_and_register_ip(input: GenerationInput) -> Result<GenerationOutput, String>
```

## 🗺️ Roadmap

### Phase 1 ✅ COMPLETE
- [x] ICP canister setup
- [x] DeepSeek AI integration
- [x] Chain-Key ECDSA implementation
- [x] Basic EVM transaction signing

### Phase 2.1 ✅ COMPLETE
- [x] SimpleNFT contract development
- [x] NFT deployment from canister
- [x] NFT minting with content hash
- [x] Story Protocol IP registration
- [x] End-to-end orchestration

### Phase 2.2 ✅ COMPLETE
- [x] Constellation DAG integration (MVP - simulated logging)
- [x] Proof of generation data structure
- [x] End-to-end flow with all 3 layers

### Phase 3 🔜 UPCOMING
- [ ] Constellation metagraph deployment (production)
- [ ] IPFS metadata upload
- [ ] SPG NFT with licensing
- [ ] Royalty module integration
- [ ] Advanced IP features

### Phase 4 📅 PLANNED
- [ ] ckBTC payment integration
- [ ] Web frontend (Next.js)
- [ ] Wallet connection
- [ ] User dashboard

### Phase 5 🔮 FUTURE
- [ ] Dispute resolution module
- [ ] Multi-chain deployment
- [ ] Advanced AI models
- [ ] Community governance

## 🔐 Security

- **No Private Keys** - Chain-Key ECDSA threshold signatures
- **Nonce Management** - Atomic increment for transaction ordering
- **Access Control** - Owner-based permissions
- **EIP-155** - Replay attack protection

## 📄 License

MIT License

## 🙏 Acknowledgments

- **Internet Computer** - Decentralized compute platform
- **Story Protocol** - IP asset infrastructure
- **Constellation Network** - DAG technology
- **DeepSeek** - AI content generation
- **OpenZeppelin** - Secure smart contract libraries

## 📞 Contact

For questions and support, please open an issue in this repository.

---

**Built with ❤️ for the future of AI-generated IP** 🚀

*ProvenanceAI - Where AI meets Blockchain meets IP Protection*
