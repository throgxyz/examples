//! Set `msg.sender` for a read-only contract call with `.caller()`.
//!
//! Since 0.4.1, read-only calls made without a signer default `msg.sender` to
//! the **zero address** (most `view` functions ignore the caller). When a view
//! *does* branch on `msg.sender` — allowances, per-caller quotes, access checks
//! — use `.caller(address)` to set it explicitly. `.caller()` is available on
//! `ContractInstance`, the dynamic-ABI call builder, and `tron_sol!` bindings.
//!
//! No private key required (read-only).
//!
//! Required env:
//!   TRON_CONTRACT — TRC20 contract address
//!   TRON_CALLER   — address to present as `msg.sender`
//!
//! Optional env:
//!   TRON_API_KEY  — TronGrid API key
//!
//! ```bash
//! TRON_CONTRACT=<addr> TRON_CALLER=<addr> \
//!   cargo run -p examples-contracts --example contract_call_as
//! ```

use alloy_dyn_abi::DynSolValue;
use alloy_json_abi::JsonAbi;
use tronz::{
    Address, ProviderBuilder, TRONGRID_NILE,
    contract::{ContractExt, Interface},
};

// Minimal ABI: `allowance(owner, spender)` is a natural caller-sensitive view —
// a spender typically queries its own allowance, i.e. `spender == msg.sender`.
const ERC20_ABI: &str = r#"[
    {
        "name": "allowance",
        "type": "function",
        "inputs": [
            {"name": "owner", "type": "address"},
            {"name": "spender", "type": "address"}
        ],
        "outputs": [{"name": "", "type": "uint256"}],
        "stateMutability": "view"
    }
]"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let contract: Address =
        std::env::var("TRON_CONTRACT").expect("TRON_CONTRACT env var required").parse()?;
    let caller: Address =
        std::env::var("TRON_CALLER").expect("TRON_CALLER env var required").parse()?;
    let api_key = std::env::var("TRON_API_KEY").ok();

    let provider = ProviderBuilder::new().maybe_api_key(api_key).on_grpc(TRONGRID_NILE).await?;

    let abi: JsonAbi = serde_json::from_str(ERC20_ABI)?;

    // `.caller(caller)` sets `msg.sender` for every constant call on this handle.
    // Without it, `msg.sender` would be the zero address.
    let instance = provider.contract(contract, Interface::new(abi)).caller(caller);

    // Query the caller's allowance granted by itself (owner == spender == caller).
    let args = [DynSolValue::Address(caller.into()), DynSolValue::Address(caller.into())];
    let allowance = match instance.call("allowance", &args).await?.into_iter().next() {
        Some(DynSolValue::Uint(n, _)) => n,
        other => anyhow::bail!("unexpected return value: {other:?}"),
    };

    println!("=== allowance (as msg.sender = {caller}) ===");
    println!("  contract  : {contract}");
    println!("  allowance : {allowance} (raw units)");

    Ok(())
}
