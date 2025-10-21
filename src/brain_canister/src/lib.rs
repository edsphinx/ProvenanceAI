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

// Re-export for convenience
use config::*;

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
    pub constellation_tx_hash: String,
    pub ai_model_id: String,
}

// ==============================================================================
// Canister State
// ==============================================================================

pub struct State {
    pub owner: Principal,
    pub evm_nonce: U256,
}

impl Default for State {
    fn default() -> Self {
        Self {
            owner: Principal::anonymous(),
            evm_nonce: U256::zero(),
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
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.owner = ic_cdk::caller();
        state.evm_nonce = U256::zero();
    });

    ic_cdk::println!("✅ Brain Canister initialized successfully");
    ic_cdk::println!("   Owner: {}", ic_cdk::caller());
    ic_cdk::println!("   Constellation URL: {}", config.constellation_metagraph_url);
}

// ==============================================================================
// Configuration Management
// ==============================================================================

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

    // STEP 2: Register on Story Protocol (Stubbed for now)
    ic_cdk::println!("\n📜 STEP 2: Registering IP on Story Protocol...");
    ic_cdk::println!("   ⚠️  [STUB] - Will be implemented in Phase 2");
    let story_ip_id = "STORY_IP_ID_STUB_PHASE2".to_string();

    // STEP 3: Log on Constellation (Stubbed for now)
    ic_cdk::println!("\n🌌 STEP 3: Logging proof on Constellation...");
    ic_cdk::println!("   ⚠️  [STUB] - Will be implemented in Phase 3");
    let constellation_tx_hash = "CONSTELLATION_TX_STUB_PHASE3".to_string();

    ic_cdk::println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    ic_cdk::println!("✅ ORCHESTRATION COMPLETE");
    ic_cdk::println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(GenerationOutput {
        image_url,
        content_hash,
        story_ip_id,
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
// Candid Export
// ==============================================================================

ic_cdk::export_candid!();
