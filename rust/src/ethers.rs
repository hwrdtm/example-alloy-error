use ethers::{
    contract::abigen,
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer}, types::U256,
};
use ethers::prelude::*;
use eyre::Result;
use std::{sync::Arc, time::Duration};

// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    Counter,
    "../contracts/out/Counter.sol/Counter.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Launch anvil
    let anvil = Anvil::new().spawn();

    println!("Anvil running at `{}`", anvil.endpoint());

    // Instantiate our wallet
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    // Connect to the network
    let provider =
        Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));

    // Instantiate the client with the wallet
    let client = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id())));

    // Deploy contract
    let counter_contract =
        Counter::deploy(client, ()).unwrap().send().await.unwrap();

    println!("Deployed contract at address: {}", counter_contract.address());

    //////////////////////////////////////////////////////////////
    // 1. Use `setNumber` with even number and it will succeed. //
    //////////////////////////////////////////////////////////////

    let tx = counter_contract.set_number(U256::from(42));
    let tx = tx.send().await?;
    println!("Set number to 42: {:?}", tx.tx_hash());

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 2. Use `setNumber` with odd number and it will revert with a typed error - use old method to unpack the error. //
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let tx = counter_contract.set_number(U256::from(43));
    if let Err(e) = tx.send().await {

        match e.decode_contract_revert::<counter::CounterErrors>().expect("failed to decode error") {
            counter::CounterErrors::InvalidNumber(e) => {
                println!("InvalidNumber Error: {:?}", e);
            },
            counter::CounterErrors::DummyError(e) => {
                println!("DummyError Error: {:?}", e);
            }
            counter::CounterErrors::RevertString(e) => {
                println!("Revert String Error: {:?}", e);
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 3. Use `setNumberV2` with odd number and it will revert with a error string - use the old method to unpack the error. //
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let tx = counter_contract.set_number_v2(U256::from(45));
    if let Err(e) = tx.send().await {

        match e.decode_contract_revert::<counter::CounterErrors>().expect("failed to decode error") {
            counter::CounterErrors::InvalidNumber(e) => {
                println!("InvalidNumber Error: {:?}", e);
            },
            counter::CounterErrors::DummyError(e) => {
                println!("DummyError Error: {:?}", e);
            }
            counter::CounterErrors::RevertString(e) => {
                println!("Revert String Error: {:?}", e);
            }
        }
    }

    Ok(())
}