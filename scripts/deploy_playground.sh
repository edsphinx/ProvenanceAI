#!/bin/bash
# Deploy a ICP Playground (testnet gratuito con IPv6)

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🎮 Deployment a ICP Playground"
echo "=============================="
echo ""
echo -e "${YELLOW}ℹ️  Playground tiene IPv6 y es GRATIS${NC}"
echo -e "${YELLOW}⚠️  Canisters expiran después de 20 minutos${NC}"
echo ""

# Deploy con --playground flag (builds automatically)
echo -e "${YELLOW}🚀 Desplegando a Playground...${NC}"
dfx deploy brain_canister --playground --argument '(record {
  deepseek_api_key = "DEEPSEEK_API_KEY_REDACTED";
  constellation_metagraph_url = "http://198.144.183.32:9400";
  replicate_api_key = opt "";
})'

# Setear NFT contract
echo -e "${YELLOW}📝 Registrando NFT contract...${NC}"
dfx canister call brain_canister set_nft_contract_address \
  '("0x6853943248910243cDCE66C21C12398ebbC1642D")' \
  --playground

# Obtener canister ID
CANISTER_ID=$(dfx canister id brain_canister --playground)

echo ""
echo -e "${GREEN}✅ Deployment completado en Playground!${NC}"
echo "Canister ID: $CANISTER_ID"
echo ""
echo -e "${YELLOW}⏰ IMPORTANTE: Canister expira en 20 minutos${NC}"
echo ""
echo "🧪 Ejecutar E2E Test:"
echo "dfx canister call brain_canister generate_and_register_ip '(record {
  prompt = \"Mountain landscape at sunset\";
  metadata = record { 
    title = \"E2E Playground Test\"; 
    description = \"Full integration test\"; 
    tags = vec {\"test\"; \"playground\"} 
  };
})' --playground"

