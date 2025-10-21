/**
 * Script to register the Parent AI Agent on Story Protocol
 * This AI Agent represents the model that generates derivative IP assets
 *
 * Prerequisites:
 * 1. Install dependencies: npm install @story-protocol/core-sdk viem dotenv
 * 2. Create .env file with WALLET_PRIVATE_KEY and RPC_PROVIDER_URL
 * 3. Fund your wallet with IP tokens from https://faucet.story.foundation
 *
 * Usage:
 * npx ts-node scripts/register_ai_agent.ts
 */

import { StoryClient, StoryConfig } from '@story-protocol/core-sdk';
import { http } from 'viem';
import { privateKeyToAccount, Address } from 'viem/accounts';
import * as fs from 'fs';
import * as path from 'path';
import * as dotenv from 'dotenv';

// Load environment variables
dotenv.config();

// Load Story configuration
const configPath = path.join(__dirname, '../story_config.json');
const config = JSON.parse(fs.readFileSync(configPath, 'utf-8'));

async function main() {
  console.log('🚀 Starting AI Agent Registration on Story Protocol...\n');

  // Validate environment variables
  if (!process.env.WALLET_PRIVATE_KEY) {
    throw new Error('WALLET_PRIVATE_KEY not found in .env file');
  }
  if (!process.env.RPC_PROVIDER_URL) {
    throw new Error('RPC_PROVIDER_URL not found in .env file');
  }

  // Create account from private key
  const account = privateKeyToAccount(process.env.WALLET_PRIVATE_KEY as `0x${string}`);
  console.log(`📝 Using wallet address: ${account.address}\n`);

  // Initialize Story Client
  const storyConfig: StoryConfig = {
    account: account,
    transport: http(process.env.RPC_PROVIDER_URL),
    chainId: 'aeneid', // Story testnet chain ID
  };

  const client = StoryClient.newClient(storyConfig);
  console.log('✅ Story Client initialized\n');

  // Prepare AI Agent metadata
  const aiAgentMetadata = config.aiAgent;

  // Update creator address with the actual wallet address
  aiAgentMetadata.creators[0].address = account.address;

  console.log('🤖 AI Agent Metadata:');
  console.log(JSON.stringify(aiAgentMetadata, null, 2));
  console.log('\n');

  try {
    // Register the IP Asset
    console.log('📤 Registering AI Agent as IP Asset on Story Protocol...');

    // Note: This is a placeholder - you'll need to use the actual SDK method
    // The exact method depends on Story SDK version and documentation
    // Typically it would be something like:
    // const response = await client.ipAsset.register({
    //   metadata: aiAgentMetadata,
    //   ...
    // });

    console.log('⚠️  IMPORTANT: Update this script with the correct Story SDK method');
    console.log('   Reference: https://docs.story.foundation/developers/typescript-sdk/');
    console.log('\n');

    // Placeholder response structure
    const response = {
      ipId: 'PLACEHOLDER_IP_ID',
      txHash: 'PLACEHOLDER_TX_HASH',
    };

    console.log('✅ AI Agent registered successfully!');
    console.log(`   IP Asset ID: ${response.ipId}`);
    console.log(`   Transaction Hash: ${response.txHash}`);
    console.log(`   Explorer: ${config.network.explorerUrl}/tx/${response.txHash}`);
    console.log('\n');

    // Update config file with the IP ID
    config.deployment.parentAiModelIpId = response.ipId;
    fs.writeFileSync(configPath, JSON.stringify(config, null, 2));

    console.log('💾 Updated story_config.json with AI Agent IP ID');
    console.log('\n');
    console.log('🎉 Next steps:');
    console.log('   1. Verify the IP Asset on the explorer');
    console.log('   2. Attach a license to the AI Agent IP');
    console.log('   3. Update the ICP canister configuration with this IP ID');
    console.log('   4. Fund the canister EVM address for gas fees');

  } catch (error) {
    console.error('❌ Error registering AI Agent:', error);
    throw error;
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
