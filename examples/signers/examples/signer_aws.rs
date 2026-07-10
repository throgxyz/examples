//! Sign with an AWS KMS-backed key — the private key never leaves the HSM.
//!
//! `AwsSigner` wraps a KMS `ECC_SECG_P256K1` asymmetric signing key. On
//! construction it fetches the public key once (via `GetPublicKey`) and derives
//! the TRON address; signing is delegated to the KMS `Sign` API, and the
//! recovery id is reconstructed locally by trial recovery.
//!
//! Requires the `signer-aws` feature (enabled for this crate) and working AWS
//! credentials in the environment (e.g. `AWS_ACCESS_KEY_ID`,
//! `AWS_SECRET_ACCESS_KEY`, `AWS_REGION`) or an IAM role.
//!
//! Required env:
//!   AWS_KEY_ID — KMS key id or ARN of a secp256k1 signing key
//!
//! ```bash
//! AWS_KEY_ID=<key-id> cargo run -p examples-signers --example signer_aws
//! ```

use aws_config::BehaviorVersion;
use aws_sdk_kms::Client;
use tronz::{AwsSigner, TronSigner, primitives::B256};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let key_id = std::env::var("AWS_KEY_ID").expect("AWS_KEY_ID env var required");

    // Load AWS config + credentials from the standard sources (env vars, shared
    // config files, or IAM role), then build a KMS client.
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    // Fetches the public key once and caches the derived TRON address.
    let signer = AwsSigner::new(client, key_id).await?;

    println!("=== AWS KMS signer ===");
    println!("  key id  : {}", signer.key_id());
    println!("  address : {}", signer.address());

    // Sign an arbitrary 32-byte hash. The signature is a TRON-style
    // recoverable signature (r ‖ s ‖ v), with `s` normalized to low-S.
    let hash = B256::repeat_byte(0xab);
    let sig = signer.sign_hash(hash).await?;

    println!("\n=== Signature ===");
    println!("  hash : 0x{}", hex::encode(hash));
    println!("  sig  : 0x{}", hex::encode(sig.to_bytes()));
    println!("  v    : {}", sig.v());

    Ok(())
}
