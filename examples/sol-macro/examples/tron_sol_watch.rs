//! Watch new TRC20 `Transfer` events with an `EventWatcher`.
//!
//! TRON has no log-subscription RPC, so the watcher polls new blocks and decodes
//! matching receipt logs. By default events are held for 19 confirmations.
//!
//! Required env:
//!   TRON_CONTRACT — TRC20 contract address
//!
//! Optional env:
//!   TRON_API_KEY      — TronGrid API key
//!   TRON_CONFIRMATIONS — confirmations before reporting (default: 19)
//!
//! ```bash
//! TRON_CONTRACT=<addr> cargo run -p examples-sol-macro --example tron_sol_watch
//! ```

use futures::StreamExt;
use tronz::{Address, ProviderBuilder, TRONGRID_NILE, contract::tron_sol};

tron_sol! {
    #[sol(rpc)]
    interface ITrc20 {
        event Transfer(address indexed from, address indexed to, uint256 value);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let contract: Address =
        std::env::var("TRON_CONTRACT").expect("TRON_CONTRACT env var required").parse()?;
    let confirmations: i64 =
        std::env::var("TRON_CONFIRMATIONS").unwrap_or_else(|_| "19".to_owned()).parse()?;
    let api_key = std::env::var("TRON_API_KEY").ok();

    let provider =
        ProviderBuilder::new().maybe_api_key(api_key).connect_grpc(TRONGRID_NILE).await?;
    let token = ITrc20::new(contract, provider);

    // `watch()` starts at the block after the current head. Use `watch_from(n)`
    // instead when resuming from a persisted block cursor.
    let watcher = token.Transfer_filter().address(contract).watch().await?;
    println!("Watching Transfer events from {contract} ({confirmations} confirmations)...");

    let mut events = watcher.confirmations(confirmations).into_stream();
    while let Some(event) = events.next().await {
        let transfer = event?;
        let from: Address = transfer.from.into();
        let to: Address = transfer.to.into();
        println!("{from} -> {to}: {}", transfer.value);
    }

    Ok(())
}
