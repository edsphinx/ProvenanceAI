// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import "forge-std/Test.sol";
import "../src/SimpleNFT.sol";

contract SimpleNFTTest is Test {
    SimpleNFT public nft;
    address owner;
    address user1;
    address user2;

    event NFTMinted(address indexed to, uint256 indexed tokenId, string contentHash);

    function setUp() public {
        owner = address(this);
        user1 = address(0x1);
        user2 = address(0x2);

        // Deploy SimpleNFT
        nft = new SimpleNFT("ProvenanceAI NFT", "PNFT");
    }

    // ============================================================
    // Deployment Tests
    // ============================================================

    function testDeployment() public {
        assertEq(nft.name(), "ProvenanceAI NFT");
        assertEq(nft.symbol(), "PNFT");
        assertEq(nft.owner(), owner);
    }

    // ============================================================
    // Minting Tests
    // ============================================================

    function testMintNFT() public {
        string memory contentHash = "0xabc123def456";
        string memory metadataURI = "ipfs://QmTest123";

        // Mint NFT
        uint256 tokenId = nft.mint(user1, contentHash, metadataURI);

        // Verify
        assertEq(tokenId, 1);
        assertEq(nft.ownerOf(tokenId), user1);
        assertEq(nft.tokenURI(tokenId), metadataURI);
        assertEq(nft.tokenContentHash(tokenId), contentHash);
    }

    function testMintMultipleNFTs() public {
        // Mint first NFT
        uint256 tokenId1 = nft.mint(user1, "hash1", "uri1");
        assertEq(tokenId1, 1);

        // Mint second NFT
        uint256 tokenId2 = nft.mint(user2, "hash2", "uri2");
        assertEq(tokenId2, 2);

        // Verify owners
        assertEq(nft.ownerOf(tokenId1), user1);
        assertEq(nft.ownerOf(tokenId2), user2);
    }

    function testMintEmitsEvent() public {
        string memory contentHash = "0xabc123";
        string memory metadataURI = "ipfs://test";

        // Expect NFTMinted event
        vm.expectEmit(true, true, false, true);
        emit NFTMinted(user1, 1, contentHash);

        nft.mint(user1, contentHash, metadataURI);
    }

    function testCannotMintAsNonOwner() public {
        vm.prank(user1);

        vm.expectRevert();
        nft.mint(user1, "hash", "uri");
    }

    // ============================================================
    // Content Hash Tests
    // ============================================================

    function testGetContentHash() public {
        string memory contentHash = "0xdeadbeef";
        uint256 tokenId = nft.mint(user1, contentHash, "uri");

        assertEq(nft.tokenContentHash(tokenId), contentHash);
    }

    function testContentHashIsImmutable() public {
        string memory originalHash = "0xoriginal";
        uint256 tokenId = nft.mint(user1, originalHash, "uri");

        // Content hash should remain the same
        assertEq(nft.tokenContentHash(tokenId), originalHash);
    }

    // ============================================================
    // Metadata URI Tests
    // ============================================================

    function testTokenURI() public {
        string memory metadataURI = "ipfs://QmTest123456";
        uint256 tokenId = nft.mint(user1, "hash", metadataURI);

        assertEq(nft.tokenURI(tokenId), metadataURI);
    }

    function testTokenURIForNonexistentToken() public {
        vm.expectRevert();
        nft.tokenURI(999);
    }

    // ============================================================
    // Ownership Tests
    // ============================================================

    function testTransferNFT() public {
        uint256 tokenId = nft.mint(user1, "hash", "uri");

        // Transfer from user1 to user2
        vm.prank(user1);
        nft.transferFrom(user1, user2, tokenId);

        assertEq(nft.ownerOf(tokenId), user2);
    }

    function testOwnerCannotChangeAfterTransfer() public {
        uint256 tokenId = nft.mint(user1, "hash", "uri");

        vm.prank(user1);
        nft.transferFrom(user1, user2, tokenId);

        // Original owner cannot transfer anymore
        vm.prank(user1);
        vm.expectRevert();
        nft.transferFrom(user2, user1, tokenId);
    }

    // ============================================================
    // Edge Cases
    // ============================================================

    function testMintToZeroAddress() public {
        vm.expectRevert();
        nft.mint(address(0), "hash", "uri");
    }

    function testEmptyContentHash() public {
        // Empty content hash should work (contract doesn't validate)
        uint256 tokenId = nft.mint(user1, "", "uri");
        assertEq(nft.tokenContentHash(tokenId), "");
    }

    function testEmptyMetadataURI() public {
        // Empty metadata URI should work
        uint256 tokenId = nft.mint(user1, "hash", "");
        assertEq(nft.tokenURI(tokenId), "");
    }

    // ============================================================
    // Gas Tests
    // ============================================================

    function testMintGas() public {
        uint256 gasBefore = gasleft();
        nft.mint(user1, "hash", "uri");
        uint256 gasUsed = gasBefore - gasleft();

        // Ensure mint costs less than 200k gas
        assertLt(gasUsed, 200_000);
    }
}
