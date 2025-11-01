use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long)]
    pub asset: String,

    #[arg(long)]
    pub amount: u32,

    #[arg(long)]
    pub send_to: String,

    #[arg(long)]
    pub public: Option<String>,

    #[arg(long)]
    pub secret: Option<String>,

    #[arg(long)]
    pub mnemonic: String,
}
