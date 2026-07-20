//! Page through super representatives sorted by real-time vote count.
//!
//! `list_witnesses()` returns the full SR + candidate set in one shot.
//! `get_paginated_now_witness_list(offset, limit)` (java-tron 4.8.1's
//! `GetPaginatedNowWitnessList`) returns a page already sorted by *live* vote
//! count — handy for leaderboards without fetching and sorting everything.
//!
//! No private key required (read-only).
//!
//! Optional env:
//!   TRON_API_KEY — TronGrid API key
//!   TRON_LIMIT   — page size (default: 10)
//!
//! ```bash
//! cargo run -p examples-queries --example witness_pagination
//! ```

use tronz::{ProviderBuilder, TRONGRID_NILE, TronProvider};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = std::env::var("TRON_API_KEY").ok();
    let limit: i64 = std::env::var("TRON_LIMIT").ok().and_then(|s| s.parse().ok()).unwrap_or(10);

    let provider = ProviderBuilder::new().maybe_api_key(api_key).on_grpc(TRONGRID_NILE).await?;

    // First page: the current top `limit` SRs by real-time votes.
    let top = provider.get_paginated_now_witness_list(0, limit).await?;

    println!("=== Top {limit} witnesses (by live votes) ===");
    println!("  {:<3}  {:<35}  {:>14}  URL", "#", "Address", "Votes");
    println!("  {}", "-".repeat(80));
    for (i, w) in top.iter().enumerate() {
        println!(
            "  {:>2}   {:<35}  {:>14}  {}",
            i + 1,
            w.address.to_string(),
            w.vote_count,
            if w.url.len() > 40 { &w.url[..40] } else { &w.url },
        );
    }

    // Second page: the next `limit`, using `offset` to continue.
    let next = provider.get_paginated_now_witness_list(limit, limit).await?;
    if next.is_empty() {
        println!("\n  (no further pages)");
    } else {
        println!("\n=== Next page (offset {limit}) ===");
        for (i, w) in next.iter().enumerate() {
            println!(
                "  {:>2}   {:<35}  {:>14}",
                limit + i as i64 + 1,
                w.address.to_string(),
                w.vote_count
            );
        }
    }

    Ok(())
}
