//! TRX ↔ sun conversions and amount arithmetic.
//!
//! TRON denominates all on-chain values in *sun* (`i64`), where:
//!
//! ```text
//! 1 TRX = 1_000_000 sun
//! ```
//!
//! [`Trx`] wraps `i64` sun and provides:
//!
//! - `from_sun` constructor (rejects negatives) and `as_sun` accessor
//! - exact decimal parsing via `str::parse` / `parse_trx` (no `f64`, 6 dp)
//! - fixed-precision formatting via `Display` / `format_trx`
//! - `+` / `-` operators that panic on overflow or a negative result
//! - non-panicking `checked_add` / `checked_sub`
//!
//! TRC20 token amounts use [`U256`] (256-bit unsigned) — identical to ERC-20.
//!
//! No network access required.
//!
//! ```bash
//! cargo run -p examples-queries --example amount_math
//! ```

use tronz::{
    Trx, U256,
    primitives::{SUN_PER_TRX, format_trx, parse_trx},
};

fn main() -> anyhow::Result<()> {
    // ── TRX ↔ sun conversions ─────────────────────────────────────────────────

    println!("=== TRX ↔ sun ===");
    println!("  1 TRX = {SUN_PER_TRX} sun  (SUN_PER_TRX)");

    // From a raw sun value (what the chain returns).
    let one_trx = Trx::from_sun(1_000_000)?;
    println!("  from_sun(1_000_000) = {one_trx}  ({} sun)", one_trx.as_sun());

    // From an exact decimal string — no floating point, up to 6 dp.
    let half: Trx = "0.5".parse()?;
    let two_and_half = parse_trx("2.5")?; // free-fn alias, like alloy's parse_ether
    println!("  \"0.5\".parse()       = {half}  ({} sun)", half.as_sun());
    println!("  parse_trx(\"2.5\")     = {two_and_half}  ({} sun)", two_and_half.as_sun());

    // Display is exact and fixed to 6 fractional digits; `format_trx` is the
    // free-fn equivalent (mirrors alloy's `format_ether`).
    println!("  format_trx(2.5 TRX) = {}", format_trx(two_and_half));

    // ── Arithmetic ────────────────────────────────────────────────────────────
    //
    // `+` / `-` panic on overflow or a negative result — amounts are always
    // non-negative, so this surfaces logic bugs instead of silently wrapping.

    println!("\n=== Arithmetic ===");

    let a: Trx = "10".parse()?;
    let b: Trx = "3.5".parse()?;
    println!("  a = {a}");
    println!("  b = {b}");
    println!("  a + b = {}", a + b);
    println!("  a - b = {}", a - b);

    // ── Checked arithmetic ────────────────────────────────────────────────────
    //
    // Use the checked variants for untrusted input: they return `None` instead
    // of panicking on overflow or a negative result.

    println!("\n=== Checked arithmetic ===");

    let x: Trx = "5".parse()?;
    let y: Trx = "2".parse()?;
    println!("  5 - 2 = {:?}", x.checked_sub(y)); // Some(3)
    println!("  2 - 5 = {:?}  (None = would be negative)", y.checked_sub(x));

    let max = Trx::from_sun(i64::MAX)?;
    println!("  MAX + 1 = {:?}  (None = overflow)", max.checked_add(Trx::from_sun(1)?));

    // ── Ordering ──────────────────────────────────────────────────────────────

    println!("\n=== Ordering ===");

    let amounts: [Trx; 4] = ["100".parse()?, "1.5".parse()?, "50".parse()?, Trx::ZERO];
    let min = amounts.iter().min().copied().unwrap();
    let max = amounts.iter().max().copied().unwrap();
    println!("  min = {min}");
    println!("  max = {max}");

    // ── U256 for TRC20 tokens ─────────────────────────────────────────────────
    //
    // TRC20 balances and transfer amounts use U256, matching the ERC-20 ABI.
    // A 6-decimal token (e.g. USDT) represents 1 unit as 1_000_000 in U256.

    println!("\n=== U256 for TRC20 amounts ===");

    let usdt_decimals: u32 = 6;
    let one_usdt = U256::from(10u64).pow(U256::from(usdt_decimals));
    println!("  1 USDT   = {one_usdt} (raw U256 with 6 decimals)");

    let hundred_usdt = U256::from(100u64) * one_usdt;
    println!("  100 USDT = {hundred_usdt} (raw U256)");

    // Human-readable formatting helper (divide by 10^decimals).
    let whole = hundred_usdt / one_usdt;
    let frac = hundred_usdt % one_usdt;
    println!("  display  : {whole}.{frac:0>6} USDT");

    // 18-decimal token (like WETH bridged to TRON).
    let weth_decimals: u32 = 18;
    let one_weth = U256::from(10u64).pow(U256::from(weth_decimals));
    println!("\n  1 WETH   = {one_weth} (raw U256 with 18 decimals)");

    Ok(())
}
