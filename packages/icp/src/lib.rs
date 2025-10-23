// Provenance AI - Brain Canister
// Main orchestrator for cross-chain IP registration and audit

use candid::{CandidType, Deserialize, Principal};
use primitive_types::U256;
use std::cell::RefCell;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};

// Custom getrandom implementation for WASM
use getrandom::register_custom_getrandom;

fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    // For WASM, use IC time as entropy source (Note: not cryptographically secure!)
    // In production, use ic_cdk::api::management_canister::main::raw_rand() asynchronously
    let time = ic_cdk::api::time();
    let time_bytes = time.to_le_bytes();

    for (i, byte) in buf.iter_mut().enumerate() {
        *byte = time_bytes[i % time_bytes.len()].wrapping_add(i as u8);
    }

    Ok(())
}

register_custom_getrandom!(custom_getrandom);

// Import submodules
mod config;
mod http_util;
mod ai_util;
mod evm_util;
mod story_util;
mod nft_deployment;
mod constellation_util;

// ==============================================================================
// Data Structures
// ==============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CanisterConfig {
    pub deepseek_api_key: String,
    pub replicate_api_key: Option<String>,
    pub constellation_metagraph_url: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IPMetadata {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GenerationInput {
    pub prompt: String,
    pub metadata: IPMetadata,
}

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct GenerationOutput {
    pub image_url: String,
    pub content_hash: String,
    pub story_ip_id: String,
    pub story_tx_hash: String,
    pub story_nft_contract: String,
    pub story_token_id: u64,
    pub constellation_tx_hash: String,
    pub ai_model_id: String,
}

// ==============================================================================
// Canister State
// ==============================================================================

pub struct State {
    pub owner: Principal,
    pub evm_nonce: U256,
    pub nft_contract_address: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            owner: Principal::anonymous(),
            evm_nonce: U256::zero(),
            nft_contract_address: None,
        }
    }
}

// Thread-local state storage
thread_local! {
    static CONFIG: RefCell<Option<CanisterConfig>> = RefCell::new(None);
    static STATE: RefCell<State> = RefCell::new(State::default());
}

// ==============================================================================
// Initialization
// ==============================================================================

#[ic_cdk::init]
fn init(config: CanisterConfig) {
    ic_cdk::println!("Initializing Provenance AI Brain Canister...");

    // Store configuration
    CONFIG.with(|c| {
        *c.borrow_mut() = Some(config.clone());
    });

    // Initialize state
    // NOTE: If redeploying to an address that already has transactions,
    // you need to set the correct nonce here
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.owner = ic_cdk::caller();
        // TODO: Query this from the RPC on init in production
        state.evm_nonce = U256::from(8); // Set to current RPC nonce (updated 2025-10-22 post-IP-registration)
    });

    ic_cdk::println!("✅ Brain Canister initialized successfully");
    ic_cdk::println!("   Owner: {}", ic_cdk::caller());
    ic_cdk::println!("   Constellation URL: {}", config.constellation_metagraph_url);
}

// ==============================================================================
// Configuration Management
// ==============================================================================

#[ic_cdk::update]
fn set_owner(new_owner: Principal) {
    let caller = ic_cdk::caller();

    STATE.with(|state| {
        let current_owner = state.borrow().owner;

        // Only current owner can change owner
        if current_owner != caller {
            ic_cdk::trap("Unauthorized: Only current owner can change owner");
        }

        state.borrow_mut().owner = new_owner;
    });

    ic_cdk::println!("Owner updated to: {}", new_owner);
}

#[ic_cdk::query]
fn get_owner() -> Principal {
    STATE.with(|state| state.borrow().owner)
}

#[ic_cdk::query]
fn is_configured() -> bool {
    CONFIG.with(|c| c.borrow().is_some())
}

#[ic_cdk::update]
fn update_config(new_config: CanisterConfig) {
    let caller = ic_cdk::caller();

    // Verify caller is owner
    STATE.with(|state| {
        if state.borrow().owner != caller {
            ic_cdk::trap("Unauthorized: Only owner can update config");
        }
    });

    CONFIG.with(|c| {
        *c.borrow_mut() = Some(new_config);
    });

    ic_cdk::println!("Configuration updated by owner");
}

#[ic_cdk::query]
fn get_constellation_url() -> String {
    CONFIG.with(|c| {
        c.borrow()
            .as_ref()
            .expect("Canister not configured")
            .constellation_metagraph_url
            .clone()
    })
}

// ==============================================================================
// Helper Functions
// ==============================================================================

pub fn get_deepseek_api_key() -> String {
    CONFIG.with(|c| {
        c.borrow()
            .as_ref()
            .expect("Canister not configured")
            .deepseek_api_key
            .clone()
    })
}

pub fn get_config() -> CanisterConfig {
    CONFIG.with(|c| {
        c.borrow()
            .as_ref()
            .expect("Canister not configured")
            .clone()
    })
}

/// Query current nonce from blockchain via RPC
///
/// This queries eth_getTransactionCount from Alchemy to get the real nonce
/// Eliminates "nonce too low" errors by always using blockchain truth
///
/// # Returns
/// * `Result<u64, String>` - Current nonce from blockchain or error
pub async fn get_nonce_from_blockchain() -> Result<u64, String> {
    ic_cdk::println!("   📡 Querying nonce from blockchain via RPC...");

    // Get our EVM address
    let evm_address = evm_util::get_canister_evm_address().await?;

    // Story Protocol RPC URL from config
    let rpc_url = config::STORY_RPC_URL;

    // Build JSON-RPC request for eth_getTransactionCount
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_getTransactionCount",
        "params": [evm_address, "latest"]  // "latest" for most recent nonce
    });

    let payload_str = serde_json::to_string(&payload)
        .map_err(|e| format!("Failed to serialize RPC request: {}", e))?;

    ic_cdk::println!("   📡 RPC Request: {}", payload_str);

    // Make HTTP POST to RPC
    let response = http_util::http_post(rpc_url, &payload_str, 2_000_000_000_000).await?;

    ic_cdk::println!("   📡 RPC Response status: {}", response.status);
    ic_cdk::println!("   📡 RPC Response body: {}", response.body);

    // Parse JSON response
    let json: serde_json::Value = serde_json::from_str(&response.body)
        .map_err(|e| format!("Failed to parse RPC response: {}", e))?;

    // Extract nonce from result field
    let nonce_hex = json["result"]
        .as_str()
        .ok_or_else(|| {
            if let Some(error) = json.get("error") {
                format!("RPC error: {}", error)
            } else {
                format!("No result in RPC response: {}", response.body)
            }
        })?;

    // Convert hex string to u64
    let nonce = u64::from_str_radix(nonce_hex.trim_start_matches("0x"), 16)
        .map_err(|e| format!("Failed to parse nonce hex '{}': {}", nonce_hex, e))?;

    ic_cdk::println!("   ✅ Blockchain nonce: {}", nonce);

    // Update cached nonce in state for reference
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.evm_nonce = U256::from(nonce);
    });

    Ok(nonce)
}

/// Get and atomically increment the EVM nonce (DEPRECATED)
///
/// # Deprecated
/// This function uses a cached nonce which can get out of sync.
/// Use `get_nonce_from_blockchain()` instead for reliable nonce management.
#[deprecated(note = "Use get_nonce_from_blockchain() instead")]
pub fn get_and_increment_nonce() -> U256 {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let current_nonce = state.evm_nonce;
        state.evm_nonce += U256::one();
        current_nonce
    })
}

// ==============================================================================
// Main Orchestration Function (Stubbed for Day 1-3)
// ==============================================================================

#[ic_cdk::update]
async fn generate_and_register_ip(input: GenerationInput) -> Result<GenerationOutput, String> {
    ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    ic_cdk::println!("🚀 PROVENANCE AI ORCHESTRATION STARTED");
    ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    ic_cdk::println!("   Prompt: {}", input.prompt);
    ic_cdk::println!("   Title: {}", input.metadata.title);

    // STEP 1: AI Content Generation
    ic_cdk::println!("\n📸 STEP 1: Generating AI content...");
    let (image_url, content_hash) = ai_util::generate_ai_content(input.prompt.clone()).await?;
    ic_cdk::println!("   ✅ Image URL: {}", image_url);
    ic_cdk::println!("   ✅ Content Hash: {}", content_hash);

    // STEP 2: Register IP on Story Protocol using SPG (Mint + Register in one tx)
    ic_cdk::println!("\n📜 STEP 2: Registering IP on Story Protocol (SPG - Mint & Register)...");

    // Create metadata URI (for now, use placeholder - in production, upload to IPFS)
    let metadata_uri = format!(
        "ipfs://placeholder/{}/metadata.json",
        content_hash
    );

    let (story_tx_hash, parsed_values) = match story_util::register_ip_on_story(
        content_hash.clone(),
        metadata_uri,
    ).await {
        Ok((tx_hash, values)) => {
            ic_cdk::println!("   ✅ Transaction Hash: {}", tx_hash);
            (tx_hash, values)
        }
        Err(e) => {
            ic_cdk::println!("   ❌ Story Protocol registration failed: {}", e);
            return Err(format!("Failed to register IP on Story Protocol: {}", e));
        }
    };

    // Extract SPG NFT contract address from config (used for Constellation proof)
    let spg_nft_contract = {
        let addr = config::spg_nft_contract_address();
        format!("0x{}", hex::encode(addr.to_fixed_bytes()))
    };

    // Use parsed values if available, otherwise fallback to placeholders
    let (story_ip_id, token_id) = match parsed_values {
        Some((ip_id, tid)) => {
            ic_cdk::println!("   ✅ Using parsed IP ID and Token ID from receipt");
            (ip_id, tid)
        }
        None => {
            ic_cdk::println!("   ⚠️  Using placeholder values (receipt parsing failed)");
            (story_tx_hash.clone(), 0u64)
        }
    };

    // STEP 3: Log on Constellation DAG
    ic_cdk::println!("\n🌌 STEP 3: Logging proof on Constellation DAG...");

    let proof = constellation_util::ProofOfGeneration {
        content_hash: content_hash.clone(),
        model_name: "deepseek-chat".to_string(),
        timestamp: ic_cdk::api::time(),
        story_ip_id: story_ip_id.clone(),
        nft_contract: spg_nft_contract.clone(),
        nft_token_id: token_id,
        generator_address: "ICP-Canister".to_string(), // Placeholder for canister identity
    };

    let constellation_url = get_config().constellation_metagraph_url;

    let constellation_tx_hash = match constellation_util::log_proof_on_constellation(
        constellation_url,
        proof,
    ).await {
        Ok(tx_hash) => {
            ic_cdk::println!("   ✅ Logged on Constellation (simulated)");
            ic_cdk::println!("   TX Hash: {}", tx_hash);
            tx_hash
        }
        Err(e) => {
            ic_cdk::println!("   ⚠️  Constellation logging failed (non-critical): {}", e);
            format!("CONST-ERROR-{}", ic_cdk::api::time())
        }
    };

    ic_cdk::println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    ic_cdk::println!("✅ ORCHESTRATION COMPLETE");
    ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(GenerationOutput {
        image_url,
        content_hash,
        story_ip_id,
        story_tx_hash,
        story_nft_contract: spg_nft_contract,
        story_token_id: token_id,
        constellation_tx_hash,
        ai_model_id: "deepseek-chat".to_string(),
    })
}

// ==============================================================================
// Dispute Module (Stubbed for Phase 5)
// ==============================================================================

#[ic_cdk::update]
async fn raise_dispute(ip_id: String, evidence_ipfs_cid: String) -> Result<GenerationOutput, String> {
    ic_cdk::println!("🚨 DISPUTE RAISED");
    ic_cdk::println!("   IP ID: {}", ip_id);
    ic_cdk::println!("   Evidence: ipfs://{}", evidence_ipfs_cid);
    ic_cdk::println!("   ⚠️  [STUB] - Will be implemented in Phase 5");

    Err("Dispute module not yet implemented".to_string())
}

// ==============================================================================
// EVM Address (For Story Protocol)
// ==============================================================================

#[ic_cdk::update]
async fn get_canister_evm_address() -> String {
    match evm_util::get_canister_evm_address().await {
        Ok(address) => address,
        Err(e) => {
            ic_cdk::println!("❌ Failed to get EVM address: {}", e);
            "Error: EVM address derivation failed".to_string()
        }
    }
}

// ==============================================================================
// SimpleNFT Contract Deployment
// ==============================================================================

/// Deploy the SimpleNFT contract to Story Protocol
///
/// This should be called once during setup to deploy the NFT contract
/// that will be used for minting IP assets.
///
/// # Arguments
/// * `name` - The name of the NFT collection
/// * `symbol` - The symbol of the NFT collection
///
/// # Returns
/// * `Result<String, String>` - Deployed contract address or error
#[ic_cdk::update]
async fn deploy_nft_contract(name: String, symbol: String) -> Result<String, String> {
    // Check if already deployed
    let already_deployed = STATE.with(|state| {
        state.borrow().nft_contract_address.is_some()
    });

    if already_deployed {
        return Err("NFT contract already deployed. Use get_nft_contract_address() to retrieve it.".to_string());
    }

    ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    ic_cdk::println!("📦 DEPLOYING SIMPLENFT CONTRACT");
    ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let contract_address = nft_deployment::deploy_simple_nft(name, symbol).await?;

    // Store the contract address in state
    STATE.with(|state| {
        state.borrow_mut().nft_contract_address = Some(contract_address.clone());
    });

    ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    ic_cdk::println!("✅ NFT CONTRACT DEPLOYED");
    ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(contract_address)
}

/// Get the deployed NFT contract address
#[ic_cdk::query]
fn get_nft_contract_address() -> Option<String> {
    STATE.with(|state| state.borrow().nft_contract_address.clone())
}

/// Set NFT contract address manually (for already deployed contracts)
#[ic_cdk::update]
fn set_nft_contract_address(address: String) {
    STATE.with(|state| {
        state.borrow_mut().nft_contract_address = Some(address.clone());
    });

    ic_cdk::println!("NFT contract address set to: {}", address);
}

// ==============================================================================
// Story Protocol IP Registration
// ==============================================================================

/// Register an NFT as an IP Asset on Story Protocol
///
/// # Arguments
/// * `nft_contract_address` - The address of the NFT contract
/// * `token_id` - The token ID of the NFT to register
///
/// # Returns
/// * `Result<String, String>` - Transaction hash or error
#[ic_cdk::update]
async fn register_ip(nft_contract_address: String, token_id: u64) -> Result<String, String> {
    ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    ic_cdk::println!("📜 REGISTERING NFT AS IP ASSET ON STORY PROTOCOL");
    ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    ic_cdk::println!("   NFT Contract: {}", nft_contract_address);
    ic_cdk::println!("   Token ID: {}", token_id);

    let result = story_util::register_nft_as_ip(nft_contract_address, token_id).await;

    match &result {
        Ok(tx_hash) => {
            ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            ic_cdk::println!("✅ IP REGISTRATION COMPLETE");
            ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            ic_cdk::println!("   Transaction Hash: {}", tx_hash);
        }
        Err(e) => {
            ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            ic_cdk::println!("❌ IP REGISTRATION FAILED");
            ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            ic_cdk::println!("   Error: {}", e);
        }
    }

    result
}

// ==============================================================================
// Candid Export
// ==============================================================================

ic_cdk::export_candid!();
