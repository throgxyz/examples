//! Update metadata and free-bandwidth limits for a TRC10 token.
//!
//! The signer must be the token issuer.
//!
//! Required env:
//!   TRON_PRIVATE_KEY — issuer private key
//!   TRON_ASSET_ID    — numeric ID of the token to update
//!
//! Optional env:
//!   TRON_API_KEY     — TronGrid API key

use anyhow::Context as _;
use tronz::{LocalSigner, ProviderBuilder, TRONGRID_NILE, providers::ext::Trc10Api as _};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let signer =
        LocalSigner::from_hex(&std::env::var("TRON_PRIVATE_KEY").context("TRON_PRIVATE_KEY")?)?;
    let issuer = signer.address();
    let asset_id = std::env::var("TRON_ASSET_ID").context("TRON_ASSET_ID")?;
    let api_key = std::env::var("TRON_API_KEY").ok();
    let provider = ProviderBuilder::new()
        .with_signer(signer)
        .maybe_api_key(api_key)
        .connect_grpc(TRONGRID_NILE)
        .await?;

    let asset = provider.get_asset_info(&asset_id).await?.context("token not found")?;
    anyhow::ensure!(asset.owner == issuer, "the signer is not the token issuer");

    provider
        .update_trc10()
        .description("updated with tronz")
        .url("https://example.com/token")
        .new_limit(1_000)
        .new_public_limit(10_000)
        .send()
        .await?
        .require_success()
        .get_receipt()
        .await?;

    let updated = provider.get_asset_info(&asset.id).await?.context("token not found")?;
    println!("updated {} ({}) at {}", updated.name, updated.id, updated.url);

    Ok(())
}
