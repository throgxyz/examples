//! Activate a new account on the Nile testnet.
//!
//! An address becomes activated when it receives any amount of TRX or TRC-10,
//! or when an existing account explicitly creates it. The sender pays the
//! network's account-creation fee separately from the transferred amount and
//! may also pay for Bandwidth when its available Bandwidth is insufficient.
//!
//! Required env:
//!   TRON_PRIVATE_KEY — funded account paying for activation
//!   TRON_TO          — new address to activate (must not already exist on-chain)
//!
//! Optional env:
//!   TRON_API_KEY     — TronGrid API key
//!
//! ```bash
//! TRON_PRIVATE_KEY=<key> TRON_TO=<new-addr> cargo run -p examples-accounts --example account_create
//! ```

use tronz::{LocalSigner, ProviderBuilder, TRONGRID_NILE, TronProvider, Trx};

/// Amount delivered to the new account; independent of network activation fees.
const TRANSFER_AMOUNT_SUN: i64 = 1;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let key_hex = std::env::var("TRON_PRIVATE_KEY").expect("TRON_PRIVATE_KEY env var required");
    let new_addr_str = std::env::var("TRON_TO").expect("TRON_TO env var required");
    let api_key = std::env::var("TRON_API_KEY").ok();

    let signer = LocalSigner::from_hex(&key_hex)?;
    let payer = signer.address();
    let new_addr: tronz::Address = new_addr_str.parse()?;

    let provider = ProviderBuilder::new()
        .with_signer(signer)
        .maybe_api_key(api_key)
        .connect_grpc(TRONGRID_NILE)
        .await?;

    let payer_account = provider.get_account(payer).await?;
    println!("=== Payer ===");
    println!("  address : {payer}");
    println!("  balance : {} TRX", payer_account.balance);

    let target = provider.get_account(new_addr).await?;
    println!("\n=== Target account {} ===", new_addr);
    if target.is_activated {
        println!("  already activated — nothing to do");
        return Ok(());
    }
    println!("  not yet activated");

    //
    // Sending any positive TRX amount to an address that does not exist
    // activates it. The transferred amount is not the activation fee: the
    // sender also pays the chain's account-creation fee and may pay a Bandwidth
    // burn fee. Those values are governance-controlled chain parameters, so the
    // example deliberately does not hard-code them.

    let amount = Trx::from_sun(TRANSFER_AMOUNT_SUN)?;
    println!("\n=== Activating with {} ===", amount);
    println!("  network activation fees are charged separately");
    println!("  broadcasting…");

    let pending = provider.send_trx().to(new_addr).amount(amount).send().await?;
    println!("  tx_id   : 0x{}", hex::encode(pending.tx_id()));

    println!("  waiting for confirmation…");
    let info = pending.get_receipt().await?;
    println!("  status  : {:?}", info.status);
    println!("  net fee : {} sun", info.net_fee.as_sun());

    let after = provider.get_account(new_addr).await?;
    println!("\n=== Result ===");
    println!("  activated : {}", after.is_activated);
    println!("  balance   : {} TRX", after.balance);

    Ok(())
}
