#!/bin/bash
# Script para deployment a mainnet ICP con password automatizado

set -e

# Colores
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🚀 Deployment a Mainnet ICP - Automatizado"
echo "=========================================="
echo ""

# Verificar que existe DFX_PASSWORD
if [ -z "$DFX_PASSWORD" ]; then
    echo -e "${RED}❌ Error: DFX_PASSWORD no está configurado${NC}"
    echo "Por favor ejecuta: export DFX_PASSWORD='tu_password'"
    exit 1
fi

# Usar identity edsphinx con password desde variable
echo -e "${YELLOW}🔐 Configurando identity edsphinx...${NC}"
echo "$DFX_PASSWORD" | dfx identity use edsphinx --password-file /dev/stdin

# Verificar cycles disponibles
echo -e "${YELLOW}💰 Verificando cycles disponibles...${NC}"
CYCLES=$(dfx wallet --network ic balance 2>&1 || echo "0")
echo "Cycles disponibles: $CYCLES"

# Build canister
echo -e "${YELLOW}🔨 Building canister...${NC}"
dfx build brain_canister

# Deploy a mainnet
echo -e "${YELLOW}🚀 Desplegando a mainnet...${NC}"
export DFX_WARNING=-mainnet_plaintext_identity

dfx deploy brain_canister --network ic --argument '(record {
  deepseek_api_key = "DEEPSEEK_API_KEY_REDACTED";
  constellation_metagraph_url = "http://198.144.183.32:9400";
  replicate_api_key = opt "";
})'

# Registrar NFT contract
echo -e "${YELLOW}📝 Registrando NFT contract...${NC}"
dfx canister call brain_canister set_nft_contract_address \
  '("0x6853943248910243cDCE66C21C12398ebbC1642D")' \
  --network ic

# Obtener canister ID
CANISTER_ID=$(dfx canister id brain_canister --network ic)

echo ""
echo -e "${GREEN}✅ Deployment completado!${NC}"
echo "Canister ID: $CANISTER_ID"
echo "URL: https://$CANISTER_ID.icp0.io"
echo ""
echo "Para probar E2E:"
echo "dfx canister call brain_canister generate_and_register_ip '(record {
  prompt = \"Test from mainnet\";
  metadata = record { title = \"Test\"; description = \"\"; tags = vec {} };
})' --network ic"

