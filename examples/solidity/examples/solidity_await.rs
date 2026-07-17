//! Broadcast on a FullNode, then wait for the transaction to *solidify*.
//!
//! FullNode inclusion (`await_confirmed`) can still be reorged; solidified state
//! cannot. `PendingTransaction::await_solidified_success` bridges the two: it
//! broadcasts through the FullNode provider, then polls a `SolidityProvider`
//! until the transaction is irreversible *and* its execution succeeded.
//!
//! Required env:
//!   TRON_PRIVATE_KEY — hex private key (no 0x prefix)
//!
//! Optional env:
//!   TRON_API_KEY     — TronGrid API key
//!   TRON_TO          — recipient (defaults to a well-known Nile address)
//!   TRON_AMOUNT_SUN  — amount in sun (default: 1_000_000 = 1 TRX)
//!
//! ```
//! TRON_PRIVATE_KEY=<key> cargo run -p examples-solidity --example solidity_await
//! ```

use core::time::Duration;

use tronz::{
    Address, LocalSigner, ProviderBuilder, SolidityProvider, TRONGRID_NILE, TRONGRID_NILE_SOLIDITY,
    TronProvider, TronSigner, Trx,
};

// Solidification lags the head by ~19 blocks (~57 s). Poll a little past that.
const POLL_INTERVAL: Duration = Duration::from_secs(3);
const POLL_ATTEMPTS: u32 = 40; // ~120 s worst case

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let key_hex = std::env::var("TRON_PRIVATE_KEY").expect("TRON_PRIVATE_KEY env var required");
    let api_key = std::env::var("TRON_API_KEY").ok();
    let amount_sun: i64 =
        std::env::var("TRON_AMOUNT_SUN").ok().and_then(|s| s.parse().ok()).unwrap_or(1_000_000);

    let signer = LocalSigner::from_hex(&key_hex)?;
    let from = signer.address();
    // Default to a well-known Nile address; TRON does not allow self-transfers.
    let to: Address = std::env::var("TRON_TO")
        .ok()
        .map(|s| s.parse().expect("valid TRON_TO address"))
        .unwrap_or_else(|| "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".parse().unwrap());
    let amount = Trx::from_sun(amount_sun)?;

    // ── Two providers: one to broadcast, one to confirm irreversibility ────────
    let full = ProviderBuilder::new()
        .with_recommended_fillers()
        .with_signer(signer)
        .maybe_api_key(api_key.clone())
        .on_grpc(TRONGRID_NILE)
        .await?;

    let solidity =
        SolidityProvider::builder().maybe_api_key(api_key).connect(TRONGRID_NILE_SOLIDITY).await?;

    println!("From   : {from}");
    println!("To     : {to}");
    println!("Amount : {amount}");

    // ── Broadcast on the FullNode ─────────────────────────────────────────────
    println!("\nBroadcasting…");
    let pending = full.send_trx().to(to).amount(amount).send().await?;
    let tx_id = pending.tx_id();
    println!("tx_id  : 0x{}", hex::encode(tx_id));

    // ── Wait for irreversible success on the SolidityNode ──────────────────────
    println!("Waiting for solidification (may take ~1 min)…");
    let info =
        pending.await_solidified_success_with(&solidity, POLL_INTERVAL, POLL_ATTEMPTS).await?;

    println!("\n=== Solidified ===");
    println!("  block       : {}", info.block_number);
    println!("  status      : {:?}", info.status);
    println!("  energy used : {}", info.energy_usage);
    println!("  net fee     : {} sun", info.net_fee.as_sun());

    Ok(())
}
