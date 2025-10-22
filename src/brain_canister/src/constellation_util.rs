// Constellation Network Integration
// Logs proof of generation data on Constellation DAG

use crate::http_util;
use candid::{CandidType, Deserialize};
use serde_json::json;

/// Proof of Generation data structure for Constellation
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ProofOfGeneration {
    /// Content hash of the generated AI content
    pub content_hash: String,
    /// AI model used (e.g., "deepseek-chat")
    pub model_name: String,
    /// Timestamp of generation
    pub timestamp: u64,
    /// Story Protocol IP ID
    pub story_ip_id: String,
    /// NFT contract address
    pub nft_contract: String,
    /// NFT token ID
    pub nft_token_id: u64,
    /// Generator address (ICP canister EVM address)
    pub generator_address: String,
}

/// Log a proof of generation on Constellation DAG
///
/// # Arguments
/// * `metagraph_url` - The Constellation metagraph L1 endpoint URL (e.g., "http://198.144.183.32:9400")
/// * `proof` - The proof data to log
///
/// # Returns
/// * `Result<String, String>` - Transaction hash from Constellation or error message
///
/// # Phase 3 Implementation
/// This now performs real HTTP POST to the Constellation metagraph Data L1 endpoint.
/// Falls back to simulated hash if HTTP request fails (for development/testing).
pub async fn log_proof_on_constellation(
    metagraph_url: String,
    proof: ProofOfGeneration,
) -> Result<String, String> {
    ic_cdk::println!("   [Constellation] Preparing to log proof on DAG...");
    ic_cdk::println!("   [Constellation] Metagraph URL: {}", metagraph_url);
    ic_cdk::println!("   [Constellation] Content Hash: {}", proof.content_hash);
    ic_cdk::println!("   [Constellation] Story IP ID: {}", proof.story_ip_id);

    // Build JSON payload for Constellation Data L1
    let payload = build_constellation_data_payload(&proof);
    let payload_str = serde_json::to_string(&payload)
        .map_err(|e| format!("Failed to serialize payload: {}", e))?;

    ic_cdk::println!("   [Constellation] Payload: {}", payload_str);

    // Construct the /data endpoint URL
    let url = if metagraph_url.ends_with('/') {
        format!("{}data", metagraph_url)
    } else {
        format!("{}/data", metagraph_url)
    };

    ic_cdk::println!("   [Constellation] POST {}", url);

    // Attempt real HTTP POST to Constellation metagraph
    match http_util::http_post(&url, &payload_str, 2_000_000_000_000).await {
        Ok(response) => {
            ic_cdk::println!("   [Constellation] ✅ Response received from metagraph");
            ic_cdk::println!("   [Constellation] Status: {}", response.status);
            ic_cdk::println!("   [Constellation] Body: {}", response.body);

            // Try to extract transaction hash from response
            match extract_tx_hash_from_response(&response.body) {
                Ok(tx_hash) => {
                    ic_cdk::println!("   [Constellation] ✅ TX Hash: {}", tx_hash);
                    Ok(tx_hash)
                }
                Err(e) => {
                    ic_cdk::println!("   [Constellation] ⚠️  Could not extract hash: {}", e);
                    ic_cdk::println!("   [Constellation] 📝 Using deterministic hash as fallback");
                    // Fallback to simulated hash
                    let fallback_hash = generate_simulated_tx_hash(&proof);
                    Ok(format!("FALLBACK-{}", fallback_hash))
                }
            }
        }
        Err(e) => {
            ic_cdk::println!("   [Constellation] ❌ HTTP POST failed: {}", e);
            ic_cdk::println!("   [Constellation] 📝 Using simulated hash for development");

            // For development: return simulated hash if HTTP fails
            // This allows testing without deployed metagraph
            let simulated_hash = generate_simulated_tx_hash(&proof);
            Ok(format!("SIMULATED-{}", simulated_hash))
        }
    }
}

/// Generate a simulated transaction hash for MVP
/// In production, this will be replaced with the actual Constellation tx hash
fn generate_simulated_tx_hash(proof: &ProofOfGeneration) -> String {
    use sha2::{Digest, Sha256};

    // Create a deterministic hash based on proof data
    let data = format!(
        "{}:{}:{}:{}",
        proof.content_hash,
        proof.story_ip_id,
        proof.nft_contract,
        proof.nft_token_id
    );

    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let hash = hasher.finalize();

    // Return as hex string with "CONST-" prefix to indicate it's a Constellation hash
    format!("CONST-{}", hex::encode(&hash[..16]))
}

/// Build the JSON payload for Constellation Data L1 submission
/// Constructs the exact format expected by the ProofOfGeneration metagraph
fn build_constellation_data_payload(proof: &ProofOfGeneration) -> serde_json::Value {
    json!({
        "value": {
            "ProofOfGeneration": {
                "contentHash": proof.content_hash,
                "modelName": proof.model_name,
                "timestamp": proof.timestamp,
                "storyIpId": proof.story_ip_id,
                "nftContract": proof.nft_contract,
                "nftTokenId": proof.nft_token_id,
                "generatorAddress": proof.generator_address
            }
        },
        // For MVP, empty proofs array (no signature validation)
        // Phase 4: Implement proper DAG wallet signing
        "proofs": []
    })
}

/// Extract transaction hash from Constellation response
/// Tries multiple possible response formats
fn extract_tx_hash_from_response(response_body: &str) -> Result<String, String> {
    // Try to parse as JSON
    match serde_json::from_str::<serde_json::Value>(response_body) {
        Ok(json) => {
            // Try common hash field names
            if let Some(hash) = json.get("hash").and_then(|h| h.as_str()) {
                return Ok(format!("CONST-{}", hash));
            }
            if let Some(hash) = json.get("transactionHash").and_then(|h| h.as_str()) {
                return Ok(format!("CONST-{}", hash));
            }
            if let Some(hash) = json.get("tx_hash").and_then(|h| h.as_str()) {
                return Ok(format!("CONST-{}", hash));
            }
            if let Some(hash) = json.get("txHash").and_then(|h| h.as_str()) {
                return Ok(format!("CONST-{}", hash));
            }

            // If no hash found, return the whole response for debugging
            Err(format!("No hash field found in response: {}", response_body))
        }
        Err(_) => {
            // If not JSON, return the body as-is (might be plain text hash)
            if !response_body.is_empty() && response_body.len() < 100 {
                Ok(format!("CONST-{}", response_body.trim()))
            } else {
                Err(format!("Could not parse response as JSON: {}", response_body))
            }
        }
    }
}

/// Query Constellation for proof verification (Phase 3+)
/// This will check if a proof exists on the Constellation DAG
#[allow(dead_code)]
pub async fn verify_proof_on_constellation(
    metagraph_url: String,
    content_hash: String,
) -> Result<bool, String> {
    // TODO Phase 3: Implement actual verification query
    // GET {metagraph_url}/snapshots/latest
    // or custom query endpoint for proof lookups

    ic_cdk::println!("   [Constellation] ⚠️  STUB: Proof verification not yet implemented");
    ic_cdk::println!("   [Constellation] Content Hash to verify: {}", content_hash);
    ic_cdk::println!("   [Constellation] Metagraph URL: {}", metagraph_url);

    Err("Proof verification will be implemented in Phase 3".to_string())
}
