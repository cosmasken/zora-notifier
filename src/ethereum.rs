use ethers::prelude::*;
use ethers::types::{Address, U256};
use std::sync::Arc;
use std::env;

#[derive(Debug)]
pub struct CoinParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub payout_recipient: Address,
    pub platform_referrer: Option<Address>,
    pub initial_purchase_wei: U256,
}

pub async fn create_coin(params: CoinParams) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
    // Load RPC URL and private key from environment variables
    let rpc_url = env::var("ETHEREUM_RPC_URL").expect("ETHEREUM_RPC_URL must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    // Create a wallet and provider
    let wallet: LocalWallet = private_key.parse()?;
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // Define the contract ABI and bytecode (replace with the actual ABI and bytecode for Zora's coin contract)
    let abi = include_str!("../abi/coin_abi.json");
    let bytecode = include_str!("../bytecode/coin_bytecode.txt");

    // Create a factory for deploying the contract
    let factory = ContractFactory::new(abi.parse()?, bytecode.parse()?, client);

    // Deploy the contract with the provided parameters
    let deploy_tx = factory.deploy((
        params.name,
        params.symbol,
        params.uri,
        params.payout_recipient,
        params.platform_referrer.unwrap_or(Address::zero()),
        params.initial_purchase_wei,
    ))?;

    let contract = deploy_tx.send().await?;
    let receipt = contract.await?;

    Ok(receipt)
}