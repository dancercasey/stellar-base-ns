use stellar_sdk::{Network, TransactionBuilder, Server, KeyPair};

pub fn submit_trade(public: &str, secret: &str) {
    let server = Server::new("https://horizon.stellar.org").unwrap();
    let kp = KeyPair::from_secret(secret).unwrap();

    let account = server.load_account(&kp.public_key()).unwrap();
    let tx = TransactionBuilder::new(&account, Network::new_public())
        .add_text_memo("Validator-grade swap")
        .build()
        .unwrap();

    let signed = tx.sign(&kp, &Network::new_public()).unwrap();
    let response = server.submit_transaction(&signed);

    println!("Response: {:?}", response);
}
