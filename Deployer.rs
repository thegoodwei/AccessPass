/*
This deployer can be compiled to WASM using the `wasm-pack` tool and deployed to a web server or a decentralized storage platform, such as IPFS or Arweave. The deployer can then be invoked from a JavaScript application or a web page, passing the required arguments and receiving the transaction hashes of the
*/

// Import the required crates and libraries
extern crate web3;
extern crate ethcontract;
extern crate hex;
extern crate arweave;
extern crate base64;
extern crate image;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate serde_json;
extern crate futures;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::futures::Future;
use web3::types::{Address, H256, U256};
use ethcontract::{Contract as EthContract, Eth};
use arweave::Transaction;
use base64::encode;
use image::{ImageFormat, ImageRgba8};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use serde_json::Value;

#[wasm_bindgen]
// Define the deployer function
pub fn deploy(
    x_qty: i32,
    thumbnail_b64: &str,
    video_b64: &str,
    rpc_url: &str,
    wallet_url: &str,
    arweave_url: &str,
    arweave_key: &str,
) -> Promise {
    // Convert the command line arguments to the required types
    let x_qty = U256::from(x_qty as u64);
    let thumbnail_b64 = thumbnail_b64.to_string();
    let video_b64 = video_b64.to_string();
    let rpc_url = rpc_url.to_string();
    let wallet_url = wallet_url.to_string();
    let arweave_url = arweave_url.to_string();
    let arweave_key = arweave_key.to_string();

    // Create a web3 instance and connect to the Ethereum network
    let (_eloop, transport) = web3::transports::Http::new(&rpc_url).unwrap();
    let web3 = web3::Web3::new(transport);

    // Create a web3 contract instance and deploy the smart contract
    let contract = EthContract::from_json(
        web3.eth(),
        Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
        include_bytes!("../res/EncryptedVideoToken.json"),
    )
    .unwrap();
    let options = Options::with(|opt| {
        opt.gas = Some(3_000_000.into());
        opt.value = Some(0.into());
    });
    let deploy = contract.deploy(options, thumbnail_b64, 10.into(), b"encrypted-video-token".to_vec());
let address = deploy.send().wait().unwrap();
let contract = Contract::from_ethcontract(web3.eth(), address, include_bytes!("../res/EncryptedVideoToken.json")).unwrap();


// Read the encrypted video file and store it on Arweave
let mut video_file = File::open(&video_b64).unwrap();
let mut video_bytes = Vec::new();
video_file.read_to_end(&mut video_bytes).unwrap();
let video_b64 = encode(&video_bytes);
let transaction = Transaction::new(arweave_key.as_bytes());
transaction.add_tag("Content-Type", "video/mp4");
transaction.add_tag("Encryption", "Widevine");
transaction.add_tag("Content-Length", &video_bytes.len().to_string());
transaction.add_tag("Access-Key", &address.to_string());
transaction.add_tag("Filename", "encrypted-video.mp4");
transaction.set_data(video_b64.as_bytes());
let transaction_id = transaction.send(arweave_url.as_str()).unwrap();

// Read the thumbnail image file and encode it as base64
let mut thumbnail_file = File::open(&thumbnail_b64).unwrap();
let mut thumbnail_bytes = Vec::new();
thumbnail_file.read_to_end(&mut thumbnail_bytes).unwrap();
let thumbnail_b64 = encode(&thumbnail_bytes);

// Create the specified number of tokens using the smart contract
let mut promises = Vec::new();
for i in 0..x_qty {
    let call = contract.call("create", (i, thumbnail_b64.clone(), transaction_id.clone()), options, wallet_url.as_str());
    let promise = future_to_promise(call.send().then(move |result| {
        let result = result.unwrap();
        let receipt = result.wait().unwrap();
        let transaction_hash = receipt.transaction_hash;
        JsValue::from_serde(&transaction_hash).unwrap()
    }));
    promises.push(promise);
}

// Return a Promise that resolves with the transaction hashes of the token creation transactions
future_to_promise(futures::future::join_all(promises).then(|results| {
    JsValue::from_serde(&results).unwrap()
}))

