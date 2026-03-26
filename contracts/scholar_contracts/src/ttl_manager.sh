#!/bin/bash

# TTL Manager Script for Stream Scholar Contracts
# This script monitors and automatically bumps the TTL for contract ledger entries
# to prevent contract expiration due to "rent" costs.

set -e

# Configuration
NETWORK="testnet"
CONTRACT_ID_FILE=".contract_id"
SERVICE_ACCOUNT_FILE=".service_account"
LOG_FILE="ttl_manager.log"

# Default TTL extension (in ledgers, ~1 week at 5s per ledger)
DEFAULT_LEDGERS_TO_LIVE=120960  # 7 days * 24 hours * 60 min * 60 sec / 5 sec = 120960

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "$(date '+%Y-%m-%d %H:%M:%S') ${BLUE}[INFO]${NC} $1" | tee -a "$LOG_FILE"
}

log_success() {
    echo -e "$(date '+%Y-%m-%d %H:%M:%S') ${GREEN}[SUCCESS]${NC} $1" | tee -a "$LOG_FILE"
}

log_warning() {
    echo -e "$(date '+%Y-%m-%d %H:%M:%S') ${YELLOW}[WARNING]${NC} $1" | tee -a "$LOG_FILE"
}

log_error() {
    echo -e "$(date '+%Y-%m-%d %H:%M:%S') ${RED}[ERROR]${NC} $1" | tee -a "$LOG_FILE"
}

# Function to check if soroban-cli is installed
check_dependencies() {
    if ! command -v soroban &> /dev/null; then
        log_error "soroban-cli is not installed. Please install it first."
        echo "Visit: https://github.com/stellar/soroban-cli"
        exit 1
    fi
    log_success "soroban-cli found"
}

# Function to check network configuration
check_network() {
    log_info "Checking network configuration for $NETWORK"

    if ! soroban config network | grep -q "$NETWORK"; then
        log_info "Setting up $NETWORK network configuration"
        soroban config network add "$NETWORK" \
            --rpc-url "https://soroban-testnet.stellar.org:443" \
            --network-passphrase "Test SDF Network ; September 2015"
    fi

    log_success "Network configuration verified"
}

# Function to get contract ID
get_contract_id() {
    if [ ! -f "$CONTRACT_ID_FILE" ]; then
        log_error "Contract ID file not found: $CONTRACT_ID_FILE"
        log_error "Please ensure the contract is deployed and the ID is saved."
        exit 1
    fi

    CONTRACT_ID=$(cat "$CONTRACT_ID_FILE" | tr -d '\n')
    if [ -z "$CONTRACT_ID" ]; then
        log_error "Contract ID is empty in $CONTRACT_ID_FILE"
        exit 1
    fi

    log_info "Using contract ID: $CONTRACT_ID"
}

# Function to get service account
get_service_account() {
    if [ ! -f "$SERVICE_ACCOUNT_FILE" ]; then
        log_error "Service account file not found: $SERVICE_ACCOUNT_FILE"
        log_error "Please create a service account for TTL management."
        log_info "Run: soroban keys generate --network $NETWORK service_account"
        log_info "Then save the address to $SERVICE_ACCOUNT_FILE"
        exit 1
    fi

    SERVICE_ACCOUNT=$(cat "$SERVICE_ACCOUNT_FILE" | tr -d '\n')
    if [ -z "$SERVICE_ACCOUNT" ]; then
        log_error "Service account is empty in $SERVICE_ACCOUNT_FILE"
        exit 1
    fi

    log_info "Using service account: $SERVICE_ACCOUNT"
}

# Function to check account balance
check_account_balance() {
    local account="$1"

    log_info "Checking balance for account: $account"

    # Get account balance (this will fail if account doesn't exist or has no balance)
    if ! soroban keys fund "$account" --network "$NETWORK" 2>/dev/null; then
        log_warning "Account may already be funded or funding failed"
    else
        log_success "Account funded successfully"
    fi
}

# Function to bump TTL for persistent entries
bump_persistent_ttl() {
    local contract_id="$1"
    local account="$2"
    local ledgers_to_live="${3:-$DEFAULT_LEDGERS_TO_LIVE}"

    log_info "Bumping TTL for persistent entries (ledgers to live: $ledgers_to_live)"

    if soroban contract bump \
        --id "$contract_id" \
        --durability persistent \
        --ledgers-to-live "$ledgers_to_live" \
        --source "$account" \
        --network "$NETWORK"; then

        log_success "TTL bumped successfully for persistent entries"
    else
        log_error "Failed to bump TTL for persistent entries"
        return 1
    fi
}

# Function to bump TTL for temporary entries (if any)
bump_temporary_ttl() {
    local contract_id="$1"
    local account="$2"
    local ledgers_to_live="${3:-$DEFAULT_LEDGERS_TO_LIVE}"

    log_info "Bumping TTL for temporary entries (ledgers to live: $ledgers_to_live)"

    if soroban contract bump \
        --id "$contract_id" \
        --durability temporary \
        --ledgers-to-live "$ledgers_to_live" \
        --source "$account" \
        --network "$NETWORK"; then

        log_success "TTL bumped successfully for temporary entries"
    else
        log_warning "Failed to bump TTL for temporary entries (may not exist)"
    fi
}

# Main function
main() {
    log_info "Starting TTL Manager for Stream Scholar Contracts"

    # Check dependencies
    check_dependencies

    # Check network
    check_network

    # Get contract ID
    get_contract_id

    # Get service account
    get_service_account

    # Check account balance
    check_account_balance "$SERVICE_ACCOUNT"

    # Bump TTL for persistent entries
    bump_persistent_ttl "$CONTRACT_ID" "$SERVICE_ACCOUNT"

    # Bump TTL for temporary entries
    bump_temporary_ttl "$CONTRACT_ID" "$SERVICE_ACCOUNT"

    log_success "TTL Manager completed successfully"
}

# Run main function
main "$@"
