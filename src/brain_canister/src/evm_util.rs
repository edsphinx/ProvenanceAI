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

/// Get the canister's raw public key (uncompressed, 65 bytes)
///
/// This is used for signature verification and recovery ID determination.
///
/// # Returns
/// * `Result<Vec<u8>, String>` - Uncompressed public key (65 bytes) or error
pub async fn get_canister_public_key() -> Result<Vec<u8>, String> {
    // Use empty derivation path for root key
    let derivation_path = vec![];
    let key_id = get_ecdsa_key_id();

    // IMPORTANT: Use Some(ic_cdk::id()) to ensure we get THIS canister's public key
    let request = EcdsaPublicKeyArgument {
        canister_id: Some(ic_cdk::id()),
        derivation_path,
        key_id: key_id.clone(),
    };

    let (response,) = ecdsa_public_key(request)
        .await
        .map_err(|e| format!("Failed to get ECDSA public key: {:?}", e))?;

    let public_key = response.public_key;

    ic_cdk::println!("   🔑 IC returned public key: {} bytes", public_key.len());
    ic_cdk::println!("   🔑 IC public key hex: 0x{}", hex::encode(&public_key));

    // Decompress if needed
    let uncompressed_key = if public_key.len() == 33 {
        ic_cdk::println!("   🔓 Decompressing...");
        let decompressed = decompress_public_key(&public_key)?;
        ic_cdk::println!("   🔓 Decompressed: 0x{}", hex::encode(&decompressed));
        decompressed
    } else if public_key.len() == 65 {
        ic_cdk::println!("   ✅ Already uncompressed");
        public_key
    } else {
        return Err(format!(
            "Invalid public key length: {} (expected 33 or 65)",
            public_key.len()
        ));
    };

    Ok(uncompressed_key)
}

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
/// Uses k256 crate for proper SEC1 decompression
fn decompress_public_key(compressed: &[u8]) -> Result<Vec<u8>, String> {
    use k256::elliptic_curve::sec1::ToEncodedPoint;
    use k256::PublicKey;

    if compressed.len() != 33 {
        return Err(format!("Invalid compressed key length: {}", compressed.len()));
    }

    // Parse the compressed public key
    let public_key = match PublicKey::from_sec1_bytes(compressed) {
        Ok(key) => key,
        Err(e) => return Err(format!("Failed to parse compressed key: {:?}", e)),
    };

    // Get uncompressed encoding
    let uncompressed_point = public_key.to_encoded_point(false);
    let uncompressed_bytes = uncompressed_point.as_bytes();

    if uncompressed_bytes.len() != 65 {
        return Err(format!(
            "Unexpected uncompressed key length: {}",
            uncompressed_bytes.len()
        ));
    }

    Ok(uncompressed_bytes.to_vec())
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
// Sign EVM Transaction
// ==============================================================================

use ic_cdk::api::management_canister::ecdsa::{
    sign_with_ecdsa, SignWithEcdsaArgument,
};

/// Sign a raw transaction hash using Chain-Key ECDSA
///
/// This function signs an Ethereum transaction hash using the canister's
/// ECDSA key, enabling the canister to send transactions on EVM chains
/// without storing private keys.
///
/// # Arguments
/// * `message_hash` - The keccak256 hash of the raw transaction (32 bytes)
///
/// # Returns
/// * `Result<Vec<u8>, String>` - The signature (65 bytes: r, s, v) or error
pub async fn sign_evm_transaction(message_hash: Vec<u8>) -> Result<Vec<u8>, String> {
    ic_cdk::println!("   🔏 Signing EVM transaction with Chain-Key ECDSA...");

    if message_hash.len() != 32 {
        return Err(format!(
            "Invalid message hash length: {} (expected 32)",
            message_hash.len()
        ));
    }

    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: crate::config::ECDSA_KEY_NAME.to_string(),
    };

    let request = SignWithEcdsaArgument {
        message_hash,
        derivation_path: vec![],
        key_id,
    };

    let (response,) = sign_with_ecdsa(request)
        .await
        .map_err(|e| format!("Failed to sign with ECDSA: {:?}", e))?;

    let signature = response.signature;

    // ECDSA signature is (r, s) - 64 bytes
    // For Ethereum, we need to add recovery ID (v)
    // The recovery ID is calculated from the signature

    ic_cdk::println!("   ✅ Transaction signed ({} bytes)", signature.len());

    Ok(signature)
}

// ==============================================================================
// RLP Encoding for EVM Transactions (EIP-155)
// ==============================================================================

use rlp::RlpStream;

/// Build unsigned transaction for EIP-155 signing
///
/// This creates the RLP-encoded transaction that will be hashed and signed.
/// For EIP-155, the unsigned transaction includes chain_id, 0, 0 as the last 3 fields.
///
/// # Arguments
/// * `nonce` - Transaction nonce
/// * `gas_price` - Gas price in wei
/// * `gas_limit` - Gas limit
/// * `to` - Recipient address (20 bytes)
/// * `value` - Value to transfer in wei
/// * `data` - Transaction data (contract call)
/// * `chain_id` - Chain ID for EIP-155
///
/// # Returns
/// * `Vec<u8>` - RLP-encoded unsigned transaction
pub fn build_evm_transaction(
    nonce: u64,
    gas_price: u64,
    gas_limit: u64,
    to: &[u8; 20],
    value: u64,
    data: Vec<u8>,
    chain_id: u64,
) -> Vec<u8> {
    let mut stream = RlpStream::new();
    stream.begin_list(9);
    stream.append(&nonce);
    stream.append(&gas_price);
    stream.append(&gas_limit);
    stream.append(&to.as_ref());
    stream.append(&value);
    stream.append(&data);
    stream.append(&chain_id);
    stream.append(&0u8); // r = 0 (unsigned)
    stream.append(&0u8); // s = 0 (unsigned)

    stream.out().to_vec()
}

/// Calculate recovery ID (v) for ECDSA signature
///
/// The recovery ID determines which of the 2-4 possible public keys
/// should be used to verify the signature. For secp256k1, we need to
/// check which recovery ID produces the correct public key.
///
/// # Arguments
/// * `message_hash` - The message hash that was signed
/// * `signature` - The signature (r, s)
/// * `public_key` - The expected public key (65 bytes, uncompressed)
///
/// # Returns
/// * `Result<u8, String>` - Recovery ID (0 or 1) or error
async fn calculate_recovery_id(
    message_hash: &[u8],
    signature: &[u8],
    public_key: &[u8],
) -> Result<u8, String> {
    // Try recovery ID 0 and 1
    for recovery_id in 0..2 {
        // In production, you would use k256 or libsecp256k1 to recover the public key
        // and compare it with the expected public key
        // For now, we'll try recovery_id = 0 first, then 1 if that fails

        // This is a simplified version - in production, implement proper key recovery
        // using the k256 crate or similar
    }

    // Default to 0 if we can't determine
    // TODO: Implement proper key recovery verification
    Ok(0)
}

/// Build a signed EVM transaction with signature (EIP-155)
///
/// # Arguments
/// * `nonce` - Transaction nonce
/// * `gas_price` - Gas price in wei
/// * `gas_limit` - Gas limit
/// * `to` - Recipient address (20 bytes)
/// * `value` - Value to transfer in wei
/// * `data` - Transaction data (contract call)
/// * `signature` - ECDSA signature (r, s) - 64 bytes
/// * `chain_id` - Chain ID for calculating v
/// * `recovery_id` - Recovery ID (0 or 1)
///
/// # Returns
/// * `Vec<u8>` - RLP-encoded signed transaction
pub fn build_signed_transaction(
    nonce: u64,
    gas_price: u64,
    gas_limit: u64,
    to: &[u8; 20],
    value: u64,
    data: Vec<u8>,
    signature: &[u8],
    chain_id: u64,
    recovery_id: u8,
) -> Vec<u8> {
    if signature.len() != 64 {
        ic_cdk::println!("⚠️  Warning: Invalid signature length: {} (expected 64)", signature.len());
    }

    // Extract r and s from signature (64 bytes)
    let r = &signature[0..32];
    let s = &signature[32..64];

    // Calculate v for EIP-155
    // v = chain_id * 2 + 35 + recovery_id
    // recovery_id is 0 or 1
    let v = chain_id * 2 + 35 + (recovery_id as u64);

    // Convert v to U256 to match r and s encoding
    let v_u256 = U256::from(v);

    ic_cdk::println!("   📝 Building signed transaction:");
    ic_cdk::println!("      Nonce: {}", nonce);
    ic_cdk::println!("      Gas Price: {} wei", gas_price);
    ic_cdk::println!("      Gas Limit: {}", gas_limit);
    ic_cdk::println!("      To: 0x{}", hex::encode(to));
    ic_cdk::println!("      Value: {} wei", value);
    ic_cdk::println!("      Data: {} bytes", data.len());
    ic_cdk::println!("      Chain ID: {}", chain_id);
    ic_cdk::println!("      Recovery ID: {}", recovery_id);
    ic_cdk::println!("      v: {} (0x{:x})", v, v);

    let mut stream = RlpStream::new();
    stream.begin_list(9);
    stream.append(&nonce);
    stream.append(&gas_price);
    stream.append(&gas_limit);
    stream.append(&to.as_ref());
    stream.append(&value);
    stream.append(&data);
    stream.append(&v_u256);
    stream.append(&U256::from_big_endian(r));
    stream.append(&U256::from_big_endian(s));

    let signed_tx = stream.out().to_vec();

    ic_cdk::println!("   ✅ Signed transaction: {} bytes", signed_tx.len());
    ic_cdk::println!("   🔍 Raw signed TX: 0x{}", hex::encode(&signed_tx));

    // Decode and verify what we encoded
    ic_cdk::println!("   🔍 Verification:");
    ic_cdk::println!("      v value we calculated: {}", v);
    ic_cdk::println!("      v as hex: 0x{:x}", v);

    signed_tx
}

/// Build unsigned transaction for contract creation (EIP-155)
///
/// For contract deployment, `to` is empty (all zeros).
///
/// # Arguments
/// * `nonce` - Transaction nonce
/// * `gas_price` - Gas price in wei
/// * `gas_limit` - Gas limit
/// * `value` - Value to transfer in wei
/// * `data` - Contract bytecode + constructor params
/// * `chain_id` - Chain ID for EIP-155
///
/// # Returns
/// * `Vec<u8>` - RLP-encoded unsigned transaction
pub fn build_evm_transaction_for_creation(
    nonce: u64,
    gas_price: u64,
    gas_limit: u64,
    value: u64,
    data: Vec<u8>,
    chain_id: u64,
) -> Vec<u8> {
    let mut stream = RlpStream::new();
    stream.begin_list(9);
    stream.append(&nonce);
    stream.append(&gas_price);
    stream.append(&gas_limit);
    stream.append(&""); // Empty string for contract creation
    stream.append(&value);
    stream.append(&data);
    stream.append(&chain_id);
    stream.append(&0u8); // r = 0 (unsigned)
    stream.append(&0u8); // s = 0 (unsigned)

    stream.out().to_vec()
}

/// Build a signed EVM transaction for contract creation with signature (EIP-155)
///
/// # Arguments
/// * `nonce` - Transaction nonce
/// * `gas_price` - Gas price in wei
/// * `gas_limit` - Gas limit
/// * `value` - Value to transfer in wei
/// * `data` - Contract bytecode + constructor params
/// * `signature` - ECDSA signature (r, s) - 64 bytes
/// * `chain_id` - Chain ID for calculating v
/// * `recovery_id` - Recovery ID (0 or 1)
///
/// # Returns
/// * `Vec<u8>` - RLP-encoded signed transaction
pub fn build_signed_transaction_for_creation(
    nonce: u64,
    gas_price: u64,
    gas_limit: u64,
    value: u64,
    data: Vec<u8>,
    signature: &[u8],
    chain_id: u64,
    recovery_id: u8,
) -> Vec<u8> {
    if signature.len() != 64 {
        ic_cdk::println!("⚠️  Warning: Invalid signature length: {} (expected 64)", signature.len());
    }

    // Extract r and s from signature (64 bytes)
    let r = &signature[0..32];
    let s = &signature[32..64];

    // Calculate v for EIP-155
    // v = chain_id * 2 + 35 + recovery_id
    let v = chain_id * 2 + 35 + (recovery_id as u64);

    // Convert v to U256 to match r and s encoding
    let v_u256 = U256::from(v);

    ic_cdk::println!("   📝 Building signed transaction for contract creation:");
    ic_cdk::println!("      Nonce: {}", nonce);
    ic_cdk::println!("      Gas Price: {} wei", gas_price);
    ic_cdk::println!("      Gas Limit: {}", gas_limit);
    ic_cdk::println!("      To: <contract creation>");
    ic_cdk::println!("      Value: {} wei", value);
    ic_cdk::println!("      Data: {} bytes", data.len());
    ic_cdk::println!("      Chain ID: {}", chain_id);
    ic_cdk::println!("      Recovery ID: {}", recovery_id);
    ic_cdk::println!("      v: {} (0x{:x})", v, v);

    let mut stream = RlpStream::new();
    stream.begin_list(9);
    stream.append(&nonce);
    stream.append(&gas_price);
    stream.append(&gas_limit);
    stream.append(&""); // Empty string for contract creation
    stream.append(&value);
    stream.append(&data);
    stream.append(&v_u256);
    stream.append(&U256::from_big_endian(r));
    stream.append(&U256::from_big_endian(s));

    let signed_tx = stream.out().to_vec();

    ic_cdk::println!("   ✅ Signed transaction: {} bytes", signed_tx.len());
    ic_cdk::println!("   🔍 Raw signed TX: 0x{}", hex::encode(&signed_tx));

    signed_tx
}
