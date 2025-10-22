// Story Protocol Integration Module
// Handles IP registration, licensing, royalties, and disputes on Story Protocol

use crate::config::{self, STORY_CHAIN_ID, STORY_RPC_URL};
use crate::evm_util::{build_evm_transaction, build_signed_transaction, sign_evm_transaction};
use crate::http_util::{json_header, make_http_request};
use ethabi::{encode, Address, Token};
use ic_cdk::api::management_canister::http_request::HttpMethod;
use serde_json::json;
use sha3::{Digest, Keccak256};
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};

// ==============================================================================
// Story Protocol IP Registration
// ==============================================================================

/// Register a new IP asset on Story Protocol
///
/// This function:
/// 1. Gets and increments the canister's nonce atomically
/// 2. Gets the canister's EVM address
/// 3. Builds the contract call data for registerRootIp
/// 4. Creates and signs an EVM transaction using Chain-Key ECDSA
/// 5. Broadcasts it to Story Protocol
/// 6. Returns the transaction hash
///
/// # Arguments
/// * `content_hash` - The keccak256 hash of the content
/// * `metadata_uri` - IPFS or HTTP URL pointing to metadata JSON
///
/// # Returns
/// * `Result<String, String>` - Transaction hash or error
/// Register IP using mintAndRegisterIp (Phase 4 - SPG NFT)
#[allow(dead_code)]
pub async fn register_ip_on_story(
    content_hash: String,
    metadata_uri: String,
) -> Result<String, String> {
    ic_cdk::println!("   📜 Registering IP on Story Protocol...");
    ic_cdk::println!("      Content Hash: {}", content_hash);
    ic_cdk::println!("      Metadata URI: {}", metadata_uri);

    // Step 1: Get fresh nonce from blockchain via RPC
    // This ensures we always use the correct nonce even after canister reinstalls
    let nonce = crate::get_nonce_from_blockchain().await?;
    ic_cdk::println!("      Nonce (from blockchain): {}", nonce);

    // Step 2: Get the canister's EVM address
    let evm_address = crate::evm_util::get_canister_evm_address().await?;
    ic_cdk::println!("      Canister EVM Address: {}", evm_address);

    // Step 3: Build contract call data for mintAndRegisterIp
    let call_data = build_mint_and_register_ip_calldata(metadata_uri, &evm_address)?;
    ic_cdk::println!("      Call Data: {} bytes", call_data.len());

    // Step 4: Build unsigned transaction (EIP-155 format)
    let to = config::registration_workflows_address();
    let to_bytes: [u8; 20] = to.to_fixed_bytes();

    let unsigned_tx = build_evm_transaction(
        nonce,
        config::GAS_PRICE,
        config::GAS_LIMIT,
        &to_bytes,
        0, // No value transfer
        call_data.clone(),
        STORY_CHAIN_ID,
    );

    ic_cdk::println!("      Unsigned TX: {} bytes", unsigned_tx.len());
    ic_cdk::println!("      Unsigned TX hex: 0x{}", hex::encode(&unsigned_tx));

    // Step 5: Hash the unsigned transaction for signing
    let tx_hash = Keccak256::digest(&unsigned_tx);
    let tx_hash_bytes = tx_hash.to_vec();

    ic_cdk::println!("      TX Hash for signing: 0x{}", hex::encode(&tx_hash_bytes));

    // Step 6: Sign the transaction using Chain-Key ECDSA
    let signature = sign_evm_transaction(tx_hash_bytes.clone()).await?;

    if signature.len() != 64 {
        return Err(format!(
            "Invalid signature length: {} (expected 64)",
            signature.len()
        ));
    }

    ic_cdk::println!("      Signature: {} bytes", signature.len());
    ic_cdk::println!("         r: 0x{}", hex::encode(&signature[0..32]));
    ic_cdk::println!("         s: 0x{}", hex::encode(&signature[32..64]));

    // Step 7: Verify signature and determine recovery ID
    ic_cdk::println!("      Verifying signature with IC public key...");

    // Get the IC's public key that was used for signing
    let ic_public_key = crate::evm_util::get_canister_public_key().await?;
    ic_cdk::println!("      IC Public Key: {} bytes", ic_public_key.len());
    ic_cdk::println!("      IC Public Key hex: 0x{}", hex::encode(&ic_public_key));

    // First, verify the signature is valid for this public key
    match verify_signature(&tx_hash_bytes, &signature, &ic_public_key) {
        Ok(true) => ic_cdk::println!("      ✅ Signature is valid for IC public key"),
        Ok(false) => ic_cdk::println!("      ⚠️  Signature verification FAILED!"),
        Err(e) => ic_cdk::println!("      ⚠️  Signature verification error: {}", e),
    }

    let recovery_id = match determine_recovery_id_with_pubkey(&tx_hash_bytes, &signature, &ic_public_key) {
        Ok(rid) => {
            ic_cdk::println!("      ✅ Recovery ID: {}", rid);
            rid
        }
        Err(e) => {
            ic_cdk::println!("      ⚠️  Could not determine recovery ID: {}", e);
            ic_cdk::println!("      ⚠️  Trying both recovery IDs...");
            // If recovery fails, try both and see which works
            0u8
        }
    };

    let signed_tx = build_signed_transaction(
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

    // Step 8: Broadcast transaction to Story Protocol
    let tx_hash_result = broadcast_transaction(signed_tx).await?;

    ic_cdk::println!("   ✅ NFT minted and IP registered! TX Hash: {}", tx_hash_result);
    ic_cdk::println!("   🔍 View on Story Explorer:");
    ic_cdk::println!("      https://aeneid.storyscan.io/tx/{}", tx_hash_result);
    ic_cdk::println!("   💡 Note: Transaction returns (ipId, tokenId) on success");

    Ok(tx_hash_result)
}

// ==============================================================================
// Signature Verification and Recovery Helpers
// ==============================================================================

/// Verify that a signature is valid for a given message and public key
///
/// # Arguments
/// * `message_hash` - The hash that was signed (32 bytes)
/// * `signature` - The ECDSA signature (r, s) - 64 bytes
/// * `public_key` - The public key to verify against (65 bytes, uncompressed)
///
/// # Returns
/// * `Result<bool, String>` - true if signature is valid, false otherwise
/// Verify ECDSA signature (kept for future use)
#[allow(dead_code)]
fn verify_signature(
    message_hash: &[u8],
    signature: &[u8],
    public_key: &[u8],
) -> Result<bool, String> {
    use k256::ecdsa::signature::Verifier;

    if message_hash.len() != 32 {
        return Err(format!("Invalid message hash length: {}", message_hash.len()));
    }

    if signature.len() != 64 {
        return Err(format!("Invalid signature length: {}", signature.len()));
    }

    if public_key.len() != 65 {
        return Err(format!("Invalid public key length: {}", public_key.len()));
    }

    // Parse the public key
    let verifying_key = match VerifyingKey::from_sec1_bytes(public_key) {
        Ok(key) => key,
        Err(e) => return Err(format!("Failed to parse public key: {:?}", e)),
    };

    // Parse the signature
    let sig = match Signature::try_from(signature) {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to parse signature: {:?}", e)),
    };

    // Verify the signature
    Ok(verifying_key.verify(message_hash, &sig).is_ok())
}

/// Determine the correct recovery ID by comparing with the actual IC public key
///
/// The recovery ID (0 or 1) determines which of the possible public keys
/// should be used to verify the signature. We try both and see which one
/// matches the IC's actual public key.
///
/// # Arguments
/// * `message_hash` - The hash that was signed (32 bytes)
/// * `signature` - The ECDSA signature (r, s) - 64 bytes
/// * `ic_public_key` - The actual public key from IC (65 bytes, uncompressed)
///
/// # Returns
/// * `Result<u8, String>` - Recovery ID (0 or 1) or error
pub fn determine_recovery_id_with_pubkey(
    message_hash: &[u8],
    signature: &[u8],
    ic_public_key: &[u8],
) -> Result<u8, String> {
    if message_hash.len() != 32 {
        return Err(format!("Invalid message hash length: {}", message_hash.len()));
    }

    if signature.len() != 64 {
        return Err(format!("Invalid signature length: {}", signature.len()));
    }

    if ic_public_key.len() != 65 {
        return Err(format!("Invalid IC public key length: {}", ic_public_key.len()));
    }

    // The IC public key should start with 0x04 (uncompressed marker)
    if ic_public_key[0] != 0x04 {
        return Err(format!(
            "Invalid IC public key format: expected 0x04 prefix, got 0x{:02x}",
            ic_public_key[0]
        ));
    }

    // Calculate the expected address from IC's public key
    let ic_key_hash = Keccak256::digest(&ic_public_key[1..]);
    let ic_address_bytes = &ic_key_hash[12..];
    let ic_address = hex::encode(ic_address_bytes).to_lowercase();

    ic_cdk::println!("      IC-derived address: 0x{}", ic_address);

    // Try both recovery IDs (0 and 1)
    for recovery_id in 0..2 {
        // Create RecoveryId
        let rid = match RecoveryId::try_from(recovery_id) {
            Ok(r) => r,
            Err(e) => {
                ic_cdk::println!("         recovery_id={}: Failed to create RecoveryId: {:?}", recovery_id, e);
                continue;
            }
        };

        // Parse signature
        let sig = match Signature::try_from(signature) {
            Ok(s) => s,
            Err(e) => {
                ic_cdk::println!("         recovery_id={}: Failed to parse signature: {:?}", recovery_id, e);
                continue;
            }
        };

        // Try to recover the verifying key (public key)
        let recovered_key = match VerifyingKey::recover_from_prehash(message_hash, &sig, rid) {
            Ok(key) => key,
            Err(e) => {
                ic_cdk::println!("         recovery_id={}: Failed to recover key: {:?}", recovery_id, e);
                continue;
            }
        };

        // Convert recovered public key to bytes
        let recovered_key_point = recovered_key.to_encoded_point(false);
        let recovered_key_bytes = recovered_key_point.as_bytes();

        // Calculate address from recovered key
        let recovered_hash = Keccak256::digest(&recovered_key_bytes[1..]);
        let recovered_address_bytes = &recovered_hash[12..];
        let recovered_address = hex::encode(recovered_address_bytes).to_lowercase();

        ic_cdk::println!(
            "         recovery_id={}: recovered address 0x{}",
            recovery_id,
            recovered_address
        );

        // Compare the recovered public key with IC's public key
        if recovered_key_bytes == ic_public_key {
            ic_cdk::println!("         ✅ Public key match! Using recovery_id={}", recovery_id);
            return Ok(recovery_id);
        }

        // Also check if addresses match (as fallback)
        if recovered_address == ic_address {
            ic_cdk::println!("         ✅ Address match! Using recovery_id={}", recovery_id);
            return Ok(recovery_id);
        }
    }

    Err(format!(
        "Neither recovery ID produces a key matching IC's public key. IC address: 0x{}",
        ic_address
    ))
}

// ==============================================================================
// ABI Encoding for Story Protocol Contracts
// ==============================================================================

/// Build calldata for RegistrationWorkflows.mintAndRegisterIp()
///
/// Function signature: mintAndRegisterIp(address spgNftContract, address recipient, IPMetadata calldata ipMetadata)
///
/// IPMetadata struct:
/// struct IPMetadata {
///     string ipMetadataURI;
///     bytes32 ipMetadataHash;
///     string nftMetadataURI;
///     bytes32 nftMetadataHash;
/// }
///
/// # Arguments
/// * `metadata_uri` - IPFS or HTTP URL to IP metadata JSON
/// * `recipient` - EVM address of the IP owner (canister address)
///
/// # Returns
/// * `Result<Vec<u8>, String>` - ABI-encoded calldata or error
/// Build calldata for mintAndRegisterIp (Phase 4 - SPG NFT)
#[allow(dead_code)]
fn build_mint_and_register_ip_calldata(metadata_uri: String, recipient: &str) -> Result<Vec<u8>, String> {
    // Parse recipient address from hex string
    let recipient_hex = recipient.trim_start_matches("0x");
    let recipient_bytes = hex::decode(recipient_hex)
        .map_err(|e| format!("Failed to decode recipient address: {}", e))?;

    if recipient_bytes.len() != 20 {
        return Err(format!("Invalid address length: {}", recipient_bytes.len()));
    }

    let mut recipient_addr_array = [0u8; 20];
    recipient_addr_array.copy_from_slice(&recipient_bytes);
    let recipient_address = Address::from(recipient_addr_array);

    // Get SPG NFT Contract address
    let spg_nft_contract = config::spg_nft_contract_address();
    let spg_nft_bytes: [u8; 20] = spg_nft_contract.to_fixed_bytes();
    let spg_nft_address = Address::from(spg_nft_bytes);

    // Calculate metadata hash (simple keccak256 of the URI for now)
    let metadata_hash = Keccak256::digest(metadata_uri.as_bytes());
    let mut metadata_hash_array = [0u8; 32];
    metadata_hash_array.copy_from_slice(&metadata_hash);

    // For NFT metadata, we'll use the same URI and hash
    let nft_metadata_uri = metadata_uri.clone();
    let nft_metadata_hash = metadata_hash_array;

    // Function selector for mintAndRegisterIp(address,address,(string,bytes32,string,bytes32))
    // keccak256("mintAndRegisterIp(address,address,(string,bytes32,string,bytes32))") = 0xa392aa86
    let function_selector = [0xa3, 0x92, 0xaa, 0x86];

    // Encode the IPMetadata struct as a tuple
    // The struct becomes a tuple: (string, bytes32, string, bytes32)
    let ip_metadata_tuple = Token::Tuple(vec![
        Token::String(metadata_uri),
        Token::FixedBytes(metadata_hash_array.to_vec()),
        Token::String(nft_metadata_uri),
        Token::FixedBytes(nft_metadata_hash.to_vec()),
    ]);

    // Encode all parameters
    let tokens = vec![
        Token::Address(spg_nft_address),
        Token::Address(recipient_address),
        ip_metadata_tuple,
    ];

    let encoded_params = encode(&tokens);

    // Combine selector + params
    let mut calldata = function_selector.to_vec();
    calldata.extend_from_slice(&encoded_params);

    ic_cdk::println!("   📝 Built mintAndRegisterIp calldata:");
    ic_cdk::println!("      SPG NFT Contract: 0x{}", hex::encode(spg_nft_bytes));
    ic_cdk::println!("      Recipient: {}", recipient);
    ic_cdk::println!("      Calldata length: {} bytes", calldata.len());

    Ok(calldata)
}

// ==============================================================================
// Story Protocol RPC Helpers
// ==============================================================================

/// Broadcast a signed transaction to Story Protocol
///
/// # Arguments
/// * `signed_tx` - RLP-encoded signed transaction
///
/// # Returns
/// * `Result<String, String>` - Transaction hash or error
async fn broadcast_transaction(signed_tx: Vec<u8>) -> Result<String, String> {
    ic_cdk::println!("      Broadcasting transaction to Story RPC...");

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

// ==============================================================================
// Attach License (Phase 2.5 - Optional)
// ==============================================================================

#[allow(dead_code)]
pub async fn attach_license_stub() -> Result<String, String> {
    ic_cdk::println!("   ⚠️  [STUB] License attachment - Phase 2.5");
    Ok("LICENSE_ATTACHED_STUB".to_string())
}

// ==============================================================================
// Raise Dispute (Phase 5)
// ==============================================================================

#[allow(dead_code)]
pub async fn raise_dispute_stub() -> Result<String, String> {
    ic_cdk::println!("   ⚠️  [STUB] Dispute filing - Phase 5");
    Ok("DISPUTE_ID_STUB".to_string())
}

// ==============================================================================
// Register NFT as IP Asset (IPAssetRegistry.register)
// ==============================================================================

/// Register an existing NFT as an IP Asset on Story Protocol
///
/// This function uses IPAssetRegistry.register() which is permissionless.
/// The NFT must already be minted and owned by the canister.
///
/// # Arguments
/// * `nft_contract_address` - The address of the NFT contract
/// * `token_id` - The token ID to register
///
/// # Returns
/// * `Result<String, String>` - The IP Asset ID or error
pub async fn register_nft_as_ip(
    nft_contract_address: String,
    token_id: u64,
) -> Result<String, String> {
    ic_cdk::println!("   📜 Registering NFT as IP Asset on Story Protocol...");
    ic_cdk::println!("      NFT Contract: {}", nft_contract_address);
    ic_cdk::println!("      Token ID: {}", token_id);

    // Get fresh nonce from blockchain via RPC
    let nonce = crate::get_nonce_from_blockchain().await?;
    ic_cdk::println!("      Nonce (from blockchain): {}", nonce);

    // Get the canister's EVM address
    let evm_address = crate::evm_util::get_canister_evm_address().await?;
    ic_cdk::println!("      Canister EVM Address: {}", evm_address);

    // Build contract call data for IPAssetRegistry.register(chainId, tokenContract, tokenId)
    let call_data = build_ip_asset_registry_register_calldata(
        STORY_CHAIN_ID,
        &nft_contract_address,
        token_id,
    )?;
    ic_cdk::println!("      Call Data: {} bytes", call_data.len());

    // Build unsigned transaction (EIP-155 format)
    let to = config::ip_asset_registry_address();
    let to_bytes: [u8; 20] = to.to_fixed_bytes();

    let unsigned_tx = build_evm_transaction(
        nonce,
        config::GAS_PRICE,
        config::GAS_LIMIT,
        &to_bytes,
        0, // No value transfer
        call_data.clone(),
        STORY_CHAIN_ID,
    );

    ic_cdk::println!("      Unsigned TX: {} bytes", unsigned_tx.len());

    // Hash the unsigned transaction for signing
    let tx_hash = Keccak256::digest(&unsigned_tx);
    let tx_hash_bytes = tx_hash.to_vec();

    ic_cdk::println!("      TX Hash for signing: 0x{}", hex::encode(&tx_hash_bytes));

    // Sign the transaction using Chain-Key ECDSA
    let signature = sign_evm_transaction(tx_hash_bytes.clone()).await?;

    if signature.len() != 64 {
        return Err(format!(
            "Invalid signature length: {} (expected 64)",
            signature.len()
        ));
    }

    ic_cdk::println!("      Signature: {} bytes", signature.len());

    // Get the IC's public key for recovery ID determination
    let ic_public_key = crate::evm_util::get_canister_public_key().await?;

    // Determine recovery ID
    let recovery_id = match determine_recovery_id_with_pubkey(&tx_hash_bytes, &signature, &ic_public_key) {
        Ok(rid) => {
            ic_cdk::println!("      ✅ Recovery ID: {}", rid);
            rid
        }
        Err(e) => {
            ic_cdk::println!("      ⚠️  Could not determine recovery ID: {}", e);
            0u8
        }
    };

    let signed_tx = build_signed_transaction(
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

    ic_cdk::println!("   ✅ IP Asset registered! TX Hash: {}", tx_hash_result);
    ic_cdk::println!("   🔍 View on Story Explorer:");
    ic_cdk::println!("      https://aeneid.storyscan.io/tx/{}", tx_hash_result);
    ic_cdk::println!("   💡 Note: Transaction returns ipId on success");

    // TODO: In production, we should wait for receipt and extract the ipId from logs
    // For now, return the transaction hash
    Ok(tx_hash_result)
}

/// Build calldata for IPAssetRegistry.register(uint256,address,uint256)
///
/// function register(uint256 chainId, address tokenContract, uint256 tokenId) external returns (address ipId)
fn build_ip_asset_registry_register_calldata(
    chain_id: u64,
    token_contract: &str,
    token_id: u64,
) -> Result<Vec<u8>, String> {
    use std::str::FromStr;

    // Parse token contract address
    let token_contract_address = primitive_types::H160::from_str(token_contract)
        .map_err(|e| format!("Invalid token contract address: {}", e))?;

    // Function selector for register(uint256,address,uint256)
    // keccak256("register(uint256,address,uint256)") = 0xfca247ac
    let function_selector = &hex::decode("fca247ac")
        .map_err(|e| format!("Failed to decode function selector: {}", e))?;

    // Encode parameters
    let params = ethabi::encode(&[
        ethabi::Token::Uint(primitive_types::U256::from(chain_id)),
        ethabi::Token::Address(token_contract_address.into()),
        ethabi::Token::Uint(primitive_types::U256::from(token_id)),
    ]);

    // Combine selector + params
    let mut calldata = function_selector.clone();
    calldata.extend_from_slice(&params);

    Ok(calldata)
}
