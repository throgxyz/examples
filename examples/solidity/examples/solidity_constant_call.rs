//! Execute a constant (read-only) contract call against solidified state.
//!
//! `SolidityProvider` has no `tron_sol!` / `Interface` binding sugar (those need
//! a full `TronProvider`), so this shows the lower-level path: ABI-encode the
//! calldata yourself, hand it to `trigger_constant_contract`, and decode the
//! returned bytes. Here we read a TRC20 `balanceOf(address)` from irreversible
//! state.
//!
//! No private key required.
//!
//! Required env:
//!   TRON_CONTRACT — TRC20 contract address
//!
//! Optional env:
//!   TRON_ADDRESS  — holder to query (defaults to the contract address itself)
//!
//! ```bash
//! TRON_CONTRACT=TXLAQ63Xg1NAzckPwKHvzw7CSEmLMEqcdj \
//!   cargo run -p examples-solidity --example solidity_constant_call
//! ```

use alloy_dyn_abi::{DynSolValue, FunctionExt as _, JsonAbiExt as _};
use alloy_json_abi::Function;
use tronz::{
    Address, SolidityProvider, TRONGRID_NILE_SOLIDITY, Trx, providers::types::TriggerSmartContract,
};

const BALANCE_OF: &str = r#"{
    "name": "balanceOf",
    "type": "function",
    "inputs": [{"name": "account", "type": "address"}],
    "outputs": [{"name": "", "type": "uint256"}],
    "stateMutability": "view"
}"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let contract: Address =
        std::env::var("TRON_CONTRACT").expect("TRON_CONTRACT env var required").parse()?;
    let holder: Address =
        std::env::var("TRON_ADDRESS").map(|s| s.parse()).unwrap_or(Ok(contract))?;

    let solidity = SolidityProvider::connect(TRONGRID_NILE_SOLIDITY).await?;

    // ── Encode calldata ───────────────────────────────────────────────────────
    //
    // `FunctionExt::abi_encode_input` prepends the 4-byte selector, producing the
    // full calldata TRON expects. `DynSolValue::Address` takes a 20-byte alloy
    // address (no 0x41 prefix).
    let func: Function = serde_json::from_str(BALANCE_OF)?;
    let data = func.abi_encode_input(&[DynSolValue::Address(holder.into())])?;

    // ── Constant call against solidified state ────────────────────────────────
    let params = TriggerSmartContract {
        owner_address: holder,
        contract_address: contract,
        call_value: Trx::ZERO,
        data: data.into(),
        call_token_value: Trx::ZERO,
        token_id: 0,
    };
    let result = solidity.trigger_constant_contract(params).await?;

    if let Some(reason) = result.revert_reason {
        anyhow::bail!("call reverted: {reason}");
    }

    // ── Decode the ABI-encoded return data ────────────────────────────────────
    let decoded = func.abi_decode_output(&result.output)?;
    let balance = match decoded.into_iter().next() {
        Some(DynSolValue::Uint(n, _)) => n,
        other => anyhow::bail!("unexpected return value: {other:?}"),
    };

    println!("=== balanceOf (solidified) ===");
    println!("  contract    : {contract}");
    println!("  holder      : {holder}");
    println!("  raw units   : {balance}");
    println!("  energy used : {}", result.energy_used);

    Ok(())
}
