//! Generate a fresh random secp256k1 key pair.
//!
//! Creates a cryptographically secure random private key, derives the TRON
//! address, and keeps the key material out of stdout. In real usage, write the
//! private key directly to secure storage.
//!
//! No network access required.
//!
//! ```bash
//! cargo run -p examples-signers --example signer_generate
//! ```
use k256::ecdsa::SigningKey;
use tronz::{LocalSigner, primitives::Address};

fn main() -> anyhow::Result<()> {
    // k256::ecdsa::SigningKey::random generates a cryptographically secure key
    // using the OS random number generator (getrandom).
    let key = SigningKey::random(&mut rand::rngs::OsRng);
    let key_bytes: [u8; 32] = key.to_bytes().into();

    let signer = LocalSigner::from_bytes(&key_bytes)?;
    let address: Address = signer.address();

    println!("=== Generated key pair ===");
    println!("  private key : generated (not printed)");
    println!("  address     : {address}  (base58check)");
    println!("  address hex : {}", address.to_hex());
    println!("  address evm : 0x{}", hex::encode(address.as_evm_bytes()));

    let key2 = SigningKey::random(&mut rand::rngs::OsRng);
    let signer2 = LocalSigner::from_bytes(&key2.to_bytes().into())?;
    assert_ne!(signer.address(), signer2.address(), "fresh keys are unique");
    println!("\n  second run  : {}", signer2.address());
    println!("  unique      : {}", signer.address() != signer2.address());

    println!("\n=== Next steps ===");
    println!("  1. Store generated key bytes in a secrets manager or encrypted keystore.");
    println!("  2. Get Nile TRX from the faucet: https://nileex.io/");
    println!("     (send to: {address})");

    Ok(())
}
