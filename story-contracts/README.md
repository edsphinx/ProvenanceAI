# Story Protocol Contracts

**Purpose**: NFT contracts and Story Protocol integration on Aeneid Testnet
**Network**: Story Protocol Aeneid Testnet (Chain ID: 1315)

---

## 📁 Directory Structure

```
story-contracts/
├── README.md                    # This file
└── ../src/SimpleNFT.sol         # ERC721 NFT contract
└── (future: additional contracts)
```

---

## 🎯 What This Does

This directory contains smart contracts deployed on Story Protocol for:

1. **NFT Minting**: ERC721 NFTs with content hash tracking
2. **IP Registration**: Registering NFTs as IP assets on Story Protocol
3. **Licensing** (future): Attach license terms to IP assets
4. **Royalties** (future): Royalty distribution for derivative works

---

## 📜 SimpleNFT Contract

### Features

- **ERC721 standard** - Standard NFT functionality
- **Content hash tracking** - Each token stores its content hash
- **Ownable** - Only owner can mint
- **URI support** - Metadata URIs for each token

### Deployed Contract

**Address**: `0x6853943248910243cDCE66C21C12398ebbC1642D`
**Explorer**: https://aeneid.storyscan.io/address/0x6853943248910243cDCE66C21C12398ebbC1642D

### Contract Code

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract SimpleNFT is ERC721, Ownable {
    uint256 private _tokenIdCounter;

    // Mapping from token ID to content hash
    mapping(uint256 => string) private _contentHashes;

    // Mapping from token ID to token URI
    mapping(uint256 => string) private _tokenURIs;

    constructor(
        string memory name,
        string memory symbol,
        address initialOwner
    ) ERC721(name, symbol) Ownable(initialOwner) {}

    function mint(
        address to,
        string memory contentHash,
        string memory uri
    ) public onlyOwner returns (uint256) {
        _tokenIdCounter++;
        uint256 tokenId = _tokenIdCounter;

        _safeMint(to, tokenId);
        _contentHashes[tokenId] = contentHash;
        _tokenURIs[tokenId] = uri;

        return tokenId;
    }

    function getContentHash(uint256 tokenId) public view returns (string memory) {
        require(ownerOf(tokenId) != address(0), "Token does not exist");
        return _contentHashes[tokenId];
    }

    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        require(ownerOf(tokenId) != address(0), "Token does not exist");
        return _tokenURIs[tokenId];
    }
}
```

---

## 🚀 Deployment

### Using ICP Canister

The NFT contract is deployed automatically by the ICP canister:

```bash
# Deploy NFT contract
dfx canister call brain_canister deploy_nft_contract '("ProvenanceAI NFT", "PROV")'

# Response: NFT contract address
```

### Manual Deployment (Foundry)

```bash
# Compile
forge build

# Deploy to Aeneid testnet
forge create --rpc-url https://aeneid.storyrpc.io \
  --private-key $PRIVATE_KEY \
  --constructor-args "ProvenanceAI NFT" "PROV" $OWNER_ADDRESS \
  src/SimpleNFT.sol:SimpleNFT
```

---

## 🔗 Story Protocol Integration

### IP Asset Registration

After minting an NFT, it's registered as an IP asset:

```solidity
// IPAssetRegistry.sol (Story Protocol)
function register(
    uint256 chainid,
    address tokenContract,
    uint256 tokenId
) external returns (address ipId)
```

### Deployed Story Protocol Contracts

**Aeneid Testnet (Chain ID: 1315)**

- **IPAssetRegistry**: `0x77319B4031e6eF1250907aa00018B8B1c67a244b`
- **RegistrationWorkflows**: `0xbe39E1C756e921BD25DF86e7AAa31106d1eb0424`

---

## 📊 Deployment History

| Contract | Address | TX Hash | Token IDs |
|----------|---------|---------|-----------|
| SimpleNFT | 0x6853...642D | 0x5b2b1b0c...762209 | 1, 2, 3... |

### Verified Transactions

1. **Contract Deployment**: [0x5b2b1b0c...762209](https://aeneid.storyscan.io/tx/0x5b2b1b0cffbbc297ccd4d4ef74d81285ce59291a657c92eb8990c6a8e5762209)
2. **First NFT Mint**: [0xa1b0eea1...96ac03](https://aeneid.storyscan.io/tx/0xa1b0eea1bedf69c9c2725daae560ae2b5df1cc26e30b3e131bbb58db4196ac03)
3. **IP Registration**: [0x96f06d91...4e3315](https://aeneid.storyscan.io/tx/0x96f06d916cdd14ab94e63442df662ae164f12fac1b7f226d0cef4800984e3315)

---

## 🔮 Future Enhancements

### SPG NFT Collection (Phase 3+)

Replace SimpleNFT with SPG (Story Protocol Gateway) NFT:

- **Licensing**: Attach license terms to IP assets
- **Royalties**: Automatic royalty distribution
- **Derivatives**: Register derivative IPs

### Dispute Module (Phase 5)

- Raise disputes on IP assets
- Arbitration workflow
- Resolution tracking

---

## 🔐 Security

- **Ownable pattern**: Only canister can mint
- **ERC721 standard**: Battle-tested implementation
- **OpenZeppelin**: Secure contract libraries

---

## 📚 Resources

- **Story Protocol Docs**: https://docs.story.foundation/
- **Aeneid Explorer**: https://aeneid.storyscan.io/
- **OpenZeppelin**: https://docs.openzeppelin.com/contracts/

---

**Next**: Implement SPG NFT with licensing and royalties
