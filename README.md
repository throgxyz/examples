# tronz/examples

Runnable examples for the [tronz](https://github.com/throgxyz/tronz) TRON SDK.

All examples target the [Nile testnet](https://nileex.io). Read-only examples run without a private key; examples that send transactions require `TRON_PRIVATE_KEY=<hex>`.

## Usage

```sh
# Read-only
cargo run -p examples-queries --example query

# With private key
TRON_PRIVATE_KEY=<hex> cargo run -p examples-staking --example stake
```

## Overview

- [ ] Queries
  - [ ] [query](./examples/queries/examples/query.rs)
  - [ ] [address_formats](./examples/queries/examples/address_formats.rs)
  - [ ] [amount_math](./examples/queries/examples/amount_math.rs)
  - [ ] [connect_custom](./examples/queries/examples/connect_custom.rs)
  - [ ] [list_witnesses](./examples/queries/examples/list_witnesses.rs)
  - [ ] [governance_list](./examples/queries/examples/governance_list.rs)
- [ ] Signers
  - [ ] [signer_generate](./examples/signers/examples/signer_generate.rs)
  - [ ] [signer_local](./examples/signers/examples/signer_local.rs)
  - [ ] [signer_mnemonic](./examples/signers/examples/signer_mnemonic.rs)
  - [ ] [signer_keystore](./examples/signers/examples/signer_keystore.rs)
- [ ] Transfers
  - [ ] [transfer_trx](./examples/transfers/examples/transfer_trx.rs)
  - [ ] [transfer_trx_memo](./examples/transfers/examples/transfer_trx_memo.rs)
- [ ] Staking
  - [ ] [stake](./examples/staking/examples/stake.rs)
  - [ ] [stake_v1](./examples/staking/examples/stake_v1.rs)
  - [ ] [stake_bandwidth](./examples/staking/examples/stake_bandwidth.rs)
  - [ ] [delegate](./examples/staking/examples/delegate.rs)
  - [ ] [undelegate](./examples/staking/examples/undelegate.rs)
  - [ ] [unfreeze](./examples/staking/examples/unfreeze.rs)
  - [ ] [cancel_unfreeze](./examples/staking/examples/cancel_unfreeze.rs)
  - [ ] [withdraw_unfreeze](./examples/staking/examples/withdraw_unfreeze.rs)
  - [ ] [claim_rewards](./examples/staking/examples/claim_rewards.rs)
  - [ ] [vote_witness](./examples/staking/examples/vote_witness.rs)
- [ ] Contracts
  - [ ] [contract_call](./examples/contracts/examples/contract_call.rs)
  - [ ] [contract_send](./examples/contracts/examples/contract_send.rs)
  - [ ] [contract_deploy](./examples/contracts/examples/contract_deploy.rs)
  - [ ] [contract_dynamic_abi](./examples/contracts/examples/contract_dynamic_abi.rs)
  - [ ] [contract_estimate_energy](./examples/contracts/examples/contract_estimate_energy.rs)
  - [ ] [contract_revert](./examples/contracts/examples/contract_revert.rs)
  - [ ] [decode_log](./examples/contracts/examples/decode_log.rs)
  - [ ] [decode_receipt](./examples/contracts/examples/decode_receipt.rs)
- [ ] TRC10
  - [ ] [trc10_query](./examples/trc10/examples/trc10_query.rs)
  - [ ] [trc10_by_name](./examples/trc10/examples/trc10_by_name.rs)
  - [ ] [trc10_balance](./examples/trc10/examples/trc10_balance.rs)
  - [ ] [trc10_transfer](./examples/trc10/examples/trc10_transfer.rs)
  - [ ] [trc10_issue](./examples/trc10/examples/trc10_issue.rs)
- [ ] TRC20
  - [ ] [trc20](./examples/trc20/examples/trc20.rs)
  - [ ] [trc20_approve](./examples/trc20/examples/trc20_approve.rs)
  - [ ] [trc20_transfer_from](./examples/trc20/examples/trc20_transfer_from.rs)
  - [ ] [trc20_decode_transfer_event](./examples/trc20/examples/trc20_decode_transfer_event.rs)
- [ ] Accounts
  - [ ] [account_create](./examples/accounts/examples/account_create.rs)
  - [ ] [account_update](./examples/accounts/examples/account_update.rs)
  - [ ] [account_permissions](./examples/accounts/examples/account_permissions.rs)

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
