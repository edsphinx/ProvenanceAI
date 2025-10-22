// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/// @title SimpleNFT - Basic ERC721 for Story Protocol IP Registration
/// @notice This contract is used by ProvenanceAI to mint NFTs that will be registered as IP Assets
/// @dev Each NFT represents AI-generated content that will be registered on Story Protocol
contract SimpleNFT is ERC721, Ownable {
    uint256 private _tokenIdCounter;

    // Mapping from token ID to content hash (for tracking AI-generated content)
    mapping(uint256 => string) public tokenContentHash;

    // Mapping from token ID to metadata URI
    mapping(uint256 => string) private _tokenURIs;

    event NFTMinted(address indexed to, uint256 indexed tokenId, string contentHash);

    /// @notice Initialize the NFT collection
    /// @param name The name of the NFT collection
    /// @param symbol The symbol of the NFT collection
    constructor(string memory name, string memory symbol)
        ERC721(name, symbol)
        Ownable(msg.sender)
    {
        _tokenIdCounter = 1; // Start token IDs at 1
    }

    /// @notice Mint a new NFT with content hash
    /// @param to The address that will receive the NFT
    /// @param contentHash The hash of the AI-generated content
    /// @param metadataURI The URI pointing to the NFT metadata
    /// @return tokenId The ID of the newly minted token
    function mint(
        address to,
        string memory contentHash,
        string memory metadataURI
    ) external onlyOwner returns (uint256) {
        uint256 tokenId = _tokenIdCounter;
        _tokenIdCounter++;

        _safeMint(to, tokenId);
        tokenContentHash[tokenId] = contentHash;
        _tokenURIs[tokenId] = metadataURI;

        emit NFTMinted(to, tokenId, contentHash);

        return tokenId;
    }

    /// @notice Get the metadata URI for a token
    /// @param tokenId The token ID
    /// @return The metadata URI
    function tokenURI(uint256 tokenId) public view virtual override returns (string memory) {
        require(ownerOf(tokenId) != address(0), "Token does not exist");
        return _tokenURIs[tokenId];
    }

    /// @notice Get the total number of tokens minted
    /// @return The current token ID counter
    function totalSupply() external view returns (uint256) {
        return _tokenIdCounter - 1;
    }
}
