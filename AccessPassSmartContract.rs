/*Overview:

This smart contract allows the creation and management of non-fungible tokens (NFTs) that hold the access key to an encrypted video. The NFTs are implemented using the ERC1155 standard, which allows for the creation and management of multiple types of NFTs within a single contract. The contract also uses the mpl-trifle library to provide safe and secure transfer functions for the NFTs.

The contract includes the following features:

The encryption key is private to the tokens, and is only accessible to the owner of the NFT.
The contract includes a royalty fee that is paid to the original owner of the NFT every time the NFT is sold.
The NFTs are semi-fungible, which means that each NFT is unique but can be exchanged for other NFTs of the same type.
Contract structure:

The contract consists of the following structs and functions:

EncryptedVideoNFT: This struct represents an NFT that holds the access key to an encrypted video. It includes the following fields:

id: A unique ID for the NFT.
owner: The address of the owner of the NFT.
video_url: The URL of the video that the NFT grants access to.
get_access_key: A function that retrieves the access key for the NFT from the Widevine key server, if the caller is the owner of the NFT.
EncryptedVideoNFTs: This struct stores the state of the contract, including a mapping from NFT IDs to EncryptedVideoNFT structs and a mapping from NFT IDs to ownership history records. The ownership history records store the original owner of each NFT.

`EncryptedVideoNFTContract




: This contract is a smart contract that implements the ERC1155, ERC1155Mutable, and SafeERC1155 traits, which define the required functions for an ERC1155 contract. The contract also includes the NFT_TYPE and ROYALTY_FEE constants, which define the ID of the NFT type and the percentage of the sale price that is paid as a royalty fee to the original owner of the NFT.

onERC1155BatchReceived: This function is called when a batch of NFTs is received by the contract. It creates new EncryptedVideoNFT structs for each NFT and stores them in the contract's state, as well as creating new ownership history records for each NFT.

getAccessKey: This function allows the owner of an NFT to retrieve the access key for the NFT. It calls the get_access_key function on the EncryptedVideoNFT struct for the specified NFT, and returns the access key as a bytes4 value if the caller is the owner of the NFT.

calculateRoyalty: This function calculates the royalty fee that is paid to the original owner of the NFT when the NFT is sold. It takes the sale price as an input and calculates the royalty fee as a percentage of the sale price, using the ROYALTY_FEE constant.

payoutRoyalty: This function allows the current owner of the NFT to claim the royalty fee that is due to the original owner of the NFT. It calculates the royalty fee using the calculateRoyalty function and transfers the fee to the original owner of the NFT. It also updates the ownership history for the NFT to reflect the new owner.

Usage:

To use this contract, an ERC1155 contract must be deployed and the NFT_TYPE and `ROYALTY_FEEconstants must be set to the desired values. Then, theonERC1155BatchReceived` function can be called to create new NFTs and add them to the contract's state.

The owner of an NFT can call the getAccessKey function to retrieve the access key for the NFT. This access key can then be used to decrypt and play the video.

If the NFT is sold, the new owner can call the payoutRoyalty function to claim the royalty fee that is due to the original owner of the NFT.

Note that this contract does not include functions for buying and selling NFTs. To facilitate the buying and selling of NFTs, an additional contract or external marketplace can be used.

Example:

// Deploy the ERC1155 contract
let contract_address = deploy_contract();

// Set the NFT type and royalty fee constants
contract_address.set_nft_type(0x01);
contract_address.set_royalty_fee(10);

// Create a batch of NFTs and add them to the contract's state
let ids = [0x01, 0x01, 0x01];
let tokens = [0x01, 0x02, 0x03];
contract_address.on_erc1155_batch_received(msg.sender, ids, tokens);

// Retrieve the access key for an NFT
let id = 0x01;
let token_id = 0x01;
let access_key = contract_address.get_access_key(id, token_id);

// Play the video using the access key
play_video(access_key);

// Sell the NFT and pay the royalty fee to the original owner
let value = 100;
contract_address.payout_royalty(id, token_id, value);

*/
use openzeppelin_sdk::{
    address,
    bytes,
    contracts::{self, ERC1155, ERC1155Mutable},
    prophy::{decode, encode},
};
use mpl_trifle::{
    contracts::SafeERC1155,
    types::{uint256, bytes32},
};
use rust_widevine::{key_server, licensing};

//The EncryptedVideoToken struct represents a smart contract that holds encrypted video tokens. It has fields for the total supply of tokens, a map of token balances for each owner, 
//a map of token ownership, a map of metadata for each token, and the royalty percentage and beneficiary address for resale royalties.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct EncryptedVideoToken {
    total_supply: U64,
    balance_of: BTreeMap<Address, U64>,
    owner_of: BTreeMap<U64, Address>,
    metadata: BTreeMap<U64, Bytes>,
    royalty_beneficiary: Address,
    royalty_percentage: U256,
    key_id: H256,

    // Define a mapping from NFT IDs to EncryptedVideoNFT structs
    mapping(bytes32 => EncryptedVideoNFT) public nfts;
    // Define a mapping from NFT IDs to ownership history records
    mapping(bytes32 => OwnableHistory) public history;
}

//The call function uses a match statement to determine which function to execute based on the variant of the Call enum passed 
//as an argument. The create, transfer, and safe_transfer functions all update the balance_of, owner_of, and metadata fields of the
//EncryptedVideoToken struct, while the get_access_key function retrieves the private encryption key for a token if the caller 
//is the owner of the token. The balance_of function returns the balance of the specified owner.

//The Call enum represents the various functions
//that can be called on the smart contract. It has variants for GetAccessKey, Create, Transfer, SafeTransfer, and BalanceOf.
#[derive(Clone, Debug, Deserialize, Serialize)]
enum Call {
    GetAccessKey,
    Create {
        thumbnail: Bytes,
        royalty_percentage: U256,
        royalty_beneficiary: Address,
        key_id: H256,
    },
    Transfer { id: U64, to: Address },
    SafeTransfer { id: U64, to: Address },
    BalanceOf { owner: Address },
}
/*
//depreciated?
struct EncryptedVideoNFT {
    id: bytes32,
    owner: address,
    video_url: String,
}*/
impl EncryptedVideoToken {
    
    
    
    fn create(&self, id: U64, qty:U64, thumbnail: Bytes, royalty_percentage: U256, royalty_beneficiary: Address, key_id: H256) -> EncryptedVideoToken {
        let metadata = json!({
            "thumbnail": thumbnail,
            "transaction_id": transaction_id
       //     ,   (1..=qty).map(|i| (i, thumbnail)).collect();
        }).to_string();
                let total_supply = qty;
        let total_supply = qty;
        let balance_of = BTreeMap::new();
        let owner_of = (1..=qty).map(|i| (i, msg.sender)).collect();

     //   let mut balance_of = self.balance_of.clone();
     //   let mut owner_of = self.owner_of.clone();
        let mut metadata_map = self.metadata.clone();
        let sender = self.get_sender();
        let new_balance = balance_of.entry(sender).or_insert(0.into());
        *new_balance += 1.into();
        owner_of.insert(id, sender);
        metadata_map.insert(id, metadata.into());
        EncryptedVideoToken {
            total_supply: self.total_supply + 1.into(),
            balance_of,
            owner_of,
            metadata: metadata_map,
            royalty_beneficiary: self.royalty_beneficiary,
            royalty_percentage: self.royalty_percentage,
            key_id: self.key_id,
        }
 fn call(&self, call: Call) -> EncryptedVideoToken {
        match call {
            Call::GetAccessKey => {
                if self.owner_of.get(&call.id)? == &self.get_sender() {
                    let key_id = self.key_id;
                    // implementation to retrieve the private encryption key goes here
                } else {
                    self.clone()
                }
            }
            Call::Create {
                qty,
                thumbnail,
                royalty_percentage,
                royalty_beneficiary,
                key_id,
            } => {
                let mut balance_of = self.balance_of.clone();
                let mut owner_of = self.owner_of.clone();
                let mut metadata = self.metadata.clone();
                let mut total_supply = self.total_supply;

                let new_tokens: Vec<U64> = (self.total_supply + 1..=self.total_supply + qty).collect();
                balance_of.insert(msg.sender, balance_of.get(&msg.sender).unwrap_or(&0.into()) + qty);
                for (i, token_id) in new_tokens.iter().enumerate() {
                    owner_of.insert(*token_id, msg.sender);
                    metadata.insert(*token_id, thumbnail.clone());
                    }
                let royalty_beneficiary = royalty_beneficiary;
                let royalty_percentage = royalty_percentage;
                let key_id = key_id;
                
                
                let sender = self.get_sender();
                total_supply += 1.into();
                let id = total_supply;
                let balance = balance_of.entry(sender).or_insert(0.into());
                *balance += 1.into();
                owner_of.insert(id, sender);
                metadata.insert(id, thumbnail);
                
                EncryptedVideoToken {
                    total_supply: self.total_supply + qty,
                    balance_of,
                    owner_of,
                    metadata,
                    royalty_beneficiary,
                    royalty_percentage,
                    key_id,
                }
            }
            Call::Transfer { id, to } => {
                let mut balance_of = self.balance_of.clone();
                let mut owner_of = self.owner_of.clone();
                let sender_balance = balance_of.get_mut(&self.get_sender())?;
                let recipient_balance = balance_of.entry(to).or_insert(0.into());
                *sender_balance -= 1.into();
                *recipient_balance += 1.into();
                owner_of.insert(id, to);
                EncryptedVideoToken {
                    total_supply: self.total_supply,
                    balance_of,
                    owner_of,
                    metadata: self.metadata.clone(),
                    royalty_beneficiary: self.royalty_beneficiary,
                    royalty_percentage: self.royalty_percentage,
                    key_id: self.key_id,
                }
            }            Call::SafeTransfer { id, to } => {
                if self.owner_of.get(&id)? == &self.get_sender() {
                    self.transfer(id, to)
                } else {
                    self.clone()
                }
            }
            Call::BalanceOf { owner } => {
                *self.balance_of.get(&owner).unwrap_or(&0.into())
            }
        }
    }



    }
        
        fn transfer(&self, id: U64, to: Address) -> EncryptedVideoToken {
        let mut balance_of = self.balance_of.clone();
        let mut owner_of = self.owner_of.clone();
        let from_balance = balance_of.get_mut(&self.get_sender()).unwrap();
        *from_balance -= 1.into();
        let to_balance = balance_of.entry(to).or_insert(0.into());
        *to_balance += 1.into();
        owner_of.insert(id, to);
        EncryptedVideoToken {
            total_supply: self.total_supply,
            balance_of,
            owner_of,
            metadata: self.metadata.clone(),
            royalty_beneficiary: self.royalty_beneficiary,
            royalty_percentage: self.royalty_percentage,
            key_id: self.key_id,
        }
        
        
        
        
        

    fn get_access_key(&self) -> Option<Vec<u8>> {
        if self.owner_of.get(&0)? == &self.get_sender() {
            Some(self.key_id.0.to_vec())
        } else {
            None
        }
    }
 fn safe_transfer(&self, id: U64, to: Address) -> EncryptedVideoToken {
        if self.owner_of.get(&id)? == &self.get_sender() {
            self.transfer(id, to)
        } else {
            self.clone()
        }
    }

    fn balance_of(&self, owner: Address) -> U64 {
        *self.balance_of.get(&owner).unwrap_or(&0.into())
    }

        fn get_sender(&self) -> Address {
        msg.sender
    }
}
    }
}


// Implement the ERC1155Mutable trait to allow the contract to manage multiple types of NFTs


contract EncryptedVideoNFTContract is ERC1155, ERC1155Mutable, SafeERC1155 {
    // Define the ID of the NFT type
    bytes32 public constant NFT_TYPE = 0x01;
    // Define the royalty fee as a percentage of the sale price
    uint256 public constant ROYALTY_FEE = 10;

    // Create a struct to store the state of the contract
    EncryptedVideoNFTs public nft_state;

    // Implement the required functions from the ERC1155Mutable trait
    function balanceOf(address owner, uint256 id) public view returns (uint) {
        // Return the number of NFTs of the specified type owned by the specified address
        return nft_balance_of(owner, id);
    }

    function ownerOf(uint256 id, bytes32 tokenId) public view returns (address) {
        // Return the owner of the specified NFT
        return nft_owner_of(id, tokenId);
    }

    function safeTransferFrom(address from, address to, uint256 id, bytes32 tokenId) public {
        // Transfer the specified NFT from the "from" address to the "to" address
        require(from == msg.sender, "Only the owner can transfer the NFT");
        nft_transfer_from(from, to, id, tokenId);
    }

    function onERC1155BatchReceived(address _from, uint256[] memory ids, bytes32[] memory _tokens) public payable {
        // Create new EncryptedVideoNFT structs and store them in the mapping
        for (uint i = 0; i < ids.length; i++) {
            // Check that the ID of the NFT type matches the expected value
            require(ids[i] == NFT_TYPE, "Invalid NFT type");
            // Create a new EncryptedVideoNFT struct and store it in the mapping
            let nft = EncryptedVideoNFT::new(_tokens[i], msg.sender, "https://example.com/video.mp4");
            nft_state.nfts[_tokens[i]] = nft;
            // Create a new ownership history record and store it in the mapping
            let history = OwnableHistory { owner: msg.sender };
            nft_state.history[_tokens[i]] = history;
        }
    }

    function getAccessKey(uint256 id, bytes32 tokenId) public view returns (bytes4) {
        // Check that the ID of the NFT type matches the expected value
        require(id == NFT_TYPE, "Invalid NFT type");
        // Look up the EncryptedVideoNFT struct for the specified NFT
        let nft = nft_state.nfts[tokenId];
        // Call
        if let Some(key) = access_key {
            // If the caller is the owner of the NFT and the access key was retrieved successfully,
            // return the key as a bytes4 value
            return licensing::key_id_to_bytes4(key);
        } else {
            // If the caller is not the owner of the NFT or the access key could not be retrieved,
            // return an empty bytes4 value
            return bytes4(0);
        }
    }

    function calculateRoyalty(uint256 id, bytes32 tokenId, uint256 value) public view returns (uint256) {
        // Check that the ID of the NFT type matches the expected value
        require(id == NFT_TYPE, "Invalid NFT type");
        // Look up the ownership history for the specified NFT
        let history = nft_state.history[tokenId];
        // Calculate the royalty fee as a percentage of the sale price
        return value.mul(ROYALTY_FEE).div(100);
    }

    function payoutRoyalty(uint256 id, bytes32 tokenId, uint256 value) public {
        // Check that the ID of the NFT type matches the expected value
        require(id == NFT_TYPE, "Invalid NFT type");
        // Look up the ownership history for the specified NFT
        let history = nft_state.history[tokenId];
        // Check that the caller is the current owner of the NFT
        require(msg.sender == history.owner, "Only the current owner can claim the royalty fee");
        // Calculate the royalty fee as a percentage of the sale price
        let royalty_fee = calculateRoyalty(id, tokenId, value);
        // Transfer the royalty fee to the original owner of the NFT
        contracts::transfer_from_to(
            ERC1155Mutable::address,
            msg.sender,
            history.owner,
            id,
            tokenId,
            royalty_fee
        );
        // Update the ownership history to reflect the new owner of the NFT
        nft_state.history[tokenId] = OwnableHistory { owner: msg.sender };
    }
}



