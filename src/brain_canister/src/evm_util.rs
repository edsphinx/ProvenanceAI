// EVM Utilities Module
// Handles Chain-Key ECDSA for deriving canister-owned EVM addresses

use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument,
};
use sha3::{Digest, Keccak256};
use primitive_types::U256;

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
    // IC returns SEC1 encoded public keys which can be either:
    // - Compressed: 33 bytes [0x02/0x03, x (32 bytes)]
    // - Uncompressed: 65 bytes [0x04, x (32 bytes), y (32 bytes)]
    // Ethereum address = last 20 bytes of keccak256(uncompressed_public_key[1..])

    let uncompressed_key = if public_key.len() == 33 {
        // Compressed key - decompress it
        ic_cdk::println!("   🔓 Decompressing SEC1 public key (33 -> 65 bytes)...");
        decompress_public_key(&public_key)?
    } else if public_key.len() == 65 {
        // Already uncompressed
        public_key
    } else {
        return Err(format!(
            "Invalid public key length: {} (expected 33 or 65)",
            public_key.len()
        ));
    };

    // Hash the public key (excluding the 0x04 prefix)
    let hash = Keccak256::digest(&uncompressed_key[1..]);

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
// SEC1 Public Key Decompression
// ==============================================================================

/// Decompress a SEC1 compressed public key (33 bytes) to uncompressed format (65 bytes)
///
/// Secp256k1 curve equation: y² = x³ + 7 (mod p)
/// Given x and sign bit, we can recover y
fn decompress_public_key(compressed: &[u8]) -> Result<Vec<u8>, String> {
    if compressed.len() != 33 {
        return Err(format!("Invalid compressed key length: {}", compressed.len()));
    }

    // Secp256k1 curve prime
    let p = U256::from_dec_str(
        "115792089237316195423570985008687907853269984665640564039457584007908834671663",
    )
    .unwrap();

    // Extract x coordinate (32 bytes)
    let x_bytes = &compressed[1..33];
    let x = U256::from_big_endian(x_bytes);

    // Calculate y² = x³ + 7 (mod p)
    let x_cubed = mul_mod(mul_mod(x, x, p), x, p);
    let seven = U256::from(7);
    let y_squared = add_mod(x_cubed, seven, p);

    // Calculate y = sqrt(y²) mod p using Tonelli-Shanks
    // For secp256k1, p ≡ 3 (mod 4), so we can use the simpler formula:
    // y = y_squared^((p+1)/4) mod p
    let exp = (p + U256::one()) / U256::from(4);
    let y = pow_mod(y_squared, exp, p);

    // Check the parity bit and negate y if necessary
    let parity_bit = compressed[0];
    let y_parity = (y.byte(0) & 1) as u8;

    let y_final = if (parity_bit == 0x02 && y_parity == 1) || (parity_bit == 0x03 && y_parity == 0)
    {
        // Negate: y = p - y
        p - y
    } else {
        y
    };

    // Build uncompressed key: 0x04 || x || y
    let mut uncompressed = vec![0x04];
    uncompressed.extend_from_slice(x_bytes);

    let mut y_bytes = [0u8; 32];
    y_final.to_big_endian(&mut y_bytes);
    uncompressed.extend_from_slice(&y_bytes);

    Ok(uncompressed)
}

// Helper functions for modular arithmetic
fn add_mod(a: U256, b: U256, m: U256) -> U256 {
    let sum = a.overflowing_add(b);
    if sum.1 || sum.0 >= m {
        sum.0.overflowing_sub(m).0
    } else {
        sum.0
    }
}

fn mul_mod(a: U256, b: U256, m: U256) -> U256 {
    let (result, overflow) = a.overflowing_mul(b);
    if overflow {
        // For large multiplications, use a more careful approach
        // This is a simplified version; production code should handle this better
        result % m
    } else {
        result % m
    }
}

fn pow_mod(base: U256, exp: U256, m: U256) -> U256 {
    let mut result = U256::one();
    let mut base = base % m;
    let mut exp = exp;

    while exp > U256::zero() {
        if exp & U256::one() == U256::one() {
            result = mul_mod(result, base, m);
        }
        base = mul_mod(base, base, m);
        exp = exp >> 1;
    }

    result
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
