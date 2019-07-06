#![feature(async_await, await_macro, futures_api)]
#![warn(clippy::pedantic, future_incompatible, unreachable_pub)]
#![allow(clippy::stutter, clippy::new_ret_no_self, clippy::module_inception)]

use once_cell::{sync::Lazy, sync_lazy};
use parking_lot::Mutex;
use tokio::runtime::Runtime;

use failure::{format_err, Error};
use futures::FutureExt;
use std::{env, thread::sleep, time::Duration};
use std::str::FromStr;
use tokio::{await, run_async};
use std::io::prelude::*;
use std::fs::File;
use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

#[macro_use]
mod macros;

mod claim;
pub mod client;
mod crypto;
mod duration;
mod entity;
mod error;
mod id;
mod info;
mod proto;
pub mod query;
mod status;
mod timestamp;
pub mod transaction;
mod transaction_id;
mod transaction_receipt;
mod transaction_record;

pub use self::{
    claim::Claim,
    client::Client,
    crypto::{PublicKey, SecretKey, Signature},
    entity::Entity,
    error::ErrorKind,
    id::*,
    info::{AccountInfo, ContractInfo, FileInfo},
    status::Status,
    transaction_id::TransactionId,
    transaction_receipt::TransactionReceipt,
    transaction_record::{TransactionRecord, TransactionRecordBody},
};


fn getClient() -> Result<Client, Error> {
    //let my_account: String = "0.0.1011".parse()?;
    let my_account: String = "0.0.1210".parse()?;
    let operator = my_account.parse()?;//env::var("OPERATOR")?.parse()?;
    // let client = Client::builder("testnet.hedera.com:50126")
    //     .node("0:0:3".parse()?)
    //     .operator(operator, || "302e020100300506032b657004220420d575934fb51d62c9568541feb2c303848fd4fd47c1db3eaf8b03aab336d4c1fe")
    //     .build()?;
    let client = Client::builder("0.testnet.hedera.com:50211")
        .node("0:0:3".parse()?)
        .operator(operator, || "302e020100300506032b657004220420d575934fb51d62c9568541feb2c303848fd4fd47c1db3eaf8b03aab336d4c1fe")
        .build()?;

    Ok(client)
}

#[no_mangle]
pub extern fn balance()->u64 {
    let balance = api_balance().unwrap();
    return balance;
}

fn api_balance() -> Result<u64, Error> {
    println!("balance");
    let my_account: String = "0.0.1210".parse()?;
    let operator = my_account.parse()?;//env::var("OPERATOR")?.parse()?;
    
    // Get _just_ the balance for the account first
    // This costs 100,000 tinybar
    let client = getClient().unwrap();
    let balance = client.account(operator).balance().get()?;

    // Now actually get the full information for the account
    // This costs 100,000 tinybar

    //let info = client.account(operator).info().get()?;
    //println!("info = {:#?}", info);

    Ok(balance)
}


// #[no_mangle]
// pub extern fn callSmartContarctGet(/*funcParam : Vec<u8> ,*/
//             func:*const u8, len:isize, gas: i64 , amount: i64) -> *mut c_char/*u8 Vec<u8>*/{
//    ////////////// hard coded rmov latr take as inpt
//    let mut funcParam : Vec<u8> = Vec::new();

//     unsafe {
//         for cnt in 0..len {
//             //println!("{}" , *func.offset(cnt));
//             funcParam.push(*func.offset(cnt));  
//         } 
//     }



// //   funcParam.push(109);
//  //  funcParam.push(76);
//  //  funcParam.push(230); /// -119
//  //  funcParam.push(60);    
//    //////////////////
//    println!("calling callSmartContarctGet");
//    let dflt : u8 = 0;
//    //let rtVal = callSmartContarct(funcParam , gas , amount).unwrap()[31];
//     CString::new(callSmartContarct(funcParam , gas , amount).unwrap()).unwrap().into_raw()
//    //return rtVal;
// }



// #[no_mangle]
// pub extern fn callSmartContarctSet(/*funcParam : Vec<u8> ,*/func:*const u8, len:isize, gas: i64 , amount: i64) {
//    ////////////// hard coded rmov latr take as inpt

//    let mut funcParam : Vec<u8> = Vec::new();

// unsafe {
//     for cnt in 0..len {
//         //println!("{}" , *func.offset(cnt));
//         funcParam.push(*func.offset(cnt));  
//     } 
// }

// //    funcParam.push(96);
// //    funcParam.push(254);
// //    funcParam.push(71);
// //    funcParam.push(177);
// //    for cnt in 0..31 {
// //        funcParam.push(0);
// //    }
// //    funcParam.push(val);    
//    //////////////////
//    println!("calling callSmartContarctSet ");
//    callSmartContarct(funcParam , gas , amount);
// }

#[no_mangle]
pub extern fn callResult_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}


#[no_mangle]
pub extern fn callSmartContarctApi(func:*const u8 , len:isize,
                gas: i64 , amount: i64 , contractId:i64) ->  *mut c_char {
    let mut funcParam : Vec<u8> = Vec::new();

    unsafe {
        for cnt in 0..len {
            //println!("{}" , *func.offset(cnt));
            funcParam.push(*func.offset(cnt));  
        } 
    }
    //let default : Vec<u8> = Vec::new();
    //CString::new(callSmartContarct(funcParam , gas , amount).unwrap_or(default)).unwrap().into_raw()


    let mut default : Vec<u8> = Vec::new();
    let result = callSmartContarct(funcParam , gas , amount , contractId).unwrap_or(default);
    //let result = callSmartContarct(funcParam , gas , amount, contractId).unwrap();
    let mut funcCallResult : Vec<u8> = Vec::new();
    funcCallResult.push(48);
    funcCallResult.push(120);
    for cnt in 0..result.len(){
        let vl = result[cnt];
        let str_vl = format!("{:x}", vl);
        if(str_vl.len() > 1) {
            funcCallResult.push(str_vl.chars().nth(0).unwrap() as u8);
            funcCallResult.push(str_vl.chars().nth(1).unwrap() as u8);
        }
        else {
            funcCallResult.push(48);
            funcCallResult.push(str_vl.chars().nth(0).unwrap() as u8);
        }  
    }
    
    /*
    default.push(48);
    default.push(120);
    for cnt in 0..31 {
        //default.push(48);
        //default.push(48);
        let vl = 0;
        let str_vl = format!("{:x}", vl);
        if(str_vl.len() > 1) {
            default.push(str_vl.chars().nth(0).unwrap() as u8);
            default.push(str_vl.chars().nth(1).unwrap() as u8);
        }
        else {
            default.push(48);
            default.push(str_vl.chars().nth(0).unwrap() as u8);
        }

    }
    let vl = 32;
    let str_vl = format!("{:x}", vl);
    default.push(str_vl.chars().nth(0).unwrap() as u8);
    default.push(str_vl.chars().nth(1).unwrap() as u8);
    */

    CString::new(funcCallResult).unwrap().into_raw()


}

fn callSmartContarct(funcParam : Vec<u8> , gas: i64 , amount: i64 , contractId:i64) -> Result<Vec<u8>, Error> {
   let client = getClient().unwrap();

   let contid = ContractId::new(0 , 0 , contractId);    
   let cont_id = client
       .contract(contid)
       .call()
       .amount(amount)
       .gas(gas)
       .function_parameters(funcParam)
       .generate_record(true)
       .memo("[hedera-sdk-rust] call_contract")
       .execute()?;

   println!("calling contract; transaction = {}", cont_id);
//0x62738998
//
//0x655605800000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000005800000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000037472790000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b747279407472792e636f6d000000000000000000000000000000000000000000
   // If we got here we know we passed pre-check
   // Depending on your requirements that may be enough for some kinds of transactions
   sleep(Duration::from_secs(3));

   let cont_record = client.transaction(cont_id).record().get()?;
   //println!("calling contract; transaction = {}", cont_record);
   if cont_record.receipt.status != Status::Success {
       Err(format_err!(
           "transaction contract has a non-successful status: {:?}",
           cont_record.receipt.status
       ))?;
   }

   //println!("creating contract; transaction = {}", cont_record.transaction_hash);
   let recordBody = cont_record.body;            
   match recordBody {
       TransactionRecordBody::ContractCall(callResult) =>  {
        println!("creating contract body; transaction = {}", callResult.error_message);
           //let contactResult = String::from_utf8(callResult.contract_call_result);
        println!("creating contract body; transaction = {:?}", callResult.contract_call_result.get(31));
        let result = callResult.contract_call_result;
        return Ok(result);
    },
       _ => ()
   }
   let nonVc : Vec<u8> = Vec::new();    
   Ok(nonVc)
}

fn create_hedera_account() -> Result<(), Error> {
    //pretty_env_logger::try_init()?;

    let (secret, _) = SecretKey::generate("");
    let public = secret.public();

    println!("secret = {}", secret);
    println!("public = {}", public);

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = "0:0:2".parse()?;
    let client = Client::builder("testnet.hedera.com:50131")
        .node("0:0:3".parse()?)
        .operator(operator, || env::var("OPERATOR_SECRET"))
        .build()?;

    // Create our account
    let id = client
        .create_account()
        .key(public)
        .initial_balance(5_000_000)
        .memo("[hedera-sdk-rust][example] create_account")
        .execute()?;

    println!("created account; transaction = {}", id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions
    sleep(Duration::from_secs(2));

    // Get the receipt and check the status to prove it was successful
    //let receipt = await!(client.transaction(id).receipt().get_async())?;
    let record = client.transaction(id).record().get()?;
    if record.receipt.status != Status::Success {
        Err(format_err!(
            "transaction has a non-successful status: {:?}",
            record.receipt.status
        ))?;
    }

    // note: account can be [None] if the receipt wasn't for creating an account
    let account = record.receipt.account_id.unwrap();
    println!("account = {}", account);

    Ok(())
}

// Used to provide a blocking API for Query and Transaction execution
static RUNTIME: Lazy<Mutex<Runtime>> = sync_lazy! { Mutex::new(Runtime::new().unwrap()) };
