//! Create and manage a native TRON exchange pool.
//!
//! The account must own enough TRX and the selected TRC10 asset. Creating an
//! exchange costs network fees and leaves persistent on-chain state, so use Nile.
//!
//! Required env:
//!   TRON_PRIVATE_KEY — issuer/owner private key
//!   TRON_ASSET_ID    — numeric TRC10 asset ID owned by the account
//!
//! Optional env:
//!   TRON_API_KEY     — TronGrid API key
//!
//! ```bash
//! TRON_PRIVATE_KEY=<key> TRON_ASSET_ID=<id> \
//!   cargo run -p examples-exchange --example exchange_lifecycle
//! ```

use anyhow::Context as _;
use tronz::{LocalSigner, ProviderBuilder, TRONGRID_NILE, providers::ext::ExchangeApi as _};

const TRX_TOKEN_ID: &str = "_";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let signer =
        LocalSigner::from_hex(&std::env::var("TRON_PRIVATE_KEY").context("TRON_PRIVATE_KEY")?)?;
    let owner = signer.address();
    let asset_id = std::env::var("TRON_ASSET_ID").context("TRON_ASSET_ID")?;
    let api_key = std::env::var("TRON_API_KEY").ok();

    let provider = ProviderBuilder::new()
        .with_signer(signer)
        .maybe_api_key(api_key)
        .connect_grpc(TRONGRID_NILE)
        .await?;
    let existing_exchange_ids = provider
        .list_exchanges()
        .await?
        .into_iter()
        .map(|exchange| exchange.exchange_id)
        .collect::<std::collections::HashSet<_>>();

    provider
        .exchange_create()
        .first_token_id(TRX_TOKEN_ID)
        .first_token_balance(1_000_000)
        .second_token_id(&asset_id)
        .second_token_balance(1_000)
        .send()
        .await?
        .require_success()
        .get_receipt()
        .await?;

    let exchange = provider
        .list_exchanges()
        .await?
        .into_iter()
        .filter(|exchange| {
            !existing_exchange_ids.contains(&exchange.exchange_id)
                && exchange.creator_address == owner
                && exchange.first_token_id == TRX_TOKEN_ID
                && exchange.second_token_id == asset_id
        })
        .max_by_key(|exchange| exchange.exchange_id)
        .context("created exchange was not found")?;

    println!("created exchange #{}", exchange.exchange_id);

    provider
        .exchange_inject()
        .exchange_id(exchange.exchange_id)
        .token_id(TRX_TOKEN_ID)
        .quant(100_000)
        .send()
        .await?
        .require_success()
        .get_receipt()
        .await?;

    provider
        .exchange_withdraw()
        .exchange_id(exchange.exchange_id)
        .token_id(TRX_TOKEN_ID)
        .quant(10_000)
        .send()
        .await?
        .require_success()
        .get_receipt()
        .await?;

    provider
        .exchange_trade()
        .exchange_id(exchange.exchange_id)
        .token_id(TRX_TOKEN_ID)
        .quant(1_000)
        .expected(1)
        .send()
        .await?
        .require_success()
        .get_receipt()
        .await?;

    let updated =
        provider.get_exchange_by_id(exchange.exchange_id).await?.context("exchange disappeared")?;
    println!(
        "pool balances: {}={}, {}={}",
        updated.first_token_id,
        updated.first_token_balance,
        updated.second_token_id,
        updated.second_token_balance
    );

    Ok(())
}
