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
if forge test --summary; then
    echo -e "${GREEN}✅ Solidity tests passed${NC}"
else
    echo -e "${RED}❌ Solidity tests failed${NC}"
    FAILED=1
fi
echo ""

# 3. Scala Tests (Constellation)
echo -e "${YELLOW}🌟 3/3 Running Scala Tests (Constellation)${NC}"
echo "------------------------------------------"
if [ -d "constellation-metagraph" ] && [ -f "constellation-metagraph/build.sbt" ]; then
    cd constellation-metagraph
    if sbt -batch test; then
        echo -e "${GREEN}✅ Scala tests passed${NC}"
    else
        echo -e "${RED}❌ Scala tests failed${NC}"
        FAILED=1
    fi
    cd ..
else
    echo -e "${YELLOW}⚠️  Constellation metagraph not yet created, skipping Scala tests${NC}"
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
