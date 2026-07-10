//! Read TRC721 (NFT) contract state with the built-in typed instance.
//!
//! `provider.trc721(addr)` returns a [`Trc721Instance`] with typed methods for
//! the standard TRC721 interface — no ABI file or `sol!` binding required.
//!
//! No private key required (read-only).
//!
//! Required env:
//!   TRON_CONTRACT — TRC721 contract address
//!
//! Optional env:
//!   TRON_TOKEN_ID — token id to inspect (default: 1)
//!   TRON_API_KEY  — TronGrid API key
//!
//! ```bash
//! TRON_CONTRACT=<addr> cargo run -p examples-trc721 --example trc721_query
//! ```

use tronz::{
    ProviderBuilder, TRONGRID_NILE,
    contract::Trc721Ext,
    primitives::{Address, U256},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let contract_str = std::env::var("TRON_CONTRACT").expect("TRON_CONTRACT env var required");
    let api_key = std::env::var("TRON_API_KEY").ok();
    let token_id: U256 = std::env::var("TRON_TOKEN_ID")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .map(U256::from)
        .unwrap_or(U256::from(1u64));

    let contract: Address = contract_str.parse()?;

    let provider = ProviderBuilder::new().maybe_api_key(api_key).on_grpc(TRONGRID_NILE).await?;

    let nft = provider.trc721(contract);

    println!("=== TRC721 metadata ===");
    println!("  contract : {contract}");
    println!("  name     : {}", nft.name().await?);
    println!("  symbol   : {}", nft.symbol().await?);

    println!("\n=== Token #{token_id} ===");
    match nft.owner_of(token_id).await {
        Ok(owner) => println!("  owner    : {owner}"),
        Err(e) => println!("  owner    : <unavailable> ({e})"),
    }
    match nft.token_uri(token_id).await {
        Ok(uri) => println!("  tokenURI : {uri}"),
        Err(e) => println!("  tokenURI : <unavailable> ({e})"),
    }

    Ok(())
}
