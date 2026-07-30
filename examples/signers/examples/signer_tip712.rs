//! Sign dynamic TIP-712 typed data in the same shape used by TronWeb.
//!
//! TIP-712 reuses EIP-712 encoding. Address fields therefore use 20-byte EVM
//! addresses, not base58check TRON addresses.
//!
//! No network access required.
//!
//! Optional env:
//!   TRON_PRIVATE_KEY — hex key (defaults to a throwaway demo key)
//!
//! ```bash
//! cargo run -p examples-signers --example signer_tip712
//! ```

use tronz::{LocalSigner, TronSigner, signers::TypedData};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let key_hex = std::env::var("TRON_PRIVATE_KEY").unwrap_or_else(|_| {
        "0000000000000000000000000000000000000000000000000000000000000001".to_owned()
    });
    let signer = LocalSigner::from_hex(&key_hex)?;

    // This is the JSON object passed to TronWeb's signTypedData API.
    let payload: TypedData = serde_json::from_str(
        r#"{
          "types": {
            "EIP712Domain": [
              { "name": "name", "type": "string" },
              { "name": "version", "type": "string" },
              { "name": "chainId", "type": "uint256" },
              { "name": "verifyingContract", "type": "address" }
            ],
            "Transfer": [
              { "name": "to", "type": "address" },
              { "name": "amount", "type": "uint256" }
            ]
          },
          "primaryType": "Transfer",
          "domain": {
            "name": "TRON Token",
            "version": "1",
            "chainId": 728126428,
            "verifyingContract": "0xa614f803B6FD780986A42c78Ec9c7f77e6DeD13C"
          },
          "message": {
            "to": "0x0000000000000000000000000000000000000001",
            "amount": "1000000"
          }
        }"#,
    )?;

    let signing_hash = payload.eip712_signing_hash()?;
    let signature = signer.sign_dynamic_typed_data(&payload).await?;
    let recovered = signature.recover_address_from_prehash(signing_hash)?;

    println!("=== TIP-712 typed data signature ===");
    println!("  signer    : {}", signer.address());
    println!("  hash      : 0x{}", hex::encode(signing_hash));
    println!("  signature : 0x{}", hex::encode(signature.to_legacy_bytes()));
    println!("  recovered : {recovered}");

    assert_eq!(recovered, signer.address());
    Ok(())
}
