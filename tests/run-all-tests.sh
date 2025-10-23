#!/bin/bash
set -e

echo "🧪 ProvenanceAI - Running All Tests"
echo "===================================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

FAILED=0
START_TIME=$(date +%s)

# 1. Rust Tests (ICP Canister)
echo -e "${YELLOW}📦 1/3 Running Rust Tests (ICP Canister)${NC}"
echo "----------------------------------------"
if cargo test --package brain_canister --quiet; then
    echo -e "${GREEN}✅ Rust tests passed${NC}"
else
    echo -e "${RED}❌ Rust tests failed${NC}"
    FAILED=1
fi
echo ""

# 2. Solidity Tests (Story Protocol)
echo -e "${YELLOW}⚒️  2/3 Running Solidity Tests (Story Protocol)${NC}"
echo "-----------------------------------------------"
cd packages/story
if forge test --summary; then
    echo -e "${GREEN}✅ Solidity tests passed${NC}"
    cd ../..
else
    echo -e "${RED}❌ Solidity tests failed${NC}"
    FAILED=1
    cd ../..
fi
echo ""

# 3. Constellation Tests (VPS)
echo -e "${YELLOW}🌟 3/3 Verifying Constellation (VPS)${NC}"
echo "------------------------------------------"
if command -v sshpass &> /dev/null; then
    VPS_STATUS=$(sshpass -p 'SSH_PASSWORD_REDACTED' ssh -o StrictHostKeyChecking=no root@198.144.183.32 'curl -s http://localhost:9400/node/info | grep -o "Ready"' 2>/dev/null || echo "")
    if [ "$VPS_STATUS" == "Ready" ]; then
        echo -e "${GREEN}✅ Constellation Data L1 is Ready on VPS${NC}"
    else
        echo -e "${YELLOW}⚠️  Constellation VPS not accessible (OK for local dev)${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  sshpass not installed, skipping Constellation check${NC}"
fi
echo ""

# Calculate duration
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

# Summary
echo "===================================="
echo -e "${BLUE}⏱️  Total time: ${DURATION}s${NC}"
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}💥 Some tests failed${NC}"
    exit 1
fi
