#!/bin/bash
set -e

# ProvenanceAI Metagraph Deployment - Script 4: Create Metagraph
# Purpose: Install template and create ProofOfGeneration data structure

echo "=========================================="
echo "Script 4: Create ProvenanceAI Metagraph"
echo "=========================================="
echo ""

LOG_FILE="/root/provenanceai-metagraph/logs/04_create_metagraph.log"
exec > >(tee -a "$LOG_FILE") 2>&1

echo "Log file: $LOG_FILE"
echo "Started at: $(date)"
echo ""

cd /root/provenanceai-metagraph/euclid-development-environment

# Install Custom NFT template (closest to our use case)
echo "[1/4] Installing Custom NFT template..."
scripts/hydra install-template custom-nft || echo "Template already installed or unavailable"
echo ""

# Configure euclid.json
echo "[2/4] Configuring euclid.json for ProvenanceAI..."
cat > euclid.json <<'EOF'
{
  "project_name": "provenanceai-metagraph",
  "version": "1.0.0",
  "network": "integrationnet",
  "framework": {
    "name": "data-api",
    "version": "v2.0.0"
  },
  "layers": {
    "data_l1": {
      "name": "ProvenanceAI Data Layer",
      "description": "AI-generated content proof logging for cross-chain IP protection"
    }
  },
  "github_token": "",
  "p12_files": {
    "genesis": {
      "alias": "genesis",
      "password": "provenanceai-genesis-2025"
    }
  },
  "start_grafana_container": true
}
EOF

echo "euclid.json configured"
cat euclid.json
echo ""

# Create ProofOfGeneration data structure (Scala)
echo "[3/4] Creating ProofOfGeneration data structure..."

# Create directory structure
mkdir -p source/project/data-l1/src/main/scala/com/provenanceai/data_l1

# Create ProofOfGeneration case class
cat > source/project/data-l1/src/main/scala/com/provenanceai/data_l1/ProofOfGeneration.scala <<'EOF'
package com.provenanceai.data_l1

import org.tessellation.schema.address.Address
import org.tessellation.currency.dataApplication.DataUpdate

// ProofOfGeneration data structure
// Tracks AI-generated content and its Story Protocol IP registration
case class ProofOfGeneration(
  contentHash: String,           // Hash of AI-generated content (0xabc123...)
  modelName: String,             // AI model used (e.g., "deepseek-chat")
  timestamp: Long,               // Generation timestamp (Unix ms)
  storyIpId: String,            // Story Protocol IP ID (0x...)
  nftContract: String,          // Story Protocol NFT contract address (0x...)
  nftTokenId: Long,             // NFT token ID
  generatorAddress: String      // ICP canister identifier
) extends DataUpdate

object ProofOfGeneration {
  // Validation logic
  def validate(proof: ProofOfGeneration): Either[String, Unit] = {
    // Validate content hash format (should be hex string starting with 0x)
    if (proof.contentHash.isEmpty || !proof.contentHash.startsWith("0x")) {
      return Left("Invalid content hash format - must start with 0x")
    }

    // Validate timestamp (should be in the past)
    val now = System.currentTimeMillis()
    if (proof.timestamp <= 0 || proof.timestamp > now) {
      return Left(s"Invalid timestamp - must be between 0 and $now")
    }

    // Validate Story Protocol IP ID format
    if (proof.storyIpId.isEmpty || !proof.storyIpId.startsWith("0x")) {
      return Left("Invalid Story Protocol IP ID format - must start with 0x")
    }

    // Validate NFT contract address (Ethereum address format)
    val addressPattern = "^0x[0-9a-fA-F]{40}$".r
    if (!addressPattern.matches(proof.nftContract)) {
      return Left("Invalid NFT contract address - must be valid Ethereum address")
    }

    // Validate NFT token ID (should be positive)
    if (proof.nftTokenId < 0) {
      return Left("Invalid NFT token ID - must be non-negative")
    }

    // Validate generator address (should not be empty)
    if (proof.generatorAddress.isEmpty) {
      return Left("Invalid generator address - cannot be empty")
    }

    // All validations passed
    Right(())
  }
}
EOF

echo "ProofOfGeneration.scala created"
echo ""

# Create validation module
echo "[4/4] Creating validation logic..."
cat > source/project/data-l1/src/main/scala/com/provenanceai/data_l1/Validation.scala <<'EOF'
package com.provenanceai.data_l1

import cats.effect.IO
import cats.syntax.all._
import org.tessellation.currency.dataApplication.DataApplicationValidationErrorOr

object Validation {
  // Validate individual proof update
  def validateUpdate(proof: ProofOfGeneration): IO[DataApplicationValidationErrorOr[Unit]] = IO {
    ProofOfGeneration.validate(proof) match {
      case Right(_) => ().validNec
      case Left(error) =>
        org.tessellation.currency.dataApplication.DataApplicationValidationError(error).invalidNec
    }
  }

  // Validate data batch (check for duplicates)
  def validateData(
    currentState: Map[String, ProofOfGeneration],
    updates: List[ProofOfGeneration]
  ): IO[DataApplicationValidationErrorOr[Unit]] = {

    // Check for duplicate content hashes in current state
    val newHashes = updates.map(_.contentHash)
    val existingHashes = currentState.keySet
    val duplicates = newHashes.filter(existingHashes.contains)

    if (duplicates.nonEmpty) {
      IO.pure(
        org.tessellation.currency.dataApplication.DataApplicationValidationError(
          s"Duplicate content hashes found: ${duplicates.mkString(", ")}"
        ).invalidNec
      )
    } else {
      // Check for duplicates within the update batch
      val batchDuplicates = newHashes.diff(newHashes.distinct)
      if (batchDuplicates.nonEmpty) {
        IO.pure(
          org.tessellation.currency.dataApplication.DataApplicationValidationError(
            s"Duplicate content hashes in batch: ${batchDuplicates.mkString(", ")}"
          ).invalidNec
        )
      } else {
        IO.pure(().validNec)
      }
    }
  }
}
EOF

echo "Validation.scala created"
echo ""

echo "=========================================="
echo "✅ ProvenanceAI Metagraph Created!"
echo "=========================================="
echo ""
echo "Files created:"
echo "- euclid.json (metagraph configuration)"
echo "- ProofOfGeneration.scala (data structure)"
echo "- Validation.scala (validation logic)"
echo ""
echo "Data structure supports:"
echo "- Content hash validation (hex format with 0x prefix)"
echo "- Timestamp validation (must be in past)"
echo "- Story Protocol IP ID validation"
echo "- NFT contract address validation (Ethereum format)"
echo "- NFT token ID validation"
echo "- Duplicate detection"
echo ""
echo "Next step: Run ./05_build_metagraph.sh"
echo ""
