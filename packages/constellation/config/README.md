# Constellation Network Configuration

This directory contains configuration files for the ProvenanceAI metagraph deployment.

## Files

- **`euclid.json`** - Main metagraph configuration (network, layers, nodes)
- **`genesis-global-l0.csv`** - Genesis configuration for Global L0 layer
- **`genesis-metagraph-l0.csv`** - Genesis configuration for Metagraph L0 layer

## Environment Variables

The `euclid.json` file uses environment variables for sensitive data (P12 passwords). Before deploying, ensure you have these variables set in your **root `.env` file**:

```bash
CONSTELLATION_GENESIS_PASSWORD=your-genesis-password
CONSTELLATION_VALIDATOR_PASSWORD=your-validator-password
CONSTELLATION_TOKEN_KEY_PASSWORD=your-token-password
```

### How to Use

1. Copy `.env.example` to `.env` in the project root:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` and fill in your actual P12 passwords

3. The deployment scripts will automatically substitute `${VARIABLE_NAME}` placeholders with actual values from `.env`

## Security

⚠️ **NEVER commit `.env` or files with actual passwords to git!**

- P12 password placeholders (`${...}`) are safe to commit
- Actual passwords must only be in `.env` (which is gitignored)
- P12 files themselves are also gitignored

## Deployment

See `../deployment/README.md` for full deployment instructions.
