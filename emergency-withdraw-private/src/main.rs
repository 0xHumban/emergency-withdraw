pub mod app_data;
pub mod constants;
pub mod provider;
pub mod transfer;
pub mod ui;
pub mod utils;
pub mod wallet;

/*
use ethers::{
    core::rand,
    signers::{coins_bip39::English, LocalWallet, MnemonicBuilder},
};
*/

use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting...");

    ui::controller::Controller::new().await?.start().await?;
    Ok(())
}
