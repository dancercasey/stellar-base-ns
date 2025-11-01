mod chains;

use chains::stellar::{derive_keypair_from_mnemonic, submit_trade};

#[tokio::main]
async fn main() {
    // Derive keypair from hardcoded mnemonic
    let (pubkey, seckey) = derive_keypair_from_mnemonic();

    println!("\n🔐 Public Key: {}", pubkey);
    println!("🔐 Secret Key: {}", seckey);

    // Simulate trade submission
    let result = submit_trade(&seckey, "GDESTADDR12345678", 100.0, "USDC").await;

    match result {
        Ok(log) => {
            println!("\n📦 TradeLog:");
            println!("Timestamp: {}", log.timestamp);
            println!("Side: {}", log.side);
            println!("Amount: {}", log.amount);
            println!("Asset: {}", log.asset);
            println!("Destination: {}", log.destination);
            println!("Tx Hash: {}", log.tx_hash);
        }
        Err(e) => eprintln!("❌ Error submitting trade: {}", e),
    }
}
