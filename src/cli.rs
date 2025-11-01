use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// BIP-39 mnemonic phrase
    #[arg(short, long)]
    pub mnemonic: String,
}
