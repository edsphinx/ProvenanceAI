#!/bin/bash

# ==============================================================================
# Provenance AI - Fully Automated Deployment Script
# ==============================================================================
# Zero-interaction deployment similar to EVM/Aptos deployments
# Configuration loaded from .env, automatic identity management
# ==============================================================================

set -e  # Exit on error

# Add dfx to PATH if not already there
if ! command -v dfx &> /dev/null; then
    export PATH="$HOME/.local/share/dfx/bin:$PATH"
fi

# ==============================================================================
# Configuration
# ==============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ==============================================================================
# Helper Functions
# ==============================================================================

log_info() {
    echo -e "${BLUE}ℹ${NC}  $1"
}

log_success() {
    echo -e "${GREEN}✅${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}⚠${NC}  $1"
}

log_error() {
    echo -e "${RED}❌${NC} $1"
}

# ==============================================================================
# Step 1: Environment Validation
# ==============================================================================

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  🚀 Provenance AI - Automated Deployment"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

log_info "Validating environment..."

# Check for .env file
if [ ! -f .env ]; then
    log_error ".env file not found"
    log_info "Creating .env from .env.example..."

    if [ -f .env.example ]; then
        cp .env.example .env
        log_warning "Please edit .env with your API keys and re-run this script"
        exit 1
    else
        log_error ".env.example not found. Cannot proceed."
        exit 1
    fi
fi

log_success "Found .env file"

# Load environment variables
source .env

# Validate required variables
if [ -z "$DEEPSEEK_API_KEY" ]; then
    log_error "DEEPSEEK_API_KEY not set in .env"
    log_info "Get your API key from: https://platform.deepseek.com/api_keys"
    exit 1
fi

log_success "DeepSeek API Key: ${DEEPSEEK_API_KEY:0:10}..."

# Set defaults for optional variables
if [ -z "$CONSTELLATION_METAGRAPH_URL" ]; then
    CONSTELLATION_METAGRAPH_URL="https://l0-lb-testnet.constellationnetwork.io"
    log_warning "Using default Constellation URL: $CONSTELLATION_METAGRAPH_URL"
fi

if [ -z "$REPLICATE_API_KEY" ]; then
    log_info "No Replicate API key (optional for Phase 1)"
fi

# ==============================================================================
# Step 2: Network Selection
# ==============================================================================

echo ""
log_info "Determining deployment network..."

NETWORK=${DFX_NETWORK:-local}

if [ "$NETWORK" == "ic" ]; then
    log_warning "Deploying to MAINNET (ic)"
    log_warning "This will consume real cycles!"

    # Auto-confirm if DEPLOY_TO_MAINNET=true in .env
    if [ "$DEPLOY_TO_MAINNET" != "true" ]; then
        log_error "To deploy to mainnet, set DEPLOY_TO_MAINNET=true in .env"
        exit 1
    fi

    log_success "Mainnet deployment confirmed via .env"
else
    log_success "Deploying to LOCAL network"
fi

# ==============================================================================
# Step 3: Identity Management (Smart)
# ==============================================================================

echo ""
log_info "Managing dfx identity..."

# Use default identity or create deployment identity
IDENTITY_NAME=${DFX_IDENTITY:-default}

# Check if identity is encrypted (password-protected)
IDENTITY_PATH="$HOME/.config/dfx/identity/$IDENTITY_NAME/identity.pem.encrypted"
if [ -f "$IDENTITY_PATH" ]; then
    if [ "$NETWORK" == "ic" ]; then
        # Mainnet: Allow encrypted identity (more secure)
        log_info "Using password-protected identity '$IDENTITY_NAME' for mainnet"
        log_warning "⚠️  IMPORTANT: This script may have issues with encrypted identities"
        log_info "If deployment fails, deploy manually:"
        log_info "  dfx identity use $IDENTITY_NAME"
        log_info "  dfx deploy brain_canister --network ic --argument '(...)'"
    else
        # Local: Switch to 'default' (no password hassle)
        log_warning "Identity '$IDENTITY_NAME' is password-protected"
        log_info "For local development, switching to 'default' (no password needed)"
        log_info "Tip: 'default' is safe for local, 'edsphinx' for mainnet only"
        IDENTITY_NAME="default"
    fi
fi

# Create non-default identity if needed
if [ "$IDENTITY_NAME" != "default" ]; then
    # Check if identity exists
    if ! dfx identity list 2>/dev/null | grep -q "^$IDENTITY_NAME"; then
        log_info "Creating new identity: $IDENTITY_NAME"
        dfx identity new "$IDENTITY_NAME" --storage-mode=plaintext || true
    fi
fi

# Always switch to the determined identity (prevents password issues)
log_info "Switching to identity: $IDENTITY_NAME"

# Kill any existing identity session to avoid password prompts
export DFX_WARNING="-"  # Suppress warnings
dfx identity use "$IDENTITY_NAME" || {
    # If switch fails (password issue), force it by restarting dfx
    log_warning "Failed to switch identity cleanly, forcing switch..."
    pkill -f "dfx" 2>/dev/null || true
    sleep 1
    dfx identity use "$IDENTITY_NAME"
}
unset DFX_WARNING

PRINCIPAL=$(dfx identity get-principal)
log_success "Using identity: $IDENTITY_NAME (Principal: ${PRINCIPAL:0:20}...)"

# ==============================================================================
# Step 4: Start Local Replica (if needed)
# ==============================================================================

echo ""
log_info "Checking ICP replica status..."

if [ "$NETWORK" == "local" ]; then
    # Check if replica is running
    if ! dfx ping &>/dev/null; then
        log_info "Starting local ICP replica..."
        dfx start --clean --background

        # Wait for replica to be ready
        log_info "Waiting for replica to be ready..."
        sleep 5

        # Verify replica is running
        if dfx ping &>/dev/null; then
            log_success "Local replica started successfully"
        else
            log_error "Failed to start local replica"
            exit 1
        fi
    else
        log_success "Local replica already running"
    fi
fi

# ==============================================================================
# Step 5: Create Canisters
# ==============================================================================

echo ""
log_info "Creating canisters..."

dfx canister create --all --network $NETWORK

log_success "Canisters created"

# ==============================================================================
# Step 6: Build Canisters
# ==============================================================================

echo ""
log_info "Building brain_canister (Rust)..."

dfx build brain_canister --network $NETWORK

log_success "Build complete"

# ==============================================================================
# Step 7: Prepare Configuration Argument
# ==============================================================================

echo ""
log_info "Preparing deployment configuration..."

# Build Candid argument from environment
REPLICATE_KEY_ARG="null"
if [ -n "$REPLICATE_API_KEY" ]; then
    REPLICATE_KEY_ARG="opt \"$REPLICATE_API_KEY\""
fi

CONFIG_ARG="(record {
  deepseek_api_key = \"${DEEPSEEK_API_KEY}\";
  replicate_api_key = ${REPLICATE_KEY_ARG};
  constellation_metagraph_url = \"${CONSTELLATION_METAGRAPH_URL}\";
})"

log_success "Configuration prepared"

# ==============================================================================
# Step 8: Deploy Canisters
# ==============================================================================

echo ""
log_info "Deploying brain_canister with configuration..."

dfx deploy brain_canister --network $NETWORK --argument "$CONFIG_ARG"

log_success "Deployment complete"

# ==============================================================================
# Step 9: Post-Deployment Verification
# ==============================================================================

echo ""
log_info "Verifying deployment..."

CANISTER_ID=$(dfx canister id brain_canister --network $NETWORK)

# Test configuration
if dfx canister call brain_canister is_configured --network $NETWORK | grep -q "true"; then
    log_success "Canister configuration verified"
else
    log_error "Canister configuration check failed"
    exit 1
fi

# Get canister EVM address (for Phase 2)
log_info "Retrieving canister EVM address..."
EVM_ADDRESS=$(dfx canister call brain_canister get_canister_evm_address --network $NETWORK | grep -o '0x[a-fA-F0-9]*' || echo "ERROR")

if [ "$EVM_ADDRESS" != "ERROR" ]; then
    log_success "Canister EVM address: $EVM_ADDRESS"

    # Save to .env for Phase 2
    if ! grep -q "CANISTER_EVM_ADDRESS=" .env; then
        echo "" >> .env
        echo "# Auto-generated by deploy_all.sh" >> .env
        echo "CANISTER_EVM_ADDRESS=$EVM_ADDRESS" >> .env
        log_success "Saved EVM address to .env"
    fi
else
    log_warning "Could not retrieve EVM address (will retry later)"
fi

# ==============================================================================
# Step 10: Cycle Management (Mainnet only)
# ==============================================================================

if [ "$NETWORK" == "ic" ]; then
    echo ""
    log_info "Checking canister cycle balance..."

    CYCLES=$(dfx canister status brain_canister --network ic 2>/dev/null | grep "Balance:" | awk '{print $2}' || echo "0")

    log_info "Current balance: $CYCLES cycles"

    if [ "$CYCLES" -lt 1000000000000 ]; then
        log_warning "Low cycle balance! Top up at: https://faucet.dfinity.org"
    fi
fi

# ==============================================================================
# Step 11: Output Deployment Information
# ==============================================================================

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  ✅ Deployment Successful!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "  Network:      $NETWORK"
echo "  Canister ID:  $CANISTER_ID"
echo "  EVM Address:  $EVM_ADDRESS"
echo "  Principal:    $PRINCIPAL"
echo ""

if [ "$NETWORK" == "ic" ]; then
    echo "  🌐 Public URL: https://$CANISTER_ID.icp0.io"
else
    echo "  🏠 Local URL:  http://127.0.0.1:4943/?canisterId=$CANISTER_ID"
    CANDID_UI=$(dfx canister id __Candid_UI --network local 2>/dev/null || echo "")
    if [ -n "$CANDID_UI" ]; then
        echo "  🎨 Candid UI:  http://127.0.0.1:4943/?canisterId=$CANDID_UI&id=$CANISTER_ID"
    fi
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  🧪 Quick Test Commands"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "  # Verify configuration"
echo "  dfx canister call brain_canister is_configured --network $NETWORK"
echo ""
echo "  # Generate AI content"
echo "  dfx canister call brain_canister generate_and_register_ip \\"
echo "    '(record {"
echo "      prompt = \"A serene mountain landscape at sunset\";"
echo "      metadata = record { title = \"Test\"; description = \"\"; tags = vec {} };"
echo "    })' --network $NETWORK"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  📋 Next Steps for Phase 2"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "  1. Fund your canister's EVM address with Story testnet tokens:"
echo "     Address: $EVM_ADDRESS"
echo "     Faucet:  https://faucet.story.foundation"
echo ""
echo "  2. Register parent AI model IP on Story Protocol:"
echo "     npx ts-node scripts/register_ai_agent.ts"
echo ""
echo "  3. Update Story config in src/brain_canister/src/config.rs"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

log_success "Deployment completed successfully!"
echo ""

# ==============================================================================
# Save deployment info to file
# ==============================================================================

DEPLOYMENT_INFO="deployments/${NETWORK}_deployment_$(date +%Y%m%d_%H%M%S).txt"
mkdir -p deployments

cat > "$DEPLOYMENT_INFO" << EOF
Provenance AI - Deployment Information
=====================================

Timestamp:    $(date)
Network:      $NETWORK
Canister ID:  $CANISTER_ID
EVM Address:  $EVM_ADDRESS
Principal:    $PRINCIPAL

Phase:        1 (Brain Canister with AI Generation)
Status:       Deployed Successfully

Configuration:
- DeepSeek API:     Configured
- Constellation:    $CONSTELLATION_METAGRAPH_URL
- Replicate:        ${REPLICATE_API_KEY:+Configured}

Next Steps:
1. Fund EVM address: $EVM_ADDRESS
2. Register AI model on Story Protocol
3. Proceed to Phase 2 implementation
EOF

log_success "Deployment info saved to: $DEPLOYMENT_INFO"
