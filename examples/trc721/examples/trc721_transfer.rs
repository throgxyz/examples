//! Transfer a TRC721 (NFT) token and wait for confirmation.
//!
//! Uses the built-in [`Trc721Instance`] `transfer_from` method, which builds a
//! `TriggerSmartContract`, fills TAPOS + fee_limit, signs, and broadcasts.
//!
//! Required env:
//!   TRON_PRIVATE_KEY — hex private key (current token owner)
//!   TRON_CONTRACT    — TRC721 contract address
//!   TRON_TO          — recipient address
//!   TRON_TOKEN_ID    — token id to transfer
//!
//! Optional env:
//!   TRON_API_KEY     — TronGrid API key
//!
//! ```bash
//! TRON_PRIVATE_KEY=<key> TRON_CONTRACT=<addr> TRON_TO=<addr> TRON_TOKEN_ID=<id> \
//!   cargo run -p examples-trc721 --example trc721_transfer
//! ```

use tronz::{
    LocalSigner, ProviderBuilder, TRONGRID_NILE, TronSigner,
    contract::Trc721Ext,
    primitives::{Address, U256},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let key_hex = std::env::var("TRON_PRIVATE_KEY").expect("TRON_PRIVATE_KEY env var required");
    let contract_str = std::env::var("TRON_CONTRACT").expect("TRON_CONTRACT env var required");
    let to_str = std::env::var("TRON_TO").expect("TRON_TO env var required");
    let token_id: U256 = std::env::var("TRON_TOKEN_ID")
        .expect("TRON_TOKEN_ID env var required")
        .parse::<u64>()
        .map(U256::from)?;
    let api_key = std::env::var("TRON_API_KEY").ok();

    let signer = LocalSigner::from_hex(&key_hex)?;
    let from = signer.address();

    let contract: Address = contract_str.parse()?;
    let to: Address = to_str.parse()?;

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .with_signer(signer)
        .maybe_api_key(api_key)
        .on_grpc(TRONGRID_NILE)
        .await?;

    let nft = provider.trc721(contract);

    println!("=== TRC721 transfer ===");
    println!("  contract : {contract}");
    println!("  token    : {token_id}");
    println!("  from     : {from}");
    println!("  to       : {to}");

    let pending = nft.transfer_from(from, to, token_id).await?;
    println!("\n  tx_id  : 0x{}", hex::encode(pending.tx_id()));

    println!("  waiting for confirmation…");
    let info = pending.get_receipt().await?;
    println!("\n=== Receipt ===");
    println!("  status       : {:?}", info.status);
    println!("  energy used  : {}", info.energy_usage);
    if let Some(ref reason) = info.revert_reason {
        println!("  revert reason: {reason}");
    }

    Ok(())
}
