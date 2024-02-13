use std::time::Duration;
use eyre::{ContextCompat, Result};
use hex::ToHex;

use ethers::{
    prelude::{Address, LocalWallet, Middleware, Provider, Signer, TransactionRequest, U256},
    utils::Ganache,
};
use ethers::abi::AbiEncode;

#[tokio::main]
async fn main() -> Result<()> {
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("Endpoint: {}", ganache.endpoint());

    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let first_address = wallet.address();
    println!("wallet first address {}", first_address.encode_hex());
}