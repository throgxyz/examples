//! Approve a TRC721 (NFT) spender — single token or all tokens.
//!
//! Demonstrates both approval flavours on the built-in [`Trc721Instance`]:
//!
//!   * `approve(to, token_id)`            — approve one token
//!   * `set_approval_for_all(op, true)`   — approve an operator for every token
//!
//! Required env:
//!   TRON_PRIVATE_KEY — hex private key (token owner)
//!   TRON_CONTRACT    — TRC721 contract address
//!   TRON_SPENDER     — address to approve
//!
//! Optional env:
//!   TRON_TOKEN_ID    — approve a single token id; if unset, approves the
//!                      spender as an operator for all tokens instead
//!   TRON_API_KEY     — TronGrid API key
//!
//! ```bash
//! TRON_PRIVATE_KEY=<key> TRON_CONTRACT=<addr> TRON_SPENDER=<addr> TRON_TOKEN_ID=<id> \
//!   cargo run -p examples-trc721 --example trc721_approve
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
    let spender_str = std::env::var("TRON_SPENDER").expect("TRON_SPENDER env var required");
    let token_id = std::env::var("TRON_TOKEN_ID").ok().and_then(|s| s.parse::<u64>().ok());
    let api_key = std::env::var("TRON_API_KEY").ok();

    let signer = LocalSigner::from_hex(&key_hex)?;
    let owner = signer.address();

    let contract: Address = contract_str.parse()?;
    let spender: Address = spender_str.parse()?;

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .with_signer(signer)
        .maybe_api_key(api_key)
        .on_grpc(TRONGRID_NILE)
        .await?;

    let nft = provider.trc721(contract);

    println!("=== TRC721 approve ===");
    println!("  contract : {contract}");
    println!("  owner    : {owner}");
    println!("  spender  : {spender}");

    let pending = match token_id {
        Some(id) => {
            println!("  mode     : single token #{id}");
            nft.approve(spender, U256::from(id)).await?
        }
        None => {
            println!("  mode     : operator for all tokens");
            nft.set_approval_for_all(spender, true).await?
        }
    };

    println!("\n  tx_id  : 0x{}", hex::encode(pending.tx_id()));
    let info = pending.get_receipt().await?;
    println!("  status : {:?}", info.status);

    Ok(())
}
