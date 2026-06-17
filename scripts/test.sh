#!/usr/bin/env bash

# Exit if anything fails.
set -eo pipefail

# Utilities
GREEN="\033[00;32m"

function log () {
  echo -e "$1"
  echo "################################################################################"
  echo "#### $2 "
  echo "################################################################################"
  echo -e "\033[0m"
}

# This script builds and runs the read-only examples (no private key required).
# Examples that send transactions are excluded — run those manually with TRON_PRIVATE_KEY set.
function main () {
    # Read-only examples from each category
    readonly_examples=(
        # queries
        query
        address_formats
        amount_math
        connect_custom
        list_witnesses
        governance_list
        # signers (no network needed)
        signer_generate
        signer_local
        signer_mnemonic
        signer_keystore
        # trc10
        trc10_query
        trc10_by_name
        trc10_balance
        # contracts
        contract_call
        contract_estimate_energy
        contract_revert
        decode_log
        decode_receipt
        # trc20
        trc20_decode_transfer_event
    )

    log $GREEN "Building read-only examples..."

    cargo build $(printf -- '--example %s ' "${readonly_examples[@]}")

    log $GREEN "Running read-only examples..."

    printf '%s\n' "${readonly_examples[@]}" | xargs -P4 -I{} bash -c '
        name="$1"
        bin="./target/debug/examples/$name"

        if [[ -x "$bin" ]]; then
            if "$bin" >/dev/null 2>&1; then
                echo "OK: $name"
            else
                echo "FAIL: $name" >&2
                exit 1
            fi
        else
            echo "MISSING: $bin" >&2
            exit 1
        fi
    ' -- {}

    log $GREEN "Done"
}

main
