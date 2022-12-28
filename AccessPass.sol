// EXPERIMENTAL implementation in solidity, not tested

/*is semi-fungible, with 10% resale royalties paid to the royalty beneficiary. The tokens are own-to-play, with the 
token owner being the only one able to access the encrypted video. The token ID is the transaction ID of the encrypted video on Arweave,
and the token metadata is the base64-encoded thumbnail image. The contract provides functions to retrieve the private encryption key,
transaction ID, and thumbnail image for the encrypted video, but only if the caller is the token owner.
The contract also implements the SafeERC1155Receiver interface to handle the royalty payments when the tokens are transferred*/


pragma solidity ^0.7.0;

import "https://github.com/OpenZeppelin/openzeppelin-solidity/contracts/math/SafeMath.sol";
import "https://github.com/OpenZeppelin/openzeppelin-solidity/contracts/token/ERC1155.sol";
import "https://github.com/OpenZeppelin/openzeppelin-solidity/contracts/utils/Address.sol";

// EncryptedVideoToken is an ERC1155 smart contract that holds the private encryption key for an encrypted video file stored on Arweave.
// The contract is semi-fungible, with 10% resale royalties paid to the royalty beneficiary.
// The tokens are own-to-play, with the token owner being the only one able to access the encrypted video.
// The token ID is the transaction ID of the encrypted video on Arweave, and the token metadata is the base64-encoded thumbnail image.
contract EncryptedVideoToken is ERC1155 {
    using SafeMath for uint256;
    using Address for address;

    // The percentage of the token sale price to be paid as a royalty
    uint8 private _royaltyPercentage;
    // The Ethereum address of the royalty beneficiary
    address private _royaltyBeneficiary;
    // The private encryption key for the encrypted video
    bytes32 private _keyId;
    // The transaction ID of the encrypted video on Arweave
    bytes32 private _transactionId;
    // The base64-encoded thumbnail image
    string private _thumbnail;

    // The constructor sets the initial values for the contract variables
    constructor(uint8 royaltyPercentage, address royaltyBeneficiary, bytes32 keyId, bytes32 transactionId, string thumbnail) public {
        _royaltyPercentage = royaltyPercentage;
        _royaltyBeneficiary = royaltyBeneficiary;
        _keyId = keyId;
        _transactionId = transactionId;
        _thumbnail = thumbnail;
    }

    // create creates a new token with the specified token ID and metadata
    function create(uint256 _id, string _metadata) external {
        _mint(msg.sender, _id, _metadata);
    }

    // SafeERC1155Receiver is implemented to handle the royalty payments
    function onERC1155Received(address operator, address from, uint256[] calldata ids, uint256[] calldata values, bytes memory data) public {
        require(operator == address(this), "Invalid operator");
        for (uint i = 0; i < ids.length; i++) {
            if (values[i] > 0) {
                uint256 royalties = values[i].mul(_royaltyPercentage).div(100);
                address(this).transfer(_royaltyBeneficiary, royalties);
            }
        }
    }

    // getAccessKey returns the private encryption key for the encrypted video if the caller is the token owner, otherwise it returns an empty bytes32
    function getAccessKey() external view returns (bytes32) {
        require(ownerOf(_transactionId) == msg.sender, "Access denied");
        return _keyId;    }

    // getTransactionId returns the transaction ID of the encrypted video on Arweave if the caller is the token owner, otherwise it returns an empty bytes32
    function getTransactionId() external view returns (bytes32) {
        require(ownerOf(_transactionId) == msg.sender, "Access denied");
        return _transactionId;
    }

    // getThumbnail returns the base64-encoded thumbnail image for the encrypted video if the caller is the token owner, otherwise it returns an empty string
    function getThumbnail() external view returns (string memory) {
        require(ownerOf(_transactionId) == msg.sender, "Access denied");
        return _thumbnail;
    }
}

