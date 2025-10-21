// EVM Utilities Module
// Handles Chain-Key ECDSA for deriving canister-owned EVM addresses

use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument,
};
use sha3::{Digest, Keccak256};

// ==============================================================================
// ECDSA Key Configuration
// ==============================================================================

fn get_ecdsa_key_id() -> EcdsaKeyId {
    EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: crate::config::ECDSA_KEY_NAME.to_string(),
    }
}

// ==============================================================================
// Get Canister EVM Address
// ==============================================================================

/// Derive the canister's Ethereum-compatible address using Chain-Key ECDSA
///
/// This address can be used to sign transactions on Story Protocol (EVM chain)
/// without ever storing a private key.
///
/// # Returns
/// * `Result<String, String>` - EVM address (0x...) or error
pub async fn get_canister_evm_address() -> Result<String, String> {
    ic_cdk::println!("   🔑 Deriving canister EVM address...");

    // Use empty derivation path for root key
    let derivation_path = vec![];

    let key_id = get_ecdsa_key_id();

    // Request public key from management canister
    let request = EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path,
        key_id: key_id.clone(),
    };

    let (response,) = ecdsa_public_key(request)
        .await
        .map_err(|e| format!("Failed to get ECDSA public key: {:?}", e))?;

    let public_key = response.public_key;

    // Derive Ethereum address from public key
    // The public key is 65 bytes: [0x04, x (32 bytes), y (32 bytes)]
    // Ethereum address = last 20 bytes of keccak256(public_key[1..])

    if public_key.len() != 65 {
        return Err(format!(
            "Invalid public key length: {} (expected 65)",
            public_key.len()
        ));
    }

    // Hash the public key (excluding the 0x04 prefix)
    let hash = Keccak256::digest(&public_key[1..]);

    // Take the last 20 bytes
    let address_bytes = &hash[12..];

    // Convert to hex string with 0x prefix
    let address = format!("0x{}", hex::encode(address_bytes));

    ic_cdk::println!("   ✅ Canister EVM Address: {}", address);
    ic_cdk::println!("   💡 Fund this address with testnet IP tokens:");
    ic_cdk::println!("      https://aeneid.faucet.story.foundation");

    Ok(address)
}

// ==============================================================================
// Sign EVM Transaction (Stubbed for Phase 2)
// ==============================================================================

// This will be implemented in Phase 2 when integrating with Story Protocol
// For now, we provide a placeholder

#[allow(dead_code)]
pub async fn sign_evm_transaction(_message_hash: Vec<u8>) -> Result<Vec<u8>, String> {
    ic_cdk::println!("   ⚠️  [STUB] EVM transaction signing - Phase 2");
    Err("EVM signing not yet implemented".to_string())
}
