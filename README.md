# tronz/examples

Runnable examples for the [tronz](https://github.com/throgxyz/tronz) TRON SDK.

These examples track **tronz `0.4.0`**.

> **Note:** the Solidity-node examples use `SolidityProvider`, which is
> unreleased at the time of writing. Until the next crates.io release, the
> workspace `Cargo.toml` carries a `[patch.crates-io]` that builds against a
> local `../tronz` checkout. Remove that patch once the version ships.

All examples target the [Nile testnet](https://nileex.io). Read-only examples run without a private key; examples that send transactions require `TRON_PRIVATE_KEY=<hex>`.

## Usage

```sh
# Read-only
cargo run -p examples-queries --example query

# With private key
TRON_PRIVATE_KEY=<hex> cargo run -p examples-staking --example stake
```

## Overview

This repository contains the following examples:

- [x] Queries
  - [x] [Query chain state](./examples/queries/examples/query.rs)
  - [x] [Address format conversions](./examples/queries/examples/address_formats.rs)
  - [x] [TRX amount math](./examples/queries/examples/amount_math.rs)
  - [x] [Connect to a custom endpoint](./examples/queries/examples/connect_custom.rs)
  - [x] [List witnesses (SRs)](./examples/queries/examples/list_witnesses.rs)
  - [x] [List governance proposals](./examples/queries/examples/governance_list.rs)
- [x] Signers
  - [x] [Generate a random keypair](./examples/signers/examples/signer_generate.rs)
  - [x] [Sign and verify a hash](./examples/signers/examples/signer_local.rs)
  - [x] [Derive from a BIP-39 mnemonic](./examples/signers/examples/signer_mnemonic.rs)
  - [x] [Encrypt and decrypt a keystore](./examples/signers/examples/signer_keystore.rs)
  - [x] [Sign with an AWS KMS key](./examples/signers/examples/signer_aws.rs)
- [x] Transfers
  - [x] [Send TRX](./examples/transfers/examples/transfer_trx.rs)
  - [x] [Send TRX with a memo](./examples/transfers/examples/transfer_trx_memo.rs)
- [x] Staking
  - [x] [Stake 2.0: freeze, delegate, claim](./examples/staking/examples/stake.rs)
  - [x] [Stake 1.0 (legacy): freeze and unfreeze](./examples/staking/examples/stake_v1.rs)
  - [x] [Freeze for bandwidth](./examples/staking/examples/stake_bandwidth.rs)
  - [x] [Delegate resource](./examples/staking/examples/delegate.rs)
  - [x] [Undelegate resource](./examples/staking/examples/undelegate.rs)
  - [x] [Unfreeze (V2)](./examples/staking/examples/unfreeze.rs)
  - [x] [Cancel a pending unfreeze](./examples/staking/examples/cancel_unfreeze.rs)
  - [x] [Withdraw an expired unfreeze](./examples/staking/examples/withdraw_unfreeze.rs)
  - [x] [Claim block/vote rewards](./examples/staking/examples/claim_rewards.rs)
  - [x] [Vote for a witness](./examples/staking/examples/vote_witness.rs)
- [x] Contracts
  - [x] [Read-only contract call](./examples/contracts/examples/contract_call.rs)
  - [x] [State-changing contract call](./examples/contracts/examples/contract_send.rs)
  - [x] [Deploy a contract](./examples/contracts/examples/contract_deploy.rs)
  - [x] [Call a contract via JSON ABI](./examples/contracts/examples/contract_dynamic_abi.rs)
  - [x] [Estimate energy cost](./examples/contracts/examples/contract_estimate_energy.rs)
  - [x] [Handle revert errors](./examples/contracts/examples/contract_revert.rs)
  - [x] [Decode an event log](./examples/contracts/examples/decode_log.rs)
  - [x] [Decode a transaction receipt](./examples/contracts/examples/decode_receipt.rs)
- [x] Sol macro (`tron_sol!`)
  - [x] [Type-safe bindings from a Solidity interface](./examples/sol-macro/examples/tron_sol_bindings.rs)
  - [x] [Query typed events with generated filters](./examples/sol-macro/examples/tron_sol_events.rs)
- [x] Solidity node (irreversible state)
  - [x] [Read solidified blocks, accounts, and receipts](./examples/solidity/examples/solidity_query.rs)
  - [x] [Constant call against solidified state](./examples/solidity/examples/solidity_constant_call.rs)
  - [x] [Broadcast, then wait for solidification](./examples/solidity/examples/solidity_await.rs)
- [x] TRC10
  - [x] [Query token metadata by ID](./examples/trc10/examples/trc10_query.rs)
  - [x] [Query token metadata by name](./examples/trc10/examples/trc10_by_name.rs)
  - [x] [Check a TRC10 balance](./examples/trc10/examples/trc10_balance.rs)
  - [x] [Transfer TRC10 tokens](./examples/trc10/examples/trc10_transfer.rs)
  - [x] [Issue a new TRC10 token](./examples/trc10/examples/trc10_issue.rs)
- [x] TRC20
  - [x] [Balance and transfer](./examples/trc20/examples/trc20.rs)
  - [x] [Approve and allowance](./examples/trc20/examples/trc20_approve.rs)
  - [x] [transferFrom flow](./examples/trc20/examples/trc20_transfer_from.rs)
  - [x] [Decode Transfer events](./examples/trc20/examples/trc20_decode_transfer_event.rs)
- [x] TRC721 (NFT)
  - [x] [Query NFT metadata and ownership](./examples/trc721/examples/trc721_query.rs)
  - [x] [Transfer an NFT](./examples/trc721/examples/trc721_transfer.rs)
  - [x] [Approve a spender or operator](./examples/trc721/examples/trc721_approve.rs)
- [x] Accounts
  - [x] [Create an account on-chain](./examples/accounts/examples/account_create.rs)
  - [x] [Set an account name](./examples/accounts/examples/account_update.rs)
  - [x] [Update account permissions (multi-sig)](./examples/accounts/examples/account_permissions.rs)

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
