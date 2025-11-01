use chrono::Utc;
use bip39::{Mnemonic, Seed, Language};
use sha2::{Sha512, Digest};
use ed25519_dalek::SigningKey;
use stellar_strkey::Strkey;
use std::convert::TryInto;

#[derive(Debug)]
pub struct TradeLog {
    pub timestamp: String,
    pub side: String,
    pub amount: f64,
    pub asset: String,
    pub destination: String,
    pub tx_hash: String,
}

pub fn derive_keypair_from_mnemonic() -> (String, String) {
    let mnemonic = Mnemonic::from_phrase("SOUMWPSSECRETKEY", Language::English).unwrap();
    let seed = Seed::new(&mnemonic, "");
    let hash = Sha512::digest(seed.as_bytes());
    let secret_bytes = &hash[0..32];

    let signing_key = SigningKey::from_bytes(secret_bytes.try_into().unwrap());
    let verifying_key = signing_key.verifying_key();
    let public_bytes = verifying_key.to_bytes();
    let public_key = Strkey::PublicKeyEd25519(
        stellar_strkey::ed25519::PublicKey(public_bytes)
    ).to_string();


    let secret_key = Strkey::PrivateKeyEd25519(
        stellar_strkey::ed25519::PrivateKey(secret_bytes.try_into().unwrap())
    ).to_string();  

    (public_key, secret_key)
}

pub async fn submit_trade(
    _secret: &str,
    destination: &str,
    amount: f64,
    asset: &str,
) -> Result<TradeLog, Box<dyn std::error::Error>> {
    let timestamp = Utc::now().to_rfc3339();
    let tx_hash = format!("{}-{}-{}", amount, asset, &destination[0..8]);

    println!("Submitting {} {} to {} at {}", amount, asset, destination, timestamp);

    Ok(TradeLog {
        timestamp,
        side: "buy".to_string(),
        amount,
        asset: asset.to_string(),
        destination: destination.to_string(),
        tx_hash,
    })
}
