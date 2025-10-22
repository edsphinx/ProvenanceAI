// NFT Deployment Module
// This module handles the deployment of SimpleNFT contracts
// Uses existing EVM signing code for consistency

use crate::config::{self, STORY_CHAIN_ID, STORY_RPC_URL};
use crate::evm_util::{build_evm_transaction_for_creation, build_signed_transaction_for_creation, sign_evm_transaction};
use crate::http_util::{json_header, make_http_request};
use ic_cdk::api::management_canister::http_request::HttpMethod;
use serde_json::json;
use sha3::{Digest, Keccak256};
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use std::str::FromStr;

// ==============================================================================
// SimpleNFT Contract ABI (generated from compiled Solidity)
// ==============================================================================

// Import the compiled bytecode
const SIMPLE_NFT_BYTECODE: &str = include_str!("../../../out/SimpleNFT.sol/SimpleNFT.json");

// ==============================================================================
// Deploy SimpleNFT Contract
// ==============================================================================

/// Deploy a SimpleNFT contract to Story Protocol Aeneid testnet
///
/// This function deploys the SimpleNFT contract using our existing EVM signing infrastructure.
///
/// # Arguments
/// * `name` - The name of the NFT collection
/// * `symbol` - The symbol of the NFT collection
///
/// # Returns
/// * `Result<String, String>` - The deployed contract address or error
pub async fn deploy_simple_nft(
    name: String,
    symbol: String,
) -> Result<String, String> {
    ic_cdk::println!("🚀 Deploying SimpleNFT contract...");
    ic_cdk::println!("   Name: {}", name);
    ic_cdk::println!("   Symbol: {}", symbol);

    // Parse the compiled JSON to get the bytecode
    let compiled_json: serde_json::Value = serde_json::from_str(SIMPLE_NFT_BYTECODE)
        .map_err(|e| format!("Failed to parse compiled contract: {}", e))?;

    let bytecode_hex = compiled_json["bytecode"]["object"]
        .as_str()
        .ok_or("Bytecode not found in compiled JSON")?;

    // Remove 0x prefix if present
    let bytecode_hex = bytecode_hex.trim_start_matches("0x");

    // Decode bytecode
    let bytecode = hex::decode(bytecode_hex)
        .map_err(|e| format!("Failed to decode bytecode: {}", e))?;

    ic_cdk::println!("   Bytecode size: {} bytes", bytecode.len());

    // Encode constructor parameters (name, symbol)
    let constructor_params = ethabi::encode(&[
        ethabi::Token::String(name),
        ethabi::Token::String(symbol),
    ]);

    // Combine bytecode + constructor params
    let mut deployment_data = bytecode;
    deployment_data.extend_from_slice(&constructor_params);

    ic_cdk::println!("   Total deployment size: {} bytes", deployment_data.len());

    // Get fresh nonce from blockchain via RPC
    let nonce = crate::get_nonce_from_blockchain().await?;
    ic_cdk::println!("   Nonce (from blockchain): {}", nonce);

    // Get the canister's EVM address
    let evm_address = crate::evm_util::get_canister_evm_address().await?;
    ic_cdk::println!("   Deploying from: {}", evm_address);

    // Build unsigned transaction for contract creation (EIP-155 format)
    let unsigned_tx = build_evm_transaction_for_creation(
        nonce,
        config::GAS_PRICE,
        config::GAS_LIMIT,
        0, // No value transfer
        deployment_data.clone(),
        STORY_CHAIN_ID,
    );

    ic_cdk::println!("   Unsigned TX: {} bytes", unsigned_tx.len());

    // Hash the unsigned transaction for signing
    let tx_hash = Keccak256::digest(&unsigned_tx);
    let tx_hash_bytes = tx_hash.to_vec();

    ic_cdk::println!("   TX Hash for signing: 0x{}", hex::encode(&tx_hash_bytes));

    // Sign the transaction using Chain-Key ECDSA
    let signature = sign_evm_transaction(tx_hash_bytes.clone()).await?;

    if signature.len() != 64 {
        return Err(format!(
            "Invalid signature length: {} (expected 64)",
            signature.len()
        ));
    }

    ic_cdk::println!("   Signature: {} bytes", signature.len());

    // Get the IC's public key for recovery ID determination
    let ic_public_key = crate::evm_util::get_canister_public_key().await?;

    // Determine recovery ID
    let recovery_id = match crate::story_util::determine_recovery_id_with_pubkey(&tx_hash_bytes, &signature, &ic_public_key) {
        Ok(rid) => {
            ic_cdk::println!("   ✅ Recovery ID: {}", rid);
            rid
        }
        Err(e) => {
            ic_cdk::println!("   ⚠️  Could not determine recovery ID: {}", e);
            0u8
        }
    };

    // Build signed transaction
    let signed_tx = build_signed_transaction_for_creation(
        nonce,
        config::GAS_PRICE,
        config::GAS_LIMIT,
        0,
        deployment_data,
        &signature,
        STORY_CHAIN_ID,
        recovery_id,
    );

    // Broadcast transaction to Story Protocol
    let tx_hash_result = broadcast_transaction(signed_tx).await?;

    ic_cdk::println!("   ✅ Deployment transaction sent!");
    ic_cdk::println!("   📍 Transaction Hash: {}", tx_hash_result);
    ic_cdk::println!("   🔍 View on Story Explorer:");
    ic_cdk::println!("      https://aeneid.storyscan.io/tx/{}", tx_hash_result);

    // Wait for transaction to be mined and get receipt
    ic_cdk::println!("   ⏳ Waiting for transaction to be mined...");

    let receipt = get_transaction_receipt(&tx_hash_result).await?;

    // Check if deployment was successful
    let status = receipt.get("status")
        .and_then(|s| s.as_str())
        .ok_or("Missing status in receipt")?;

    if status != "0x1" {
        return Err("Contract deployment failed".to_string());
    }

    // Get the deployed contract address
    let contract_address = receipt.get("contractAddress")
        .and_then(|a| a.as_str())
        .ok_or("No contract address in receipt")?
        .to_string();

    ic_cdk::println!("   ✅ SimpleNFT deployed successfully!");
    ic_cdk::println!("   📍 Contract Address: {}", contract_address);
    ic_cdk::println!("   🔍 View on Story Explorer:");
    ic_cdk::println!("      https://aeneid.storyscan.io/address/{}", contract_address);

    Ok(contract_address)
}

// ==============================================================================
// Mint NFT
// ==============================================================================

/// Mint a new NFT from the deployed SimpleNFT contract
///
/// This function mints an NFT with the given content hash and metadata URI.
/// The NFT will be minted to the canister's EVM address.
///
/// # Arguments
/// * `nft_contract_address` - The deployed SimpleNFT contract address
/// * `content_hash` - The hash of the AI-generated content
/// * `metadata_uri` - IPFS or HTTP URL pointing to metadata JSON
///
/// # Returns
/// * `Result<u64, String>` - The minted token ID or error
pub async fn mint_nft(
    nft_contract_address: String,
    content_hash: String,
    metadata_uri: String,
) -> Result<u64, String> {
    ic_cdk::println!("🎨 Minting NFT...");
    ic_cdk::println!("   Contract: {}", nft_contract_address);
    ic_cdk::println!("   Content Hash: {}", content_hash);
    ic_cdk::println!("   Metadata URI: {}", metadata_uri);

    // Get the canister's EVM address (this will be the NFT recipient)
    let recipient = crate::evm_util::get_canister_evm_address().await?;
    ic_cdk::println!("   Minting to: {}", recipient);

    // Parse contract address
    let contract_address = primitive_types::H160::from_str(&nft_contract_address)
        .map_err(|e| format!("Invalid contract address: {}", e))?;

    // Build the mint() function call data
    // function mint(address to, string memory contentHash, string memory metadataURI) external onlyOwner returns (uint256)
    let call_data = build_mint_calldata(&recipient, content_hash, metadata_uri)?;
    ic_cdk::println!("   Call Data: {} bytes", call_data.len());

    // Get fresh nonce from blockchain via RPC
    let nonce = crate::get_nonce_from_blockchain().await?;
    ic_cdk::println!("   Nonce (from blockchain): {}", nonce);

    // Build unsigned transaction (EIP-155 format)
    let to_bytes: [u8; 20] = contract_address.to_fixed_bytes();
    let unsigned_tx = crate::evm_util::build_evm_transaction(
        nonce,
        config::GAS_PRICE,
        config::GAS_LIMIT,
        &to_bytes,
        0, // No value transfer
        call_data.clone(),
        STORY_CHAIN_ID,
    );

    ic_cdk::println!("   Unsigned TX: {} bytes", unsigned_tx.len());

    // Hash the unsigned transaction for signing
    let tx_hash = Keccak256::digest(&unsigned_tx);
    let tx_hash_bytes = tx_hash.to_vec();

    ic_cdk::println!("   TX Hash for signing: 0x{}", hex::encode(&tx_hash_bytes));

    // Sign the transaction using Chain-Key ECDSA
    let signature = crate::evm_util::sign_evm_transaction(tx_hash_bytes.clone()).await?;

    if signature.len() != 64 {
        return Err(format!(
            "Invalid signature length: {} (expected 64)",
            signature.len()
        ));
    }

    ic_cdk::println!("   Signature: {} bytes", signature.len());

    // Get the IC's public key for recovery ID determination
    let ic_public_key = crate::evm_util::get_canister_public_key().await?;

    // Determine recovery ID
    let recovery_id = match crate::story_util::determine_recovery_id_with_pubkey(&tx_hash_bytes, &signature, &ic_public_key) {
        Ok(rid) => {
            ic_cdk::println!("   ✅ Recovery ID: {}", rid);
            rid
        }
        Err(e) => {
            ic_cdk::println!("   ⚠️  Could not determine recovery ID: {}", e);
            0u8
        }
    };

    // Build signed transaction
    let signed_tx = crate::evm_util::build_signed_transaction(
        nonce,
        config::GAS_PRICE,
        config::GAS_LIMIT,
        &to_bytes,
        0,
        call_data,
        &signature,
        STORY_CHAIN_ID,
        recovery_id,
    );

    // Broadcast transaction to Story Protocol
    let tx_hash_result = broadcast_transaction(signed_tx).await?;

    ic_cdk::println!("   ✅ Mint transaction sent!");
    ic_cdk::println!("   📍 Transaction Hash: {}", tx_hash_result);
    ic_cdk::println!("   🔍 View on Story Explorer:");
    ic_cdk::println!("      https://aeneid.storyscan.io/tx/{}", tx_hash_result);

    // Wait for transaction to be mined and get receipt
    ic_cdk::println!("   ⏳ Waiting for transaction to be mined...");

    let receipt = get_transaction_receipt(&tx_hash_result).await?;

    // Check if mint was successful
    let status = receipt.get("status")
        .and_then(|s| s.as_str())
        .ok_or("Missing status in receipt")?;

    if status != "0x1" {
        return Err("NFT minting failed".to_string());
    }

    // Parse logs to extract the token ID
    // The NFTMinted event emits: event NFTMinted(address indexed to, uint256 indexed tokenId, string contentHash)
    let token_id = extract_token_id_from_receipt(&receipt)?;

    ic_cdk::println!("   ✅ NFT minted successfully!");
    ic_cdk::println!("   🎫 Token ID: {}", token_id);
    ic_cdk::println!("   🔍 View on Story Explorer:");
    ic_cdk::println!("      https://aeneid.storyscan.io/token/{}/instance/{}", nft_contract_address, token_id);

    Ok(token_id)
}

// ==============================================================================
// Helper Functions
// ==============================================================================

/// Build the calldata for SimpleNFT.mint() function
///
/// function mint(address to, string memory contentHash, string memory metadataURI) external onlyOwner returns (uint256)
fn build_mint_calldata(
    to: &str,
    content_hash: String,
    metadata_uri: String,
) -> Result<Vec<u8>, String> {
    // Parse recipient address
    let to_address = primitive_types::H160::from_str(to)
        .map_err(|e| format!("Invalid recipient address: {}", e))?;

    // Function selector for mint(address,string,string)
    // keccak256("mint(address,string,string)") = 0x99071190
    let function_selector = &hex::decode("99071190")
        .map_err(|e| format!("Failed to decode function selector: {}", e))?;

    // Encode parameters
    let params = ethabi::encode(&[
        ethabi::Token::Address(to_address.into()),
        ethabi::Token::String(content_hash),
        ethabi::Token::String(metadata_uri),
    ]);

    // Combine selector + params
    let mut calldata = function_selector.clone();
    calldata.extend_from_slice(&params);

    Ok(calldata)
}

/// Extract token ID from transaction receipt logs
///
/// The NFTMinted event signature is:
/// event NFTMinted(address indexed to, uint256 indexed tokenId, string contentHash)
fn extract_token_id_from_receipt(receipt: &serde_json::Value) -> Result<u64, String> {
    let logs = receipt.get("logs")
        .and_then(|l| l.as_array())
        .ok_or("No logs in receipt")?;

    if logs.is_empty() {
        return Err("No logs emitted by mint transaction".to_string());
    }

    // The NFTMinted event should be the first (or only) log
    let log = &logs[0];

    // Get topics (indexed parameters)
    let topics = log.get("topics")
        .and_then(|t| t.as_array())
        .ok_or("No topics in log")?;

    if topics.len() < 3 {
        return Err("Not enough topics in NFTMinted event".to_string());
    }

    // topics[0] = event signature hash
    // topics[1] = indexed to address
    // topics[2] = indexed tokenId
    let token_id_hex = topics[2]
        .as_str()
        .ok_or("Token ID topic is not a string")?;

    // Parse token ID from hex
    let token_id_hex = token_id_hex.trim_start_matches("0x");
    let token_id = u64::from_str_radix(token_id_hex, 16)
        .map_err(|e| format!("Failed to parse token ID: {}", e))?;

    Ok(token_id)
}

/// Broadcast a signed transaction to the Story Protocol RPC
async fn broadcast_transaction(signed_tx: Vec<u8>) -> Result<String, String> {
    ic_cdk::println!("   Broadcasting transaction to Story RPC...");

    // Convert transaction to hex
    let tx_hex = format!("0x{}", hex::encode(&signed_tx));

    let payload = json!({
        "jsonrpc": "2.0",
        "method": "eth_sendRawTransaction",
        "params": [tx_hex],
        "id": 1
    });

    let headers = vec![json_header()];

    let response_body = make_http_request(
        STORY_RPC_URL.to_string(),
        HttpMethod::POST,
        headers,
        Some(payload.to_string().into_bytes()),
    )
    .await?;

    let response_str = String::from_utf8(response_body)
        .map_err(|e| format!("Failed to parse response as UTF-8: {}", e))?;

    let response_json: serde_json::Value = serde_json::from_str(&response_str)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Check for error
    if let Some(error) = response_json.get("error") {
        return Err(format!("RPC error: {}", error));
    }

    let tx_hash = response_json["result"]
        .as_str()
        .ok_or("No result in response")?
        .to_string();

    Ok(tx_hash)
}

/// Get transaction receipt from Story Protocol RPC
async fn get_transaction_receipt(tx_hash: &str) -> Result<serde_json::Value, String> {
    ic_cdk::println!("   Fetching transaction receipt...");

    let payload = json!({
        "jsonrpc": "2.0",
        "method": "eth_getTransactionReceipt",
        "params": [tx_hash],
        "id": 1
    });

    let headers = vec![json_header()];

    let response_body = make_http_request(
        STORY_RPC_URL.to_string(),
        HttpMethod::POST,
        headers,
        Some(payload.to_string().into_bytes()),
    )
    .await?;

    let response_str = String::from_utf8(response_body)
        .map_err(|e| format!("Failed to parse response as UTF-8: {}", e))?;

    let response_json: serde_json::Value = serde_json::from_str(&response_str)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Check for error
    if let Some(error) = response_json.get("error") {
        return Err(format!("RPC error: {}", error));
    }

    let receipt = response_json.get("result")
        .ok_or("No result in response")?
        .clone();

    Ok(receipt)
}
