//! Send TRX through an already-configured multi-signature permission.
//!
//! This example does not change account permissions. It builds one unsigned
//! transaction, signs its tx id with every configured key in a `TronWallet`,
//! asks the node for the accumulated sign weight, then broadcasts only after
//! the permission threshold is met.
//!
//! Required env:
//!   TRON_ACCOUNT       — account that owns the existing multi-sig permission
//!   TRON_PRIVATE_KEYS  — comma-separated hex private keys in that permission
//!   TRON_TO            — recipient address
//!
//! Optional env:
//!   TRON_PERMISSION_ID — active permission id (default: 2)
//!   TRON_AMOUNT_SUN    — amount in sun (default: 1)
//!   TRON_API_KEY       — TronGrid API key
//!
//! ```bash
//! TRON_ACCOUNT=<addr> TRON_PRIVATE_KEYS=<key1,key2> TRON_TO=<addr> \
//!   cargo run -p examples-accounts --example multisig_send
//! ```

use tronz::{
    Address, LocalSigner, ProviderBuilder, TRONGRID_NILE, TronNetworkWallet, TronProvider,
    TronWallet, Trx, providers::types::SignedTransaction,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let account: Address =
        std::env::var("TRON_ACCOUNT").expect("TRON_ACCOUNT env var required").parse()?;
    let to: Address = std::env::var("TRON_TO").expect("TRON_TO env var required").parse()?;
    let permission_id: i32 =
        std::env::var("TRON_PERMISSION_ID").unwrap_or_else(|_| "2".to_owned()).parse()?;
    let amount_sun: i64 =
        std::env::var("TRON_AMOUNT_SUN").unwrap_or_else(|_| "1".to_owned()).parse()?;
    let api_key = std::env::var("TRON_API_KEY").ok();

    let keys = std::env::var("TRON_PRIVATE_KEYS").expect("TRON_PRIVATE_KEYS env var required");
    let mut signers = keys.split(',').map(str::trim).filter(|key| !key.is_empty());
    let first = LocalSigner::from_hex(
        signers.next().ok_or_else(|| anyhow::anyhow!("TRON_PRIVATE_KEYS is empty"))?,
    )?;
    let mut wallet = TronWallet::new(first);
    for key in signers {
        wallet.register_signer(LocalSigner::from_hex(key)?);
    }
    let signer_addresses: Vec<_> = wallet.signer_addresses().collect();

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .maybe_api_key(api_key)
        .connect_grpc(TRONGRID_NILE)
        .await?;

    println!("=== Existing multi-sig transfer ===");
    println!("  account       : {account}");
    println!("  permission id : {permission_id}");
    println!("  recipient     : {to}");
    println!("  amount        : {amount_sun} sun");
    for address in &signer_addresses {
        println!("  signing key   : {address}");
    }

    let raw = provider
        .send_trx()
        .from(account)
        .to(to)
        .amount(Trx::from_sun(amount_sun)?)
        .permission_id(permission_id)
        .build()
        .await?;
    let tx_id = raw.tx_id();
    let signatures = wallet.sign_hash_with_many(&signer_addresses, &tx_id).await?;
    let signed = SignedTransaction { raw, signatures };

    let weight = provider.get_transaction_sign_weight(&signed).await?;
    println!("\n=== Sign weight ===");
    println!("  current  : {}", weight.current_weight);
    println!("  required : {}", weight.required_weight);
    println!("  result   : {}", weight.result);
    for address in &weight.approved_list {
        println!("  approved : {address}");
    }

    if weight.current_weight < weight.required_weight {
        anyhow::bail!(
            "signature weight {} is below required threshold {}; not broadcasting",
            weight.current_weight,
            weight.required_weight
        );
    }

    let pending = provider.broadcast(signed).await?;
    println!("\nBroadcast tx_id: 0x{}", hex::encode(pending.tx_id()));
    let receipt = pending.get_receipt().await?;
    println!("Status: {:?}", receipt.status);

    Ok(())
}
