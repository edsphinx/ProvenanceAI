#!/bin/bash
# Build all components of ProvenanceAI

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🔨 Building All Components"
echo "=========================="
echo ""

# 1. ICP Canister
echo -e "${YELLOW}📦 1/2 Building ICP Canister...${NC}"
dfx build brain_canister
echo -e "${GREEN}✅ ICP Canister built${NC}"
echo ""

# 2. Story Protocol Contracts
echo -e "${YELLOW}⚒️  2/2 Building Story Protocol Contracts...${NC}"
cd packages/story
forge build
cd ../..
echo -e "${GREEN}✅ Story Protocol Contracts built${NC}"
echo ""

echo -e "${GREEN}✅ All components built successfully!${NC}"
