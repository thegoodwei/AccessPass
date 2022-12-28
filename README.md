# AccessPass, Smart Contract dApp in Rust, enabling premium videos with tokenized access keys


This dApp allows users to buy, sell, and play DRM encrypted videos with an access key embedded in semi-fungible tokens (NFTs). The NFTs are implemented as ERC1155 tokens on the Ethereum blockchain and integrates shared royalties with 0xSplit. The dApp is implemented in Rust and uses the Yew framework for the basic user interface provided.


## Summary

#### The AccessPass.rs Smart Contract is implemented on the Ethereum network using the ERC1155 standard, which allows for both fungible and non-fungible tokens in a single contract. The Dapp is built using Rust and Yew, and provides a user-friendly interface for interacting with the Smart Contract.

#### AccessPass is a smart contract dApp for creating and managing encrypted video tokens using the ERC1155 standard, which enables users to buy, sell, and play encrypted videos using semi-fungible NFTs as access keys. The dApp provides a user-friendly interface for interacting with the smart contract and decrypting and playing the video using the access key provided in the token.

#### The smart contract implements a royalty system, where a percentage of the sale price is transferred to the original owner of the token whenever it is resold. This allows creators to earn ongoing revenue from their videos. The royalty fee and the NFT type are configurable constants in the contract. One key feature is the integration with 0xSplits.xyz, which allows the creators of the NFTs to receive royalties from the resale of their NFTs, and to share those royalties with the early investors and subscribers. 

#### The smart contract and dApp are implemented in Rust and use the WebAssembly (WASM) runtime for efficient execution on the Ethereum network. The dApp is built using the Yew framework for building web-based applications with Rust.

#### The encrypted video is stored on a decentralized storage platform, such as Arweave, a decentralized storage platform that allows for the permanent and secure storage of the encrypted videos. This ensures the long-term availability of the content and protects against censorship and data loss. This integration is to ensure its availability and durability. The access key is stored in the smart contract and is only accessible to the owner of the token. The video is encrypted using the Widevine DRM system to ensure its security and protect the copyright of the creator.

#### In summary, our Smart Contract and Dapp offer a scalable and secure solution forcreating and managing encrypted video tokens on the Ethereum network. It allows creators and their early investors to earn ongoing revenue from the distribution and monetization of encrypted videos, leveraging the benefits of blockchain technology and decentralized storage. This enables a new revenue stream for content creators and promotes the sustainability of the ecosystem. 


---------------------------------------------------------------------------------------------------------------------

AccessPass is a smart contract and dApp for creating and managing encrypted video tokens using the ERC1155 standard. The smart contract allows users to buy and sell semi-fungible tokens that grant access to an encrypted video. The dApp provides a user-friendly interface for interacting with the smart contract and decrypting and playing the video using the access key provided in the token.

### Motivation

The use of decentralized applications (dApps) on the Ethereum network is growing rapidly, as it allows for the creation of decentralized, trustless, and transparent systems for a wide range of applications. One of the areas where dApps can have a significant impact is the media industry, by providing a platform for creators to monetize their content and for users to access high-quality content without intermediaries.

However, the distribution of media content on the Ethereum network poses several challenges, including security, scalability, and copyright protection. Encrypting the content and storing it on a decentralized storage platform can address the security and scalability issues, but it does not solve the problem of copyright protection. The use of a smart contract and non-fungible tokens (NFTs) can provide a solution by creating a system for managing the access and ownership of the content.

We hope that this solution will contribute to the growth and adoption of dApps in the media industry, removing bias from content publishing incentives to enable a new renaissance of independent creators.

### Solution

We have created a smart contract and dApp for creating and managing encrypted video tokens using the ERC1155 standard. The smart contract allows users to buy and sell semi-fungible tokens that grant access to an encrypted video, and it implements a royalty system for the creators of the video. The dApp provides a user-friendly interface for interacting with the smart contract and decrypting and playing the video using the access key provided in the token.

Our solution addresses the challenges of distributing media content on the Ethereum network by providing a scalable, secure, and transparent platform for creators to monetize their content and for users to access high-quality content with the assurance of copyright protection.

We propose a smart contract and dApp that addresses the challenges of distributing media content on the Ethereum network by implementing the following features:

-    Encrypted video: The video is encrypted using a symmetric key and the Widevine DRM system to ensure its security and protect the copyright of the creator. The encrypted video is stored on a decentralized storage platform, such as Arweave, to ensure its availability and durability.

-    ERC1155 tokens: The smart contract implements the ERC1155 standard, which allows for the creation of both fungible and non-fungible tokens in a single contract. We use non-fungible tokens to represent the access keys to the encrypted video. The tokens are semi-fungible, as they can be bought and sold, but the access key is private to the owner of the token.

-    Royalties: The smart contract implements a royalty system, where a percentage of the sale price is transferred to the original owner of the token whenever it is resold. This allows creators to earn ongoing revenue from their videos. The royalty fee and the NFT type are configurable constants in the contract.

-    User interface: The dApp provides a user-friendly interface for interacting with the smart contract and decrypting and playing the video using the access key provided in the token. The dApp is built using the Yew framework for building web-based applications with Rust, and it is compiled to WebAssembly (WASM) for efficient execution on the Ethereum network.

### Implementation

The smart contract is implemented in Rust, a programming language known for its performance, safety, and concurrency support. Rust is well-suited for implementing smart contracts on the Ethereum network, as it allows for the development of high-quality and reliable code.

The smart contract uses the ERC1155 standard from the OpenZeppelin library, which provides a solid and secure foundation for implementing multi-token contracts. The smart contract also uses the MPL library for implementing the royalty system, which allows for the distribution of a percentage of the sale price to the specified beneficiaries.

The dApp is implemented in Rust using the Yew framework, which provides a high-level and ergonomic API for building web-based applications. The dApp is compiled to WASM using the wasm-pack tool, which allows for the deployment of the dApp on the Ethereum network and the interaction with the smart contract using the Web3 API.
### Deployment

To deploy the smart contract and the dApp, follow these steps:

 1.   Install the required dependencies:

        Rust compiler: https://www.rust-lang.org/tools/install
    
        Rust package manager (Cargo): https://doc.rust-lang.org/cargo/getting-started/installation.html
        
        Rust WebAssembly compiler (wasm-pack): https://rustwasm.github.io/wasm-pack/installer/
        
        Ethereum wallet: https://www.ethereum.org/greeter
    
        Web3 provider: https://web3js.readthedocs.io/en/v1.2.11/getting-started.html#installing-web3
    
        Arweave client: https://www.arweave.org/developers/tools/arweave-cli


  2.  Clone this repository and navigate to the root directory:

``
    git clone https://github.com/thegoodwei/AccessPass
    cd encrypted-video-tokens
``

3. Build the smart contract and the dApp:

``
wasm-pack build
``

 - This will generate the WASM bytecode and the ABI for the smart contract and the dApp.

4. Deploy the smart contract to the Ethereum network:

 - Use your Ethereum wallet to create a new account or import an existing one.
 - Use the Web3 provider to send a transaction to the Ethereum network, deploying the smart contract and specifying the required parameters:

    - `thumbnail`: The thumbnail image for the video, encoded as a base64 string.
    - `royalty_percentage`: The percentage of the sale price to be transferred as royalties.
    - `nft_type`: The type of the NFT, represented as a bytes32 string.

5. Deploy the dApp to a web server or a decentralized storage platform, such as IPFS or Arweave.

6. Use the dApp to interact with the smart contract and manage the encrypted video tokens.

---------------------------------------------------------------------------------------------------------------------

## Usage Guide

### To use the dApp, you will need the following:

 -   A compatible web browser that supports Rust and WebAssembly.
 -   An Ethereum wallet and a connection to the Ethereum network.


To use the AccessPass dApp and smart contract to deploy an encrypted video with an NFT of movieThumbnail.jpg as the key for an encrypted newMovie.mp4:

1.    Install the required dependencies:

        Rust compiler: https://www.rust-lang.org/tools/install
        Rust package manager (Cargo): https://doc.rust-lang.org/cargo/getting-started/installation.html
        Rust WebAssembly compiler (wasm-pack): https://rustwasm.github.io/wasm-pack/installer/
        Ethereum wallet: https://www.ethereum.org/greeter
        Web3 provider: https://web3js.readthedocs.io/en/v1.2.11/getting-started.html#installing-web3
        Arweave client: https://www.arweave.org/developers/tools/arweave-cli


2.  Clone this repository and navigate to the root directory
 
 
``
git clone https://github.com/<github.com/thegoodwei/AccessPass>
cd encrypted-video-nft
``
  
  
3. Compile the smart contract and the dApp:

``
wasm-pack build --target web contract
wasm-pack build --target no-modules --out-dir pkg ui
``

4. Deploy the smart contract to the Ethereum network using a deployer. You can use the deployer provided in the deployer directory:

``
cargo run --release -- --web3 <web3-provider-url> --eth-key <ethereum-wallet-private-key> --eth-account <ethereum-wallet-address> --video <path-to-newMovie.mp4> --access-key <access-key> --nft-type <nft-type> --royalty-fee <royalty-fee>
``

5. Replace the placeholders with your own values. The access-key should be a 32-byte hexadecimal string. The nft-type and royalty-fee should be integers.

6. Host the compiled dApp on a web server. You can use the pkg directory generated in step 3 as the root directory of the web server.

7. Access the dApp using a compatible web browser and connect to the Ethereum network using a web3 provider and your Ethereum wallet.

8. Use the dApp to buy an NFT with movieThumbnail.jpg as the thumbnail. The NFT will grant access to the encrypted newMovie.mp4.

To store the encrypted newMovie.mp4 on Arweave:

1.    Use the Arweave client to create a wallet:

``
arweave wallet create
``

- This will generate a wallet file and a wallet key file.

2. Use the Arweave client to transfer AR tokens to your wallet:

``
        arweave transfer <amount> --to <wallet-address> --key-file <path-to-wallet-key-file>
``

        Replace the placeholders with your own values. The amount should be specified in AR.
        Use the Arweave client to upload the encrypted newMovie.mp4 to Arweave:
        
3.       Store the encrypted newMovie.mp4 on Arweave by using the Arweave client to create a wallet:

``
arweave wallet create
``

- This will generate a wallet file and a wallet key file.

4.  Use the Arweave client to transfer AR tokens to your wallet:

``
arweave transfer <amount> --to <wallet-address> --key-file <path-to-wallet-key-file>
``

5.  Replace the placeholders with your own values. The amount should be specified in AR.
Use the Arweave client to upload the encrypted newMovie.mp4 to Arweave:

``
arweave upload --key-file <path-to-wallet-key-file> --file <path-to-encrypted-newMovie.mp4>
``

- This will generate a transaction ID for the upload.

6.  Use the Arweave client to verify the upload:

``
    arweave get-transaction <transaction-id>
``

    This will show the details of the transaction, including the status and the file size.

- You can now use the dApp to decrypt and play the encrypted newMovie.mp4 using the access key provided in the NFT. You can also sell the NFT to other users and claim the resale royalties.


To download the encrypted newMovie.mp4 from Arweave, use the following command:

``
    arweave download --transaction <transaction-id> --output <output-file-name>
``

-    Replace the placeholders with your own values. The output-file-name should be the name of the file you want to save the downloaded content to.

- You should have successfully deployed an encrypted video with an NFT as the key and stored the encrypted video on Arweave.

## To use 0xSplits.xyz to share the royalty

 1.   Connect your Ethereum wallet to the https://0xsplits.xyz/

2.    Click the "Add Contract" button on the top menu.

3.    Enter the address of your smart contract in the "Contract Address" field.

4.    Enter the name of your contract in the "Contract Name" field.

5.    Select the "ERC1155" option in the "Contract Type" dropdown.

6.    Click the "Add Contract" button.    You should now see your contract listed on the "Contracts" page.

7.    Click the "Add Beneficiary" button next to your contract.

8.   Enter the address of the beneficiary in the "Beneficiary Address" field. This should be the address of the Ethereum wallet where you want to receive the royalties.

9.    Enter the percentage of the royalties in the "Split Percentage" field.

10.    Click the "Add Beneficiary" button.   You should now see the beneficiary listed under your contract.

-     Whenever someone buys or sells an NFT from your contract, 0xSplits will automatically split the royalties according to the specified percentages and send them to the specified beneficiaries.

-  You can view the details of the royalties and the transactions on the "Transactions" page.


---------------------------------------------------------------------------------------------------------------------


# Documentation

## Smart Contract

### Features
The smart contract for the dApp is an ERC1155 contract that allows for the creation and transfer of NFTs that grant access to encrypted videos. The contract has the following features:

1.    NFTs are semi-fungible: each NFT has a unique ID and can only be used to access a specific encrypted video, but NFTs with the same ID can be exchanged without restrictions.

2.    Resale royalties: when an NFT is sold, the seller is entitled to a percentage of the sale price as a royalty. The royalty fee is set in the contract and is applied to all NFT sales.

3.    Access keys: the access key to decrypt the video is private to the NFT owner and is not stored on the blockchain.

### Functions

The smart contract has the following functions:

1.    on_erc1155_batch_received(from: Address, ids: Vec<U256>, tokens: Vec<H256>) -> Result<(), String>: This function is called when a batch of NFTs is received by the contract. It adds the NFTs to the contract's state.

2.    safeBatchTransferFrom(from: Address, to: Address, ids: Vec<U256>, tokens: Vec<H256>) -> Result<(), String>: This function transfers a batch of NFTs from one owner to another.

3.   getAccessKey(id: U256) -> Option<Vec<u8>>: This function returns the access key for an NFT, if the caller is the owner of the NFT.

4.    getRoyaltyFee() -> U256: This function returns the royalty fee for NFT sales.

5.    setRoyaltyFee(fee: U256) -> Result<(), String>: This function sets the royalty fee for NFT sales.

### Events

The smart contract has the following events:

-    BatchReceived(Address, Vec<U256>, Vec<H256>): This event is emitted when a batch of NFTs is received by the contract.
-    BatchTransferred(Address, Address, Vec<U256>, Vec<H256>): This event is emitted when a batch of NFTs is transferred from one owner to another.

## DApp

The dApp is a Rust application that provides the logic for interacting with the smart contract and the user interface. It includes the following components:

-    VideoPlayer: This struct represents a video player that can decrypt and play an encrypted video using an access key.
-    EncryptedVideoNFTContract: This struct represents an instance of the smart contract and provides functions for interacting with the contract.

### Functions

The dApp has the following functions:

-    `EncryptedVideoNFTContract::new(web3:

web3: &Web3, contract_address: Address) -> Self: This function creates a new instance of the EncryptedVideoNFTContract` struct and returns it. The function takes a web3 instance and the address of the contract as arguments.

-    EncryptedVideoNFTContract::deploy(web3: &Web3, sender: Address, video: Vec<u8>, access_key: Vec<u8>, nft_type: U256, royalty_fee: U256) -> Result<Address, String>: This function deploys the smart contract to the Ethereum network. It takes the web3 instance, the sender's address, the encrypted video, the access key, the NFT type, and the royalty fee as arguments. The function returns the address of the deployed contract.

-    EncryptedVideoNFTContract::on_erc1155_batch_received(from: Address, ids: Vec<U256>, tokens: Vec<H256>) -> Result<(), String>: This function is a wrapper around the on_erc1155_batch_received function of the smart contract. It calls the function on the contract and returns the result.

-    EncryptedVideoNFTContract::safe_batch_transfer_from(from: Address, to: Address, ids: Vec<U256>, tokens: Vec<H256>) -> Result<(), String>: This function is a wrapper around the safeBatchTransferFrom function of the smart contract. It calls the function on the contract and returns the result.

-    EncryptedVideoNFTContract::get_access_key(id: U256) -> Option<Vec<u8>>: This function is a wrapper around the getAccessKey function of the smart contract. It calls the function on the contract and returns the result.

##User Interface

The user interface is a web-based application built using the Yew framework. It provides UI elements for interacting with the dApp, including text input, buttons, and a video player.
Components

The user interface has the following components:

-    App: This struct represents the top-level component of the user interface. It contains the state of the interface, including the contract instance and the video player.

###Functions

The user interface has the following functions:

-    App::new() -> Self: This function creates a new instance of the App struct and returns it. The function initializes the contract and video player, and sets the NFT type and royalty fee constants in the contract.

-    App::update(&mut self, _msg: Self::Message) -> ShouldRender: This function updates the state of the app based on the received message.

-    App::view(&self) -> Html: This function renders the user interface. It includes UI elements such as text input, buttons, and a video player.

### Messages

The user interface has the following messages:

1.    Buy: This message is sent when the user clicks the "Buy" button. It triggers the process of buying an NFT.

2.    Sell: This message is sent when the user clicks the "Sell" button. It triggers the process of selling an NFT and claiming the resale royalties.

3.    Play: This message is sent when the user clicks the "Play" button. It triggers the process of decrypting and playing the video using the access key.

### Deployment

To deploy the dApp, you will need to perform the following steps:

1.    Compile the smart contract and the dApp using a Rust compiler and the relevant dependencies.

2.    Deploy the compiled smart contract to the Ethereum network using a deployer, such as the one provided in the deployer directory.

3.    Host the compiled dApp on a web server.

4.   Access the dApp using a compatible web browser and connect to the Ethereum network using a web3 provider and your Ethereum wallet.

#### License

This dApp is licensed under the MIT license.
