// AI Content Generation Module
// Integrates with DeepSeek API for AI-generated content
//
// NOTE: This module is designed to be extensible for multi-provider support (Phase 6)
// Future: Support for OpenAI GPT-4, Anthropic Claude, Replicate, DALL-E 3
// See: docs/architecture/MULTI_AI_PROVIDER_DESIGN.md

use crate::http_util::{auth_header, json_header, make_http_request};
use crate::get_deepseek_api_key;
use ic_cdk::api::management_canister::http_request::HttpMethod;
use serde_json::json;
use sha3::{Digest, Keccak256};

// ==============================================================================
// Constants
// ==============================================================================

const DEEPSEEK_API_URL: &str = "https://api.deepseek.com/v1/chat/completions";
const DEEPSEEK_MODEL: &str = "deepseek-chat";

// ==============================================================================
// Main AI Generation Function
// ==============================================================================

/// Generate AI content using configured AI provider
///
/// Current (Phase 1): Uses DeepSeek for prompt enhancement
/// Future (Phase 6): Will support multiple providers:
///   - DeepSeek (cheapest, fastest)
///   - OpenAI GPT-4 (high quality text)
///   - Anthropic Claude (reasoning)
///   - Replicate/Stable Diffusion (images)
///   - DALL-E 3 (premium images)
///
/// The function will auto-select the best provider based on:
///   - User preferences (subscription tier)
///   - Content type (text vs image vs video)
///   - Cost constraints
///   - Quality requirements
///
/// # Arguments
/// * `prompt` - User's text prompt for content generation
///
/// # Returns
/// * `Result<(String, String), String>` - (content_url, content_hash) or error
///
/// # Future Parameters (Phase 6)
/// TODO: Add `provider: Option<AIProvider>` parameter
/// TODO: Add `quality: QualityLevel` parameter
/// TODO: Add `max_cost_usd: Option<f64>` parameter
pub async fn generate_ai_content(prompt: String) -> Result<(String, String), String> {
    // TODO Phase 6: Replace with provider selection logic
    // let provider = auto_select_provider(&prompt, quality).await?;
    // match provider { ... }

    ic_cdk::println!("   🤖 AI Model: DeepSeek (Phase 6: will support multi-provider)");
    ic_cdk::println!("   📝 Prompt: {}", prompt);

    // Step 1: Enhance prompt with DeepSeek
    let enhanced_prompt = match enhance_prompt_with_deepseek(prompt.clone()).await {
        Ok(enhanced) => {
            ic_cdk::println!("   ✨ Enhanced prompt: {}", enhanced);
            enhanced
        }
        Err(e) => {
            ic_cdk::println!("   ⚠️  Prompt enhancement failed: {}", e);
            ic_cdk::println!("   Using original prompt instead");
            prompt
        }
    };

    // Step 2: For hackathon, we'll use a placeholder image
    // TODO Phase 1.5: Integrate with Replicate/Stable Diffusion for real image generation
    let image_url = generate_placeholder_image_url(&enhanced_prompt);

    // Step 3: Generate content hash (SHA-256 of the prompt + timestamp)
    let content = format!("{}:{}", enhanced_prompt, ic_cdk::api::time());
    let hash_bytes = Keccak256::digest(content.as_bytes());
    let content_hash = format!("0x{}", hex::encode(hash_bytes));

    ic_cdk::println!("   🖼️  Image URL: {}", image_url);
    ic_cdk::println!("   #️⃣  Content Hash: {}", content_hash);

    Ok((image_url, content_hash))
}

// ==============================================================================
// DeepSeek API Integration
// ==============================================================================

/// Enhance user prompt using DeepSeek chat model
///
/// # Arguments
/// * `user_prompt` - Original user prompt
///
/// # Returns
/// * `Result<String, String>` - Enhanced prompt or error
async fn enhance_prompt_with_deepseek(user_prompt: String) -> Result<String, String> {
    ic_cdk::println!("   📡 Calling DeepSeek API...");

    // Get API key from config
    let api_key = get_deepseek_api_key();

    // Build request payload
    let payload = json!({
        "model": DEEPSEEK_MODEL,
        "messages": [
            {
                "role": "system",
                "content": "You are an expert at writing prompts for AI image generation. Transform the user's request into a detailed, artistic prompt for Stable Diffusion. Keep it concise (max 50 words) but vivid. Focus on visual details, style, lighting, and composition."
            },
            {
                "role": "user",
                "content": user_prompt
            }
        ],
        "temperature": 0.7,
        "max_tokens": 100
    });

    // Prepare headers
    let headers = vec![
        json_header(),
        auth_header(&api_key),
    ];

    // Make HTTP outcall
    let response_body = make_http_request(
        DEEPSEEK_API_URL.to_string(),
        HttpMethod::POST,
        headers,
        Some(payload.to_string().into_bytes()),
    )
    .await?;

    // Parse response
    let response_str = String::from_utf8(response_body)
        .map_err(|e| format!("Failed to parse response as UTF-8: {}", e))?;

    let response_json: serde_json::Value = serde_json::from_str(&response_str)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Extract enhanced prompt
    let enhanced_prompt = response_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("No content in DeepSeek response")?
        .trim()
        .to_string();

    Ok(enhanced_prompt)
}

// ==============================================================================
// Placeholder Image Generation (Hackathon Version)
// ==============================================================================

/// Generate a placeholder image URL
///
/// For the hackathon, this creates a URL to a placeholder image service
/// In production, this would be replaced with actual image generation + IPFS upload
///
/// # Arguments
/// * `prompt` - Enhanced prompt text
///
/// # Returns
/// * `String` - Placeholder image URL
fn generate_placeholder_image_url(prompt: &str) -> String {
    // Use a placeholder image service with the prompt as seed
    let _prompt_hash = format!("{:x}", md5::compute(prompt.as_bytes()));

    // Option 1: Use picsum.photos with seeded random
    // format!("https://picsum.photos/seed/{}/1024/1024", prompt_hash)

    // Option 2: Use a static IPFS image (recommended for consistency)
    // This is the image from the Story AI Agent example
    "https://ipfs.io/ipfs/bafybeigi3k77t5h5aefwpzvx3uiomuavdvqwn5rb5uhd7i7xcq466wvute".to_string()

    // Option 3: Generate a data URL with placeholder text
    // format!("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'><text>{}</text></svg>", prompt)
}

// ==============================================================================
// Future: Multi-Provider Integration (Phase 6)
// ==============================================================================
//
// When implementing Phase 6, create separate modules for each provider:
//
// Suggested structure:
// src/brain_canister/src/ai_providers/
// ├── mod.rs                 - Provider trait and auto-selection
// ├── deepseek.rs            - Current implementation (move from here)
// ├── openai.rs              - GPT-4 for premium text enhancement
// ├── anthropic.rs           - Claude for reasoning tasks
// ├── replicate.rs           - Stable Diffusion for images
// ├── dalle.rs               - DALL-E 3 for premium images
// └── auto_selector.rs       - Intelligent provider selection
//
// See: docs/architecture/MULTI_AI_PROVIDER_DESIGN.md for full specification

// ==============================================================================
// Phase 6 TODO: Replicate (Stable Diffusion) Integration
// ==============================================================================

// TODO Phase 6: Uncomment and move to ai_providers/replicate.rs
/*
async fn generate_image_with_replicate(enhanced_prompt: String) -> Result<String, String> {
    const REPLICATE_API_URL: &str = "https://api.replicate.com/v1/predictions";
    const REPLICATE_MODEL: &str = "stability-ai/sdxl:39ed52f2a78e934b3ba6e2a89f5b1c712de7dfea535525255b1aa35c5565e08b";

    let replicate_api_key = crate::get_config().replicate_api_key
        .ok_or("Replicate API key not configured")?;

    let payload = json!({
        "version": REPLICATE_MODEL,
        "input": {
            "prompt": enhanced_prompt,
            "num_outputs": 1,
            "aspect_ratio": "1:1",
            "output_format": "png"
        }
    });

    let headers = vec![
        json_header(),
        auth_header(&replicate_api_key),
    ];

    let response_body = make_http_request(
        REPLICATE_API_URL.to_string(),
        HttpMethod::POST,
        headers,
        Some(payload.to_string().into_bytes()),
    ).await?;

    // Parse response and extract prediction ID
    let response_json: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&response_body))
        .map_err(|e| format!("JSON parse error: {}", e))?;

    let prediction_id = response_json["id"]
        .as_str()
        .ok_or("No prediction ID in response")?;

    // Poll for completion (simplified - should use timers in production)
    // For now, return placeholder
    Ok(format!("https://replicate.delivery/pbxt/{}/output.png", prediction_id))
}
*/

// ==============================================================================
// Phase 6 TODO: OpenAI GPT-4 Integration
// ==============================================================================

// TODO Phase 6: Create ai_providers/openai.rs with implementation like this:
/*
async fn enhance_prompt_with_openai(prompt: String, quality: QualityLevel) -> Result<String, String> {
    const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

    let model = match quality {
        QualityLevel::Draft => "gpt-3.5-turbo",
        QualityLevel::Standard => "gpt-4",
        QualityLevel::Premium => "gpt-4-turbo",
    };

    let api_key = crate::get_config().openai_api_key
        .ok_or("OpenAI API key not configured")?;

    let payload = json!({
        "model": model,
        "messages": [
            {
                "role": "system",
                "content": "You are an expert at writing prompts for AI image generation..."
            },
            {
                "role": "user",
                "content": prompt
            }
        ],
        "temperature": 0.7,
        "max_tokens": 500
    });

    // ... HTTP outcall logic similar to DeepSeek
}
*/

// ==============================================================================
// Phase 6 TODO: Anthropic Claude Integration
// ==============================================================================

// TODO Phase 6: Create ai_providers/anthropic.rs
/*
async fn enhance_prompt_with_claude(prompt: String) -> Result<String, String> {
    const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
    const ANTHROPIC_MODEL: &str = "claude-3-5-sonnet-20241022";

    let api_key = crate::get_config().anthropic_api_key
        .ok_or("Anthropic API key not configured")?;

    // Claude uses different API format (messages API)
    let payload = json!({
        "model": ANTHROPIC_MODEL,
        "max_tokens": 500,
        "messages": [
            {
                "role": "user",
                "content": format!("Enhance this prompt for AI image generation: {}", prompt)
            }
        ]
    });

    // Custom headers for Anthropic
    let headers = vec![
        json_header(),
        HttpHeader {
            name: "x-api-key".to_string(),
            value: api_key,
        },
        HttpHeader {
            name: "anthropic-version".to_string(),
            value: "2023-06-01".to_string(),
        },
    ];

    // ... HTTP outcall logic
}
*/

// ==============================================================================
// Phase 6 TODO: DALL-E 3 Integration
// ==============================================================================

// TODO Phase 6: Create ai_providers/dalle.rs
/*
async fn generate_image_with_dalle(enhanced_prompt: String, quality: QualityLevel) -> Result<String, String> {
    const DALLE_API_URL: &str = "https://api.openai.com/v1/images/generations";

    let size = match quality {
        QualityLevel::Draft => "1024x1024",
        QualityLevel::Standard => "1024x1792",
        QualityLevel::Premium => "1792x1024",
    };

    let api_key = crate::get_config().openai_api_key
        .ok_or("OpenAI API key not configured")?;

    let payload = json!({
        "model": "dall-e-3",
        "prompt": enhanced_prompt,
        "n": 1,
        "size": size,
        "quality": "hd",
        "response_format": "url"
    });

    // ... HTTP outcall to get image URL
    // Then upload to IPFS for permanent storage
}
*/

// ==============================================================================
// Phase 6 TODO: Auto Provider Selection
// ==============================================================================

// TODO Phase 6: Create ai_providers/auto_selector.rs
/*
pub async fn auto_select_provider(
    prompt: &str,
    quality: QualityLevel,
    user_tier: SubscriptionTier,
) -> Result<AIProvider, String> {
    // Classify content type from prompt
    let content_type = classify_prompt(prompt);

    match (content_type, quality, user_tier) {
        // Free tier: DeepSeek only
        (_, _, SubscriptionTier::Free) => Ok(AIProvider::DeepSeek),

        // Image generation
        (ContentType::Image, QualityLevel::Draft, _) => Ok(AIProvider::Replicate),
        (ContentType::Image, QualityLevel::Premium, _) => Ok(AIProvider::DALLE),

        // Text enhancement
        (ContentType::Text, QualityLevel::Draft, _) => Ok(AIProvider::DeepSeek),
        (ContentType::Text, QualityLevel::Standard, _) => Ok(AIProvider::OpenAI),
        (ContentType::Text, QualityLevel::Premium, _) => Ok(AIProvider::Anthropic),

        // Fallback
        _ => Ok(AIProvider::DeepSeek),
    }
}

fn classify_prompt(prompt: &str) -> ContentType {
    let prompt_lower = prompt.to_lowercase();

    if prompt_lower.contains("image") || prompt_lower.contains("picture")
        || prompt_lower.contains("photo") || prompt_lower.contains("draw") {
        return ContentType::Image;
    }

    if prompt_lower.contains("video") || prompt_lower.contains("animation") {
        return ContentType::Video;
    }

    ContentType::Text
}
*/
