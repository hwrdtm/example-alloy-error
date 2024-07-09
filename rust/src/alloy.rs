//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

use std::str::FromStr;

use alloy::{contract::Error, network::EthereumWallet, node_bindings::Anvil, primitives::{Bytes, U256}, providers::ProviderBuilder, signers::local::PrivateKeySigner, sol, sol_types::SolInterface, transports::TransportError};
use eyre::Result;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Counter,
    "../contracts/out/Counter.sol/Counter.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().fork("https://eth.merkle.io").try_spawn()?;
    println!("Anvil running at `{}`", anvil.endpoint());

    // Set up signer from the first default Anvil account (Alice).
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer);

    // Create a provider with the wallet.
    let rpc_url = anvil.endpoint().parse()?;
    let provider =
        ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc_url);

    // Deploy the `Counter` contract.
    let contract = Counter::deploy(&provider).await?;

    println!("Deployed contract at address: {}", contract.address());

    //////////////////////////////////////////////////////////////
    // 1. Use `setNumber` with even number and it will succeed. //
    //////////////////////////////////////////////////////////////

    let builder = contract.setNumber(U256::from(42));
    let tx_hash = builder.send().await?.watch().await?;

    println!("Set number to 42: {tx_hash}");

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 2. Use `setNumber` with odd number and it will revert with a typed error - use old method to unpack the error. //
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let builder = contract.setNumber(U256::from(43));
    let tx_res = builder.send().await;

    if let Err(e) = tx_res {
        match e {
            Error::TransportError(TransportError::ErrorResp(err)) => {
                println!("Transport Error: {:?}", err);

                let data = err.data.unwrap_or_default();
                let data = data.get().trim_matches('"');
                let data = Bytes::from_str(data)?;
                let decoded_error = Counter::CounterErrors::abi_decode(&data, true)?;
                match decoded_error {
                    Counter::CounterErrors::InvalidNumber(d_err) => {
                        println!("Invalid Number Error - number:{:?} msg:{:?}", d_err.number, d_err.message);
                    }
                    Counter::CounterErrors::DummyError(_) => {}
                }
            }
            _ => {
                println!("Generic Error: {:?}", e)
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 3. Use `setNumberV2` with odd number and it will revert with a error string - use the old method to unpack the error. //
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let builder = contract.setNumberV2(U256::from(45));
    let tx_res = builder.send().await;

    if let Err(e) = tx_res {
        match e {
            Error::TransportError(TransportError::ErrorResp(err)) => {
                println!("Transport Error: {:?}", err);

                let data = err.data.unwrap_or_default();
                println!("Data: {:?}", data);
                let data = data.get().trim_matches('"');
                println!("Data: {:?}", data);
                let data = Bytes::from_str(data)?;
                println!("Data: {:?}", data);

                let decoded_error = Counter::CounterErrors::abi_decode(&data, true);
                // This will fail to decode since there is no typed error.
                assert!(decoded_error.is_err());
            }
            _ => {
                println!("Generic Error: {:?}", e)
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 4. Use `setNumber` with odd number and it will revert with a typed error - use new method to unpack the error. //
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // 5. Use `setNumber` with odd nubmer and it will revert with a error string - use the new method to unpack the error. //
    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


    Ok(())
}