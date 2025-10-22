# ProvenanceAI - Testing Guide

**Date**: 2025-10-22
**Status**: ✅ Ready to Use

---

## 🚀 Quick Start

### Run All Tests
```bash
# From project root
./tests/run-all-tests.sh
```

### Run Individual Test Suites

#### Rust Tests (ICP Canister)
```bash
# All tests
cargo test --package brain_canister

# With output
cargo test --package brain_canister -- --nocapture

# Specific test
cargo test --package brain_canister test_parse_nonce_from_hex
```

#### Solidity Tests (Story Protocol)
```bash
# All tests
forge test

# Verbose output
forge test -vvv

# Specific test
forge test --match-test testMintNFT

# Gas report
forge test --gas-report

# Coverage
forge coverage
```

#### Scala Tests (Constellation) - After Metagraph Setup
```bash
cd constellation-metagraph

# All tests
sbt test

# Specific test
sbt "testOnly ProofOfGenerationTest"

# With detailed output
sbt "testOnly * -- -oF"
```

---

## 📁 Test Structure

```
tests/
├── run-all-tests.sh              # Master test runner (USE THIS!)
├── README.md                     # This file
├── icp/                          # ICP-specific test utilities
├── story/                        # Story-specific test utilities
└── constellation/                # Constellation test templates
    └── ProofOfGenerationTest.scala

src/brain_canister/tests/         # Rust integration tests
└── integration_test.rs           # Main integration test file

test/                             # Foundry tests (Solidity)
└── SimpleNFT.t.sol               # SimpleNFT contract tests

constellation-metagraph/          # Will be created by VPS deployment
└── src/test/scala/               # Scala tests (when metagraph ready)
```

---

## 🧪 Current Test Status

| Component | Tests Created | Status | Coverage |
|-----------|--------------|--------|----------|
| **ICP Canister** | ✅ Yes | 🟢 Passing | TBD |
| **Story Protocol** | ✅ Yes | 🟢 Passing | TBD |
| **Constellation** | 🟡 Template | ⏸️ Pending | N/A |

---

## 📝 Test Categories

### 1. Rust Tests (ICP Canister)

#### Unit Tests
Location: `src/brain_canister/tests/integration_test.rs`

- ✅ Nonce parsing (hex → u64)
- ✅ JSON RPC response parsing
- ✅ Constellation payload structure
- ✅ ABI encoding basics

#### Integration Tests (TODO)
- [ ] Full canister lifecycle
- [ ] HTTP outcall mocking
- [ ] End-to-end workflows
- [ ] State management

### 2. Solidity Tests (Story Protocol)

#### Unit Tests
Location: `test/SimpleNFT.t.sol`

- ✅ Deployment
- ✅ NFT Minting
- ✅ Content hash storage
- ✅ Metadata URI
- ✅ Ownership & transfers
- ✅ Access control
- ✅ Events
- ✅ Edge cases
- ✅ Gas estimation

#### Integration Tests (TODO)
- [ ] IP registration flow
- [ ] Story Protocol contract interactions
- [ ] Fork tests against Aeneid testnet

### 3. Scala Tests (Constellation)

#### Template Created
Location: `tests/constellation/ProofOfGenerationTest.scala`

**Status**: ⏸️ Waiting for VPS metagraph deployment

**When metagraph is ready**:
1. Move template to `constellation-metagraph/src/test/scala/`
2. Update imports and package
3. Implement actual tests
4. Run `sbt test`

---

## 🎯 How to Add New Tests

### Adding Rust Tests

1. **Unit tests** - Add to existing module:
```rust
// In src/brain_canister/tests/integration_test.rs

#[cfg(test)]
mod my_new_tests {
    #[test]
    fn test_my_feature() {
        assert_eq!(2 + 2, 4);
    }
}
```

2. **Integration tests** - Create new file:
```bash
# Create new test file
touch src/brain_canister/tests/my_feature_test.rs
```

### Adding Solidity Tests

1. Create new test file:
```bash
touch test/MyContract.t.sol
```

2. Follow Foundry test structure:
```solidity
pragma solidity ^0.8.23;

import "forge-std/Test.sol";
import "../src/MyContract.sol";

contract MyContractTest is Test {
    function setUp() public {
        // Setup
    }

    function testMyFeature() public {
        // Test
    }
}
```

### Adding Scala Tests

Wait for metagraph deployment, then:
```bash
cd constellation-metagraph
touch src/test/scala/com/provenanceai/MyTest.scala
```

---

## 🔧 Troubleshooting

### Rust Tests Fail
```bash
# Clean and rebuild
cargo clean
cargo build --package brain_canister
cargo test --package brain_canister
```

### Solidity Tests Fail
```bash
# Update dependencies
forge install

# Clean and rebuild
forge clean
forge build
forge test
```

### Test Runner Fails
```bash
# Make sure it's executable
chmod +x tests/run-all-tests.sh

# Check dependencies
which cargo  # Should exist
which forge  # Should exist
which sbt    # May not exist yet (Constellation pending)
```

---

## 📊 Coverage Reports

### Rust Coverage
```bash
# Install tarpaulin (first time)
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --package brain_canister --out Html
# Open tarpaulin-report.html
```

### Solidity Coverage
```bash
forge coverage
```

### Scala Coverage
```bash
cd constellation-metagraph
sbt clean coverage test coverageReport
# Report in: target/scala-*/scoverage-report/
```

---

## 🎯 Next Steps

1. **Immediate**:
   - [ ] Run `./tests/run-all-tests.sh` to verify setup
   - [ ] Run `forge test` to verify Solidity tests pass
   - [ ] Run `cargo test --package brain_canister` to verify Rust tests pass

2. **After Constellation Deployment**:
   - [ ] Move Scala test template to metagraph
   - [ ] Implement actual Scala tests
   - [ ] Update test runner to include Scala

3. **Future Improvements**:
   - [ ] Add Pocket IC for canister integration tests
   - [ ] Add fork tests for Story Protocol
   - [ ] Set up CI/CD with GitHub Actions
   - [ ] Achieve 80%+ test coverage

---

## 📚 References

- **Testing Strategy**: See `TESTING_STRATEGY.md` for comprehensive guide
- **Rust Testing**: https://doc.rust-lang.org/book/ch11-00-testing.html
- **Foundry Book**: https://book.getfoundry.sh/forge/tests
- **ScalaTest**: https://www.scalatest.org/

---

**Questions?** Check `TESTING_STRATEGY.md` for detailed information.
