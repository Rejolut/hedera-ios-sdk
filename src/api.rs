use failure::Error;
use hedera::Client;



#[no_mangle]
pub extern fn balance() {
    let balance = api_balance().unwrap();
    //return balance;     
}

fn api_balance() -> Result<u64, Error> {
    println!("balance");
    let my_account: String = "0.0.1011".parse()?;
    let operator = my_account.parse()?;//env::var("OPERATOR")?.parse()?;
    let client = Client::builder("testnet.hedera.com:50126")
        .node("0:0:3".parse()?)
        .operator(operator, || "302e020100300506032b657004220420d575934fb51d62c9568541feb2c303848fd4fd47c1db3eaf8b03aab336d4c1fe")
        .build()?;
    
    // Get _just_ the balance for the account first
    // This costs 100,000 tinybar

    let balance = client.account(operator).balance().get()?;
    
    // Now actually get the full information for the account
    // This costs 100,000 tinybar

    //let info = client.account(operator).info().get()?;
    //println!("info = {:#?}", info);

    Ok(balance)
}


