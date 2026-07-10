//! Query and decode typed events with `tron_sol!`-generated filters.
//!
//! When an `#[sol(rpc)]` interface declares `event` items, `tron_sol!` generates
//! a per-event filter method on the `Instance`:
//!
//!   * `token.Transfer_filter()` → a [`TronEventFilter`] typed to `Transfer`
//!   * `.address(addr)` narrows it to one emitter
//!   * `.query_tx(tx_id)`  → decode matching events from one transaction
//!   * `.query_block(num)` → decode matching events from a whole block
//!
//! TRON has no `eth_getLogs`, so events are read from transaction receipts. To
//! scan a range, call `query_block` in a loop (concurrently for throughput).
//!
//! No private key required (read-only).
//!
//! Required env (one of):
//!   TRON_TX_ID    — hex tx id that emitted Transfer events, OR
//!   TRON_BLOCK    — block number to scan
//!
//! Optional env:
//!   TRON_CONTRACT — restrict to events from this contract address
//!   TRON_API_KEY  — TronGrid API key
//!
//! ```bash
//! TRON_TX_ID=<txid> cargo run -p examples-sol-macro --example tron_sol_events
//! TRON_BLOCK=<num>  cargo run -p examples-sol-macro --example tron_sol_events
//! ```

use tronz::{
    ProviderBuilder, TRONGRID_NILE,
    contract::tron_sol,
    primitives::{Address as TronAddress, B256},
};

tron_sol! {
    #[sol(rpc)]
    interface IErc20 {
        event Transfer(address indexed from, address indexed to, uint256 value);
        event Approval(address indexed owner, address indexed spender, uint256 value);
        function transfer(address to, uint256 amount) external returns (bool);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = std::env::var("TRON_API_KEY").ok();
    let contract = std::env::var("TRON_CONTRACT").ok();

    let provider = ProviderBuilder::new().maybe_api_key(api_key).on_grpc(TRONGRID_NILE).await?;

    // Any address works as the binding target; the filter can be scoped to a
    // specific emitter separately via `.address(...)`.
    let bind_addr: TronAddress = match &contract {
        Some(c) => c.parse()?,
        None => "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".parse()?,
    };
    let token = IErc20::new(bind_addr, provider);

    // Build a typed filter for the `Transfer` event, optionally narrowed to the
    // configured contract address.
    let mut filter = token.Transfer_filter();
    if let Some(c) = &contract {
        filter = filter.address(c.parse()?);
    }

    // ── Query by transaction id ───────────────────────────────────────────────
    if let Ok(tx_hex) = std::env::var("TRON_TX_ID") {
        let tx_id = B256::from_slice(&hex::decode(tx_hex.trim_start_matches("0x"))?);
        let transfers = filter.query_tx(tx_id).await?;

        println!("=== Transfer events in tx 0x{} ===", hex::encode(tx_id));
        print_transfers(&transfers);
        return Ok(());
    }

    // ── Query by block number ─────────────────────────────────────────────────
    if let Ok(block_str) = std::env::var("TRON_BLOCK") {
        let block_num: i64 = block_str.parse()?;
        let transfers = filter.query_block(block_num).await?;

        println!("=== Transfer events in block {block_num} ===");
        print_transfers(&transfers);
        return Ok(());
    }

    anyhow::bail!("set TRON_TX_ID or TRON_BLOCK");
}

fn print_transfers(transfers: &[IErc20::Transfer]) {
    if transfers.is_empty() {
        println!("  (none)");
        return;
    }
    for (i, t) in transfers.iter().enumerate() {
        // Indexed `from`/`to` are `alloy_primitives::Address`; convert to a
        // tronz `Address` to display in base58check (`T...`).
        let from: TronAddress = t.from.into();
        let to: TronAddress = t.to.into();
        println!("  [{i}] {from} -> {to} : {}", t.value);
    }
}
