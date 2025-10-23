# Makefile for ProvenanceAI
# Rust/ICP idiomatic approach for task management

.PHONY: help test build deploy clean start stop

# Default target
.DEFAULT_GOAL := help

## help: Show this help message
help:
	@echo "ProvenanceAI - Available Commands"
	@echo "=================================="
	@echo ""
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' | sed -e 's/^/ /'
	@echo ""

## test: Run all tests (ICP + Story + Constellation)
test:
	@bash tests/run-all-tests.sh

## test-icp: Run Rust tests only
test-icp:
	@cd packages/icp && cargo test

## test-story: Run Solidity tests only
test-story:
	@cd packages/story && forge test

## test-story-verbose: Run Solidity tests with verbose output
test-story-verbose:
	@cd packages/story && forge test -vvv

## build: Build all components
build:
	@bash scripts/build_all.sh

## build-icp: Build ICP canister only
build-icp:
	@dfx build brain_canister

## build-story: Build Story contracts only
build-story:
	@cd packages/story && forge build

## deploy-local: Deploy to local dfx network
deploy-local:
	@bash scripts/deploy_all.sh

## deploy-playground: Deploy to ICP Playground (free, 20min expiry)
deploy-playground:
	@bash scripts/deploy_playground.sh

## deploy-mainnet: Deploy to ICP mainnet (requires DFX_PASSWORD)
deploy-mainnet:
	@bash scripts/deploy_mainnet.sh

## start: Start local dfx network in background
start:
	@dfx start --background

## stop: Stop local dfx network (use 'make backup' first to preserve state)
stop:
	@dfx stop

## restart: Restart dfx (with auto-backup before stop)
restart:
	@echo "🔒 AUTO-BACKUP: Creating backup before restart..."
	@bash scripts/backup_canister_state.sh || true
	@echo ""
	@make stop
	@make start

## clean: Clean all build artifacts and stop dfx (AUTO-BACKUP INCLUDED)
clean:
	@echo "🔒 AUTO-BACKUP: Creating backup before clean..."
	@bash scripts/backup_canister_state.sh || true
	@echo ""
	@dfx stop 2>/dev/null || true
	@rm -rf .dfx
	@cd packages/icp && cargo clean
	@cd packages/story && forge clean
	@echo "✅ All artifacts cleaned (backup saved to ~/.provenanceai-backups/)"

## status: Show canister status
status:
	@dfx canister status brain_canister

## logs: Show canister logs
logs:
	@dfx canister logs brain_canister

## address: Show canister EVM address
address:
	@dfx canister call brain_canister get_canister_evm_address

## e2e: Run end-to-end integration test
e2e:
	@dfx canister call brain_canister generate_and_register_ip '(record { prompt = "Test image"; metadata = record { title = "E2E Test"; description = "Integration test"; tags = vec {"test"} }; })'

## vps-status: Check VPS Constellation status
vps-status:
	@sshpass -p 'SSH_PASSWORD_REDACTED' ssh root@198.144.183.32 'cd /root/provenanceai-deployment && bash check_status.sh'

## vps-logs: Check VPS Constellation logs
vps-logs:
	@sshpass -p 'SSH_PASSWORD_REDACTED' ssh root@198.144.183.32 'cd /root/provenanceai-deployment && bash check_logs.sh'

## fmt: Format Rust code
fmt:
	@cd packages/icp && cargo fmt

## clippy: Run Rust linter
clippy:
	@cd packages/icp && cargo clippy -- -D warnings

## check: Run cargo check
check:
	@cd packages/icp && cargo check

## backup: Backup canister state (IMPORTANT: preserves canister ID and EVM address)
backup:
	@bash scripts/backup_canister_state.sh

## restore: Restore canister state from backup
restore:
	@bash scripts/restore_canister_state.sh

## list-backups: List available backups
list-backups:
	@ls -1t ~/.provenanceai-backups/ 2>/dev/null || echo "No backups found"
