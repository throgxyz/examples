//! Read the native TRON order-book market.
//!
//! No signer is required. `"_"` is TRON's reserved token ID for native TRX.
//!
//! Optional env:
//!   TRON_SELL_TOKEN_ID — sell-side TRC10 ID (default: 1002000)
//!   TRON_BUY_TOKEN_ID  — buy-side token ID (default: _ for TRX)
//!   TRON_ORDER_ID      — 32-byte order ID in hex
//!   TRON_ADDRESS       — account whose orders should be listed
//!   TRON_API_KEY       — TronGrid API key
//!
//! ```bash
//! cargo run -p examples-exchange --example market_queries
//! ```

use tronz::{
    Address, ProviderBuilder, TRONGRID_NILE, primitives::B256, providers::ext::MarketApi as _,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sell_token = std::env::var("TRON_SELL_TOKEN_ID").unwrap_or_else(|_| "1002000".to_owned());
    let buy_token = std::env::var("TRON_BUY_TOKEN_ID").unwrap_or_else(|_| "_".to_owned());
    let api_key = std::env::var("TRON_API_KEY").ok();
    let provider =
        ProviderBuilder::new().maybe_api_key(api_key).connect_grpc(TRONGRID_NILE).await?;

    let pairs = provider.get_market_pair_list().await?;
    let prices = provider.get_market_price_by_pair(&sell_token, &buy_token).await?;
    let orders = provider.get_market_order_list_by_pair(&sell_token, &buy_token).await?;
    println!("pairs={}, price_levels={}, open_orders={}", pairs.len(), prices.len(), orders.len());

    if let Ok(order_id) = std::env::var("TRON_ORDER_ID") {
        let order_id: B256 = order_id.parse()?;
        match provider.get_market_order_by_id(order_id).await? {
            Some(order) => println!("order: {order:?}"),
            None => println!("order not found"),
        }
    }

    if let Ok(address) = std::env::var("TRON_ADDRESS") {
        let address: Address = address.parse()?;
        let account_orders = provider.get_market_order_by_account(address).await?;
        println!("account orders: {}", account_orders.len());
    }

    Ok(())
}
