//! Connect to both gRPC services exposed by a local TronBox Runtime Environment.
//!
//! Start TRE first, then run:
//!
//! ```bash
//! cargo run -p examples-queries --example connect_local_tre
//! ```
//!
//! Optional env:
//!   TRON_FULL_NODE_URL     — default: http://127.0.0.1:50051
//!   TRON_SOLIDITY_NODE_URL — default: http://127.0.0.1:50052

use tronz::{ProviderBuilder, SolidityProvider, TronProvider};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let full_node =
        std::env::var("TRON_FULL_NODE_URL").unwrap_or_else(|_| "http://127.0.0.1:50051".to_owned());
    let solidity_node = std::env::var("TRON_SOLIDITY_NODE_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:50052".to_owned());

    let full_provider = ProviderBuilder::new().connect_grpc(full_node.as_str()).await?;
    let solidity_provider = SolidityProvider::connect(solidity_node.as_str()).await?;

    let full_block = full_provider.get_now_block().await?;
    let solid_block = solidity_provider.get_now_block().await?;

    println!("FullNode block:     #{}", full_block.number);
    println!("SolidityNode block: #{}", solid_block.number);

    Ok(())
}
