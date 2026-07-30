//! Sign, recover, and verify a TronWeb-compatible plaintext message.
//!
//! `TronSigner::sign_message` matches TronWeb's `signMessageV2`. The signature
//! can be exported with a legacy `v` value (`27`/`28`) for TronWeb consumers.
//!
//! No network access required.
//!
//! Optional env:
//!   TRON_PRIVATE_KEY — hex key (defaults to a throwaway demo key)
//!   TRON_MESSAGE     — message to sign (default: "hello from tronz")
//!
//! ```bash
//! cargo run -p examples-signers --example signer_message
//! ```

use tronz::{LocalSigner, TronSigner, recover_message_address, verify_message};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let key_hex = std::env::var("TRON_PRIVATE_KEY").unwrap_or_else(|_| {
        "0000000000000000000000000000000000000000000000000000000000000001".to_owned()
    });
    let message = std::env::var("TRON_MESSAGE").unwrap_or_else(|_| "hello from tronz".to_owned());

    let signer = LocalSigner::from_hex(&key_hex)?;
    let expected = signer.address();
    let signature = signer.sign_message(message.as_bytes()).await?;

    // TronWeb accepts the recoverable signature as r || s || v with legacy
    // recovery values 27/28.
    let tronweb_signature = signature.to_legacy_bytes();
    let recovered = recover_message_address(message.as_bytes(), &signature)?;
    let verified = verify_message(message.as_bytes(), &signature, expected);

    println!("=== TronWeb-compatible message signature ===");
    println!("  message   : {message:?}");
    println!("  signer    : {expected}");
    println!("  signature : 0x{}", hex::encode(tronweb_signature));
    println!("  recovered : {recovered}");
    println!("  verified  : {verified}");

    assert_eq!(recovered, expected);
    assert!(verified);
    assert!(!verify_message(b"a different message", &signature, expected));

    Ok(())
}
