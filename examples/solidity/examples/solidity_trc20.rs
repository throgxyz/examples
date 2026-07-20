//! Read a TRC20 token from solidified state using typed bindings.
//!
//! Since 0.4.1, contract bindings implement `ContractReadProvider`, which both
//! FullNode providers *and* `SolidityProvider` satisfy. That means the same
//! ergonomic `.trc20()` / `tron_sol!` handles work against a SolidityNode — you
//! read irreversible state without dropping to a manual `trigger_constant_contract`
//! call (compare `solidity_constant_call.rs`).
//!
//! No private key required (read-only).
//!
//! Required env:
//!   TRON_CONTRACT — TRC20 contract address
//!
//! Optional env:
//!   TRON_ADDRESS  — holder to query (defaults to the contract address itself)
//!   TRON_API_KEY  — TronGrid API key
//!
//! ```bash
//! TRON_CONTRACT=TXLAQ63Xg1NAzckPwKHvzw7CSEmLMEqcdj \
//!   cargo run -p examples-solidity --example solidity_trc20
//! ```

use tronz::{Address, SolidityProvider, TRONGRID_NILE_SOLIDITY, contract::Trc20Ext};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let contract: Address =
        std::env::var("TRON_CONTRACT").expect("TRON_CONTRACT env var required").parse()?;
    let holder: Address =
        std::env::var("TRON_ADDRESS").map(|s| s.parse()).unwrap_or(Ok(contract))?;
    let api_key = std::env::var("TRON_API_KEY").ok();

    // ── Connect to a SolidityNode ─────────────────────────────────────────────
    let solidity =
        SolidityProvider::builder().maybe_api_key(api_key).connect(TRONGRID_NILE_SOLIDITY).await?;

    // ── Bind the TRC20 instance to the read-only provider ─────────────────────
    //
    // `solidity.trc20(addr)` reads solidified state; a FullNode provider's
    // `.trc20(addr)` reads latest state. Same handle, different finality.
    let token = solidity.trc20(contract);

    println!("=== TRC20 {contract} (solidified) ===");
    println!("  name         : {}", token.name().await?);
    println!("  symbol       : {}", token.symbol().await?);
    println!("  decimals     : {}", token.decimals().await?);
    println!("  total_supply : {}", token.total_supply().await?);

    let balance = token.balance_of(holder).await?;
    println!("\n=== Balance of {holder} ===");
    println!("  {balance} (raw units)");

    Ok(())
}
