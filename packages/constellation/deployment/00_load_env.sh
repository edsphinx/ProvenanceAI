#!/bin/bash
# ProvenanceAI - Environment Variable Loader
# Purpose: Load .env variables and substitute them in configuration files

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=========================================="
echo "Loading Environment Variables"
echo "=========================================="
echo ""

# Find project root (where .env should be)
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../" && pwd)"
ENV_FILE="$PROJECT_ROOT/.env"

echo "Project root: $PROJECT_ROOT"
echo "Looking for .env at: $ENV_FILE"
echo ""

# Check if .env exists
if [ ! -f "$ENV_FILE" ]; then
    echo -e "${RED}ERROR: .env file not found at $ENV_FILE${NC}"
    echo ""
    echo "Please create .env from .env.example:"
    echo "  cp .env.example .env"
    echo "  # Then edit .env with your actual values"
    echo ""
    exit 1
fi

# Load environment variables
echo "Loading variables from .env..."
set -a  # automatically export all variables
source "$ENV_FILE"
set +a
echo -e "${GREEN}✓ Environment variables loaded${NC}"
echo ""

# Check required Constellation variables
REQUIRED_VARS=(
    "CONSTELLATION_GENESIS_PASSWORD"
    "CONSTELLATION_VALIDATOR_PASSWORD"
    "CONSTELLATION_TOKEN_KEY_PASSWORD"
)

MISSING_VARS=()
for var in "${REQUIRED_VARS[@]}"; do
    if [ -z "${!var}" ]; then
        MISSING_VARS+=("$var")
    fi
done

if [ ${#MISSING_VARS[@]} -ne 0 ]; then
    echo -e "${RED}ERROR: Missing required variables in .env:${NC}"
    for var in "${MISSING_VARS[@]}"; do
        echo "  - $var"
    done
    echo ""
    exit 1
fi

echo -e "${GREEN}✓ All required Constellation variables present${NC}"
echo ""

# Function to substitute environment variables in a file
substitute_env_vars() {
    local file="$1"
    local temp_file="${file}.tmp"

    if [ ! -f "$file" ]; then
        echo -e "${YELLOW}Warning: File not found: $file${NC}"
        return 1
    fi

    echo "Substituting variables in: $file"

    # Use envsubst to replace ${VAR} with actual values
    # If envsubst is not available, use sed
    if command -v envsubst &> /dev/null; then
        envsubst < "$file" > "$temp_file"
    else
        # Fallback: use sed
        cp "$file" "$temp_file"
        sed -i "s|\${CONSTELLATION_GENESIS_PASSWORD}|${CONSTELLATION_GENESIS_PASSWORD}|g" "$temp_file"
        sed -i "s|\${CONSTELLATION_VALIDATOR_PASSWORD}|${CONSTELLATION_VALIDATOR_PASSWORD}|g" "$temp_file"
        sed -i "s|\${CONSTELLATION_TOKEN_KEY_PASSWORD}|${CONSTELLATION_TOKEN_KEY_PASSWORD}|g" "$temp_file"
    fi

    mv "$temp_file" "$file"
    echo -e "${GREEN}✓ Variables substituted in $file${NC}"
}

# Export this function so other scripts can use it
export -f substitute_env_vars

echo "=========================================="
echo -e "${GREEN}Environment ready for deployment${NC}"
echo "=========================================="
echo ""
