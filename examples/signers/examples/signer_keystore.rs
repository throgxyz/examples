//! Web3 Secret Storage V3 keystore — encrypt and decrypt a TRON private key.
//!
//! No network access required.
//!
//! ```bash
//! cargo run -p examples-signers --example signer_keystore
//! ```
//!
//! The keystore format is compatible with TronLink, go-ethereum, and gotron-sdk.
//! It stores the TRON address in base58check format (not Ethereum hex).

use tronz::{LocalSigner, signers::keystore::KdfparamsType};

fn main() -> anyhow::Result<()> {
    let private_key = "b5a4cea271ff424d7c31dc12a3e43e401df7a40d7412a15750f3f0b6b5449a28";
    let signer = LocalSigner::from_hex(private_key)?;

    println!("=== Original signer ===");
    println!("  address : {}", signer.address());

    let dir = tempfile::tempdir()?;
    let password = "my-secure-password";

    println!("\n=== Encrypting keystore ===");
    println!("  password : {password}");

    let path = signer.encrypt_keystore(dir.path(), password)?;
    println!("  saved to : {}", path.display());

    let json = std::fs::read_to_string(&path)?;
    let ks: tronz::KeystoreFile = serde_json::from_str(&json)?;

    println!("\n=== Keystore contents ===");
    println!("  version  : {}", ks.version);
    println!("  id       : {}", ks.id);
    println!("  address  : {}", ks.address);
    match &ks.crypto.kdfparams {
        KdfparamsType::Scrypt { n, .. } => {
            println!("  kdf      : {} (N={n})", ks.crypto.kdf);
        }
        KdfparamsType::Pbkdf2 { c, .. } => {
            println!("  kdf      : {} (iterations={c})", ks.crypto.kdf);
        }
    }
    println!("  cipher   : {}", ks.crypto.cipher);

    println!("\n=== Decrypting ===");
    let recovered = LocalSigner::decrypt_keystore(&path, password)?;
    println!("  recovered address : {}", recovered.address());

    assert_eq!(signer.address(), recovered.address(), "round-trip address mismatch");
    println!("  addresses match   : true");

    let err = LocalSigner::decrypt_keystore(&path, "wrong-password").unwrap_err();
    println!("\n=== Wrong password ===");
    println!("  error : {err}");

    Ok(())
}
