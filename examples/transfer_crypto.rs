#![feature(async_await, futures_api, await_macro)]
use failure::{format_err, Error};
use futures::FutureExt;
use hedera::{AccountId, Client, Status};
use std::{env, thread::sleep, time::Duration};
use tokio::{await, run_async};

async fn main_() -> Result<(), Error> {
    pretty_env_logger::try_init()?;

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = env::var("OPERATOR")?.parse()?;
    let client = Client::builder("testnet.hedera.com:50131")
        .node("0:0:3".parse()?)
        .operator(operator, || env::var("OPERATOR_SECRET"))
        .build()?;

    // Receiver is the account that receives the transferred crypto
    let receiver: AccountId = "0:0:2".parse()?;

    // transfer 1 hbar from the operator account to the receiver account.
    let id = await!(client
        .transfer_crypto()
        .transfer(operator, -1_000_000)
        .transfer(receiver, 1_000_000)
        .memo("[hedera-sdk-rust][example] transfer_crypto")
        .sign(&env::var("OPERATOR_SECRET")?.parse()?)
        .sign(&env::var("OPERATOR_SECRET")?.parse()?)
        .execute_async())?;

    println!("created transfer; transaction = {}", id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions
    sleep(Duration::from_secs(5));

    // Get the receipt and check the status to prove it was successful
    let receipt = await!(client.transaction(id).receipt().get_async())?;

    if receipt.status != Status::Success {
        Err(format_err!(
            "transaction has a non-successful status: {:?}",
            receipt.status
        ))?;
    }

    Ok(())
}

fn main() {
    run_async(main_().map(|res| match res {
        Ok(_) => {}
        Err(err) => eprintln!("error: {}", err),
    }))
}
