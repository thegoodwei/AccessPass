# AccessPass
DRM encrypted videos, with erc1155 tokenized license keys and shared royalty. 


Introduction
This dApp is a decentralized application that allows users to buy, sell, and play encrypted videos using non-fungible tokens (NFTs). The dApp uses a smart contract implemented on the Ethereum blockchain to store and manage the NFTs. The smart contract implements the ERC1155 standard, which allows for the creation and management of semi-fungible and non-fungible tokens in a single contract.

The dApp includes the following features:

Encrypted videos: The videos are encrypted using symmetric key encryption, and the access key is stored in the NFTs.
Resale royalties: When an NFT is sold, a percentage of the sale price is paid as a royalty fee to the original owner of the NFT.
Semi-fungibility: NFTs can be transferred between users, but each NFT is unique and cannot be replaced or divided.
Requirements
To use the dApp, you will need the following:

A web3 provider, such as an Ethereum node running on your local machine or a cloud-based provider like Infura.
An Ethereum wallet with a funded account, such as MetaMask.
A compatible web browser that supports web3 and can communicate with the web3 provider.
Usage
To use the dApp, follow these steps:

Connect to the Ethereum network using a web3 provider and your Ethereum wallet.
Deploy the smart contract to the Ethereum network.
Set the NFT type and royalty fee constants in the smart contract.
Create a batch of NFTs and add them to the contract's state.
Retrieve the access key for an NFT.

Use the access key to decrypt and play the video.
If you want to sell the NFT, call the payoutRoyalty function of the smart contract to claim the royalty fee that is due to you.
If you want to transfer the NFT to another user, call the transfer_from function of the smart contract and provide the address of the new owner, the ID of the NFT, and the token ID of the NFT.
API Reference
The dApp provides the following functions and types:

Types
EncryptedVideoNFTContract: This struct represents an instance of the smart contract.
VideoPlayer: This struct represents the video player that is used to play the encrypted videos.
Functions
EncryptedVideoNFTContract::new(web3: &web3::Web3<web3::transports::Http>, contract_address: Address) -> Self: This function creates a new instance of the EncryptedVideoNFTContract struct and returns it.
EncryptedVideoNFTContract::get_access_key(&self, id: U256, token_id: H256) -> impl Future<Item = Vec<u8>, Error = String>: This function calls the getAccessKey function of the smart contract to retrieve the access key for an NFT.
EncryptedVideoNFTContract::payout_royalty(&self, id: U256, token_id: H256, value: U256) -> impl Future<Item = H256, Error = String>: This function calls the payoutRoyalty function of the smart contract to claim the royalty fee for an NFT that has been sold.
`EncryptedVideoNFTContract::transfer_from(&self, from: Address, to: Address, ids: Vec<U256>,

tokens: Vec<H256>) -> impl Future<Item = H256, Error = String>: This function calls the safeBatchTransferFrom` function of the smart contract to transfer NFTs from one owner to another.

VideoPlayer::new() -> Self: This function creates a new instance of the VideoPlayer struct and returns it.
VideoPlayer::play_video(&self, access_key: Vec<u8>): This function uses the provided access key to decrypt and play the video.
Usage
To use the user interface, follow these steps:

Connect to the Ethereum network using a web3 provider and your Ethereum wallet.
Deploy the smart contract to the Ethereum network.
Set the NFT type and royalty fee constants in the smart contract.
Create a batch of NFTs and add them to the contract's state.
Use the user interface to buy an NFT.
Use the user interface to retrieve the access key for the NFT.
Use the user interface to decrypt and play the video using the
If you want to sell the NFT, use the user interface to claim the resale royalties due to you.
If you want to transfer the NFT to another user, use the user interface to transfer the NFT to the new owner.


Here is an example of how to use the user interface to buy an NFT and play an encrypted video:

Connect to the Ethereum network using a web3 provider and your Ethereum wallet.
Deploy the smart contract to the Ethereum network.
Set the NFT type and royalty fee constants in the smart contract.
Create a batch of NFTs and add them to the contract's state.
Use the user interface to buy an NFT.
Use the user interface to retrieve the access key for the NFT.
Use the user interface to decrypt and play the video using the access key.

License
Everything is licensed under the MIT license. See the LICENSE file for details.
