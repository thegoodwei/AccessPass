/*This deployer.rs file is a Rust program that can be used to deploy an ERC1155 smart contract that holds the private encryption key 
for an encrypted video file stored on Arweave. The deployer reads an unencrypted video file and a thumbnail image file, encrypts the 
video file using Widevine, stores the encrypted video on Arweave, and creates a specified number of tokens with the 
private encryption key and transaction ID of the encrypted video stored on the smart contract. 

The deployer requires several command line arguments, including the number of tokens to create, the paths to the thumbnail and
video files, the percentage of the token sale price to be paid as a royalty, the Ethereum address of the royalty beneficiary, 
and the URLs of the Ethereum JSON-RPC endpoint, Ethereum wallet, and Arweave node, as well as the base64-encoded Arweave key file.
Upon successful execution, the deployer returns a list of transaction hashes for the token creation transactions
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
extern crate rust_widevine;

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
use rust_widevine::{KeyType, LicenseRequest, Widevine};

#[wasm_bindgen]
// Define the deployer function
pub fn deploy(
    x_qty: i32,
    thumbnail_b64: &str,
    video_b64: &str,
    royalty_percentage: i32,
    royalty_beneficiary: &str,
    rpc_url: &str,
    wallet_url: &str,
    arweave_url: &str,
    arweave_key: &str,
) -> Promise {
    // Convert the command line arguments to the required types
    let x_qty = U256::from(x_qty as u64);
    let thumbnail_b64 = thumbnail_b64.to_string();
    let video_b64 = video_b64.to_string();
    let royalty_percentage = U256::from(royalty_percentage as u64);
    let royalty_beneficiary = Address::from_str(royalty_beneficiary).unwrap();
    let rpc_url = rpc_url.to_string();
let wallet_url = wallet_url.to_string();
let arweave_url = arweave_url.to_string();
let arweave_key = arweave_key.to_string();

// Read the unencrypted video file
let mut video_file = File::open(&video_b64).unwrap();
let mut video_bytes = Vec::new();
video_file.read_to_end(&mut video_bytes).unwrap();

// Encrypt the video file using Widevine
let widevine = Widevine::new("/path/to/widevine/keys");
let key_type = KeyType::ContentId;
let request = LicenseRequest::new(
    &video_bytes,
    "video/mp4",
    key_type,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
);
let response = widevine.request_license(request).unwrap();
let encrypted_video_bytes = response.ciphertext.unwrap();
let key_id = hex::encode(response.key_id.unwrap());

// Read the thumbnail image file and encode it as base64
let mut thumbnail_file = File::open(&thumbnail_b64).unwrap();
let mut thumbnail_bytes = Vec::new();
thumbnail_file.read_to_end(&mut thumbnail_bytes).unwrap();
let thumbnail_b64 = encode(&thumbnail_bytes);

// Store the encrypted video on Arweave
let video_b64 = encode(&encrypted_video_bytes);
let transaction = Transaction::new(arweave_key.as_bytes());
transaction.add_tag("Content-Type", "video/mp4");
transaction.add_tag("Encryption", "Widevine");
transaction.add_tag("Content-Length", &encrypted_video_bytes.len().to_string());
transaction.add_tag("Key-Id", &key_id);
transaction.add_tag("Filename", "encrypted-video.mp4");
transaction.set_data(video_b64.as_bytes());
let transaction_id = transaction.send(arweave_url.as_str()).unwrap();

// Create a web3 instance and connect to the Ethereum network
let (_eloop, transport) = web3::transports::Http::new(&rpc_url).unwrap();
let web3 = web3::Web3::new(transport);

// Create a web3 contract instance and deploy the smart contract
let contract = EthContract::from_json(
    web3.eth(),
    Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
    include_bytes!("../res/EncryptedVideoToken.json"),
).unwrap();
let options = Options::with(|opt| {
opt.gas = Some(3_000_000.into());
opt.value = Some(0.into());
});
let deploy = contract.deploy(
options,
thumbnail_b64,
royalty_percentage,
royalty_beneficiary,
key_id,
b"encrypted-video-token".to_vec(),
);
let address = deploy.send().wait().unwrap();
let contract = Contract::from_ethcontract(
web3.eth(),
address,
include_bytes!("../res/EncryptedVideoToken.json"),
)
.unwrap();

// Create the specified number of tokens using the smart contract
let mut promises = Vec::new();
for i in 0..x_qty {
    let call = contract.call(
        "create",
        (i, thumbnail_b64.clone(), transaction_id.clone()),
        options,
        wallet_url.as_str(),
    );
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
}
    
