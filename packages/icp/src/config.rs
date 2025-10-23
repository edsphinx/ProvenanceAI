// Configuration constants for Provenance AI
//
// NOTE: This file is designed to be extensible for multi-provider support (Phase 6)
// Future enums for AIProvider, QualityLevel, ContentType, SubscriptionTier are included
// as commented code below for reference.
// See: docs/architecture/MULTI_AI_PROVIDER_DESIGN.md

use primitive_types::H160;
use std::str::FromStr;

// Uncomment when implementing Phase 6
// use candid::{CandidType, Deserialize};

// ==============================================================================
// Story Protocol Configuration (Aeneid Testnet)
// ==============================================================================

/// Story Protocol Aeneid Testnet RPC URL
/// Using official Story RPC (proven to work in Phase 2)
pub const STORY_RPC_URL: &str = "https://aeneid.storyrpc.io";

/// Story Protocol Aeneid Testnet Chain ID
pub const STORY_CHAIN_ID: u64 = 1315;

/// Gas settings for Story Protocol transactions
pub const GAS_LIMIT: u64 = 3_000_000;
pub const GAS_PRICE: u64 = 20_000_000_000; // 20 Gwei

// ==============================================================================
// Story Protocol Contract Addresses (Aeneid Testnet - Chain ID 1315)
// ==============================================================================
// Official deployment addresses from:
// https://github.com/storyprotocol/protocol-periphery-v1/blob/main/deploy-out/deployment-1315.json
// Documentation: https://docs.story.foundation/network/connect/aeneid

/// IPAssetRegistry contract address (DEPRECATED - Use RegistrationWorkflows instead)
pub fn ip_asset_registry_address() -> H160 {
    H160::from_str("0x77319B4031e6eF1250907aa00018B8B1c67a244b")
        .expect("Invalid IPAssetRegistry address")
}

/// RegistrationWorkflows contract address (SPG - Story Protocol Gateway)
/// This is the correct contract for registering IP via mintAndRegisterIp
#[allow(dead_code)]
pub fn registration_workflows_address() -> H160 {
    H160::from_str("0xbe39E1C756e921BD25DF86e7AAa31106d1eb0424")
        .expect("Invalid RegistrationWorkflows address")
}

/// Public SPG NFT Contract (Aeneid testnet)
/// This is a public collection provided by Story Protocol for testing
#[allow(dead_code)]
pub fn spg_nft_contract_address() -> H160 {
    H160::from_str("0xc32A8a0FF3beDDDa58393d022aF433e78739FAbc")
        .expect("Invalid SPG NFT Contract address")
}

/// LicensingModule contract address
#[allow(dead_code)]
pub fn licensing_module_address() -> H160 {
    H160::from_str("0x04fbd8a2e56dd85CFD5500A4A4DfA955B9f1dE6f")
        .expect("Invalid LicensingModule address")
}

/// RoyaltyModule contract address
#[allow(dead_code)]
pub fn royalty_module_address() -> H160 {
    // Placeholder - replace with actual address
    H160::from_str("0xD2f60c40fEbccf6311f8B47c4f2Ec6b040400086")
        .expect("Invalid RoyaltyModule address")
}

/// DisputeModule contract address
#[allow(dead_code)]
pub fn dispute_module_address() -> H160 {
    // Placeholder - replace with actual address
    H160::from_str("0x9b7A9c70AFF961C799110954fc06F3093aeb94C5")
        .expect("Invalid DisputeModule address")
}

// ==============================================================================
// Parent AI Model Configuration
// ==============================================================================

/// Parent AI Model IP ID on Story Protocol
/// This must be set after registering the AI Model as an IP Asset
/// Run: npx ts-node scripts/register_ai_agent.ts
#[allow(dead_code)]
pub const PARENT_AI_MODEL_IP_ID: &str = "REPLACE_AFTER_REGISTRATION";

// ==============================================================================
// ICP Configuration
// ==============================================================================

/// ECDSA key for signing EVM transactions
/// Use "test_key_1" for playground/mainnet, "dfx_test_key" for local only
pub const ECDSA_KEY_NAME: &str = "test_key_1";

// ==============================================================================
// ckBTC Configuration (Testnet)
// ==============================================================================

/// ckBTC Ledger Canister ID (Testnet)
#[allow(dead_code)]
pub const CKBTC_LEDGER_CANISTER_ID: &str = "mxzaz-hqaaa-aaaar-qaada-cai";

/// ckBTC Minter Canister ID (Testnet)
#[allow(dead_code)]
pub const CKBTC_MINTER_CANISTER_ID: &str = "mqygn-kiaaa-aaaar-qaadq-cai";

/// Payment amount required for IP generation (in satoshis)
#[allow(dead_code)]
pub const REQUIRED_PAYMENT_SATOSHIS: u64 = 100_000; // 0.001 ckBTC

/// Royalty percentage for parent AI model (10%)
#[allow(dead_code)]
pub const ROYALTY_PERCENTAGE: u64 = 10;

// ==============================================================================
// Phase 6: Multi-AI Provider Configuration
// ==============================================================================
//
// TODO Phase 6: Uncomment these enums when implementing multi-provider support
//
// These enums are ready to use for Phase 6 implementation
// They provide the foundation for the multi-provider architecture
// documented in docs/architecture/MULTI_AI_PROVIDER_DESIGN.md

/*
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum AIProvider {
    DeepSeek,      // Cheapest, fastest (current implementation)
    OpenAI,        // GPT-4 for high quality text
    Anthropic,     // Claude for reasoning tasks
    Replicate,     // Stable Diffusion for images
    DALLE,         // DALL-E 3 for premium images
    Midjourney,    // Future: via unofficial API
    RunwayML,      // Future: video generation
    Auto,          // Automatic provider selection
}

impl Default for AIProvider {
    fn default() -> Self {
        AIProvider::Auto
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum QualityLevel {
    Draft,      // Cheapest, fastest
    Standard,   // Balanced quality/cost
    Premium,    // Best quality, highest cost
    Ultra,      // Future: custom trained models
}

impl Default for QualityLevel {
    fn default() -> Self {
        QualityLevel::Standard
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum ContentType {
    Text,       // Text enhancement/generation
    Image,      // Image generation
    Video,      // Video generation (future)
    Audio,      // Audio generation (future)
}

impl Default for ContentType {
    fn default() -> Self {
        ContentType::Image
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum SubscriptionTier {
    Free,           // Limited access, DeepSeek only
    Pro,            // $20/month, all providers
    Teams,          // $30/user/month, priority queue
    Enterprise,     // Custom pricing
}

impl Default for SubscriptionTier {
    fn default() -> Self {
        SubscriptionTier::Free
    }
}

// Provider cost configuration (in USD per generation)
impl AIProvider {
    pub fn cost_per_generation(&self, quality: &QualityLevel) -> f64 {
        match (self, quality) {
            (AIProvider::DeepSeek, _) => 0.01,
            (AIProvider::OpenAI, QualityLevel::Draft) => 0.05,
            (AIProvider::OpenAI, QualityLevel::Standard) => 0.10,
            (AIProvider::OpenAI, QualityLevel::Premium) => 0.20,
            (AIProvider::Anthropic, _) => 0.08,
            (AIProvider::Replicate, _) => 0.05,
            (AIProvider::DALLE, QualityLevel::Standard) => 0.15,
            (AIProvider::DALLE, QualityLevel::Premium) => 0.20,
            (AIProvider::RunwayML, _) => 2.00,
            (AIProvider::Auto, _) => 0.01, // Minimum cost
            _ => 0.10, // Default
        }
    }

    pub fn api_url(&self) -> &str {
        match self {
            AIProvider::DeepSeek => "https://api.deepseek.com/v1/chat/completions",
            AIProvider::OpenAI => "https://api.openai.com/v1/chat/completions",
            AIProvider::Anthropic => "https://api.anthropic.com/v1/messages",
            AIProvider::Replicate => "https://api.replicate.com/v1/predictions",
            AIProvider::DALLE => "https://api.openai.com/v1/images/generations",
            _ => "",
        }
    }
}
*/
