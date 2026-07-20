//! Read solidified (irreversible) state from a TRON SolidityNode.
//!
//! A `SolidityProvider` targets `protocol.WalletSolidity`, which only serves
//! state confirmed by 2/3+ of the super representatives. It is read-only by
//! construction — no signer, no fillers, no broadcast — so it is the safest
//! source of truth for balances and receipts you never want reorged away.
//!
//! No private key required.
//!
//! Optional env:
//!   TRON_ADDRESS — address to query (defaults to a well-known Nile account)
//!   TRON_TX_ID   — a 32-byte tx id (hex) to look up a solidified receipt
//!
//! ```
//! cargo run -p examples-solidity --example solidity_query
//! ```

use tronz::{SolidityProvider, TRONGRID_NILE_SOLIDITY, primitives::ResourceCode};

// A well-known TRON address with on-chain activity (mainnet USDT contract; also
// present on Nile testnet with TRX balance).
const DEFAULT_ADDR: &str = "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr_str = std::env::var("TRON_ADDRESS").unwrap_or_else(|_| DEFAULT_ADDR.to_owned());
    let address = addr_str.parse().expect("valid TRON address");

    // ── Connect to a SolidityNode ─────────────────────────────────────────────
    //
    // Use `SolidityProvider::builder()` if you need custom timeouts, retries,
    // failover endpoints, or a TronGrid API key.
    let solidity = SolidityProvider::connect(TRONGRID_NILE_SOLIDITY).await?;

    // ── Latest solidified block ───────────────────────────────────────────────
    //
    // This lags the FullNode head by ~19 blocks (the solidification window).
    let head = solidity.get_now_block().await?;
    println!("=== Solidified head ===");
    println!("  number    : {}", head.number);
    println!("  timestamp : {} ms", head.timestamp);
    println!("  hash      : 0x{}", hex::encode(head.hash));

    // ── A specific solidified block ───────────────────────────────────────────
    let block = solidity.get_block_by_number(head.number).await?;
    let tx_count = solidity.get_transaction_count_by_block_num(head.number).await?;
    println!("\n=== Block {} ===", block.number);
    println!("  hash         : 0x{}", hex::encode(block.hash));
    println!("  transactions : {tx_count}");

    // ── Account state (irreversible) ──────────────────────────────────────────
    let account = solidity.get_account(address).await?;
    println!("\n=== Account {address} ===");
    println!("  balance   : {} TRX", account.balance);
    println!("  activated : {}", account.is_activated);

    // ── Staking & governance (solidified) ─────────────────────────────────────
    //
    // Since 0.4.1 the SolidityNode also serves the stake/delegation and witness
    // queries, so these mirror the FullNode `TronProvider` methods against
    // irreversible state.
    let idx = solidity.get_delegated_resource_index(address).await?;
    let max_energy = solidity.get_can_delegate_max(address, ResourceCode::Energy).await?;
    let witnesses = solidity.list_witnesses().await?;
    let active = witnesses.iter().filter(|w| w.is_active).count();
    println!("\n=== Staking & governance ===");
    println!("  delegating to    : {} accounts", idx.to_accounts.len());
    println!("  receiving from   : {} accounts", idx.from_accounts.len());
    println!("  max delegatable  : {max_energy} TRX (energy)");
    println!("  witnesses        : {} ({active} active)", witnesses.len());

    // ── Optional receipt lookup ───────────────────────────────────────────────
    //
    // `get_transaction_info` returns `None` until the transaction has solidified,
    // which is exactly the signal the `wait_for_*` helpers poll on.
    if let Ok(tx_hex) = std::env::var("TRON_TX_ID") {
        let tx_id = tx_hex.parse().expect("valid 32-byte hex tx id");
        match solidity.get_transaction_info(tx_id).await? {
            Some(info) => {
                println!("\n=== Receipt {tx_hex} ===");
                println!("  block       : {}", info.block_number);
                println!("  status      : {:?}", info.status);
                println!("  energy used : {}", info.energy_usage);
            }
            None => println!("\n{tx_hex} has not solidified yet"),
        }
    }

    Ok(())
}
