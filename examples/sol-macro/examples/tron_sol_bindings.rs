//! Generate type-safe contract bindings with the `tron_sol!` macro.
//!
//! `tron_sol!` generates ABI types and a provider-bound `Instance` from Solidity.
//! Attributes on bare `struct` and `enum` declarations are forwarded to the generated types.
//!
//! No private key required (read-only).
//!
//! Required env:
//!   TRON_CONTRACT — TRC20 contract address (e.g. USDT on Nile)
//!
//! Optional env:
//!   TRON_API_KEY  — TronGrid API key
//!
//! ```bash
//! TRON_CONTRACT=<addr> cargo run -p examples-sol-macro --example tron_sol_bindings
//! ```

use tronz::{
    ProviderBuilder, TRONGRID_NILE,
    contract::tron_sol,
    primitives::{Address as TronAddress, U256},
};

// A single `tron_sol!` invocation can declare multiple items. Here we mix a
// bare `struct` (with forwarded `#[derive(...)]`) and an `#[sol(rpc)]`
// interface that becomes a provider-bound `Instance`.
tron_sol! {
    // `#[derive(...)]` on a bare struct is forwarded to the generated type,
    // so `TokenMeta` is a normal Rust struct you can build and debug-print.
    #[derive(Debug, Default, PartialEq)]
    struct TokenMeta {
        string name;
        string symbol;
        uint8 decimals;
    }

    // `#[sol(rpc)]` turns this interface into `IErc20::Instance`, with a typed
    // async method for every function below.
    #[sol(rpc)]
    interface IErc20 {
        function name() external view returns (string);
        function symbol() external view returns (string);
        function decimals() external view returns (uint8);
        function totalSupply() external view returns (uint256);
        function balanceOf(address owner) external view returns (uint256);
        function transfer(address to, uint256 amount) external returns (bool);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let contract_str = std::env::var("TRON_CONTRACT").expect("TRON_CONTRACT env var required");
    let api_key = std::env::var("TRON_API_KEY").ok();

    let contract: TronAddress = contract_str.parse()?;

    let provider =
        ProviderBuilder::new().maybe_api_key(api_key).connect_grpc(TRONGRID_NILE).await?;

    //
    // `IErc20::new` takes (address, provider) and returns a typed `Instance`.
    let token = IErc20::new(contract, provider);

    //
    // Each call returns a builder; `.call().await` performs a constant
    // (read-only) `trigger_constant_contract` and decodes the return value to
    // its native Rust type — no manual ABI encoding or decoding.
    let name: String = token.name().call().await?;
    let symbol: String = token.symbol().call().await?;
    let decimals: u8 = token.decimals().call().await?;
    let supply: U256 = token.totalSupply().call().await?;

    println!("=== tron_sol! typed reads ===");
    println!("  contract : {contract}");
    println!("  name     : {name}");
    println!("  symbol   : {symbol}");
    println!("  decimals : {decimals}");
    println!("  supply   : {supply}");

    // `balanceOf(address)` accepts any `Into<Address>`, so a tronz `Address`
    // works directly (its 0x41 prefix is stripped for the ABI encoding).
    let balance: U256 = token.balanceOf(contract).call().await?;
    println!("  self-bal : {balance}");

    let meta = TokenMeta { name, symbol, decimals };
    println!("\n=== forwarded #[derive] on bare struct ===");
    println!("  {meta:?}");
    assert_ne!(meta, TokenMeta::default());

    //
    // With a signer-backed provider you would send it:
    //   let pending = token.transfer(to, U256::from(1u64)).send().await?;
    // `.send()` fills TAPOS + fee_limit, signs, and broadcasts.
    println!("\n(transfer(...).send() requires a signer-backed provider — see contract_send)");

    Ok(())
}
