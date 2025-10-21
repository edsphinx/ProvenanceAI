#!/bin/bash

# ==============================================================================
# Provenance AI - ICP Canister Deployment Script
# ==============================================================================
# This script deploys the brain_canister with secure configuration from .env
# ==============================================================================

set -e  # Exit on error

echo "🚀 Deploying Provenance AI to Internet Computer..."
echo ""

# ==============================================================================
# Step 1: Validate Environment
# ==============================================================================

if [ ! -f .env ]; then
    echo "❌ Error: .env file not found"
    echo ""
    echo "📝 To fix this:"
    echo "   1. Copy .env.example to .env"
    echo "   2. Fill in your API keys and configuration"
    echo ""
    exit 1
fi

echo "✅ Found .env file"

# Load environment variables
source .env

# ==============================================================================
# Step 2: Validate Required Variables
# ==============================================================================

echo "🔍 Validating configuration..."

if [ -z "$DEEPSEEK_API_KEY" ]; then
    echo "❌ Error: DEEPSEEK_API_KEY not set in .env"
    exit 1
fi

if [ -z "$CONSTELLATION_METAGRAPH_URL" ]; then
    echo "⚠️  Warning: CONSTELLATION_METAGRAPH_URL not set"
    echo "   Using placeholder: https://placeholder.metagraph.example.com"
    CONSTELLATION_METAGRAPH_URL="https://placeholder.metagraph.example.com"
fi

echo "✅ DeepSeek API Key: ${DEEPSEEK_API_KEY:0:10}... (truncated)"
echo "✅ Constellation URL: $CONSTELLATION_METAGRAPH_URL"
echo ""

# ==============================================================================
# Step 3: Build Candid Argument
# ==============================================================================

CONFIG_ARG="(record {
  deepseek_api_key = \"${DEEPSEEK_API_KEY}\";
  replicate_api_key = null;
  constellation_metagraph_url = \"${CONSTELLATION_METAGRAPH_URL}\";
})"

# ==============================================================================
# Step 4: Determine Network
# ==============================================================================

NETWORK=${DFX_NETWORK:-local}

if [ "$NETWORK" == "ic" ]; then
    echo "🌐 Deploying to MAINNET (ic)"
    echo "⚠️  WARNING: This will deploy to production!"
    read -p "   Are you sure? (yes/no): " confirm
    if [ "$confirm" != "yes" ]; then
        echo "❌ Deployment cancelled"
        exit 1
    fi
else
    echo "🏠 Deploying to LOCAL network"
fi

echo ""

# ==============================================================================
# Step 5: Create Canisters
# ==============================================================================

echo "📦 Creating canisters..."
dfx canister create --all --network $NETWORK

echo ""

# ==============================================================================
# Step 6: Build Canisters
# ==============================================================================

echo "🔨 Building brain_canister (Rust)..."
dfx build brain_canister --network $NETWORK

echo ""

# ==============================================================================
# Step 7: Deploy with Configuration
# ==============================================================================

echo "🚢 Deploying brain_canister with secure configuration..."
dfx deploy brain_canister --network $NETWORK --argument "$CONFIG_ARG"

echo ""

# ==============================================================================
# Step 8: Verify Deployment
# ==============================================================================

echo "✅ Deployment complete!"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  📊 Canister Information"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

CANISTER_ID=$(dfx canister id brain_canister --network $NETWORK)
echo "  🆔 Canister ID: $CANISTER_ID"

if [ "$NETWORK" == "ic" ]; then
    echo "  🌐 URL: https://$CANISTER_ID.icp0.io"
else
    echo "  🏠 Local URL: http://127.0.0.1:4943/?canisterId=$CANISTER_ID"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  🧪 Test Commands"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "  # Verify configuration"
echo "  dfx canister call brain_canister is_configured --network $NETWORK"
echo ""
echo "  # Test AI generation (simple)"
echo "  dfx canister call brain_canister generate_and_register_ip \\"
echo "    '(record {"
echo "      prompt = \"A futuristic cityscape at sunset\";"
echo "      metadata = record { title = \"Test\"; description = \"\"; tags = vec {} };"
echo "    })' --network $NETWORK"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
