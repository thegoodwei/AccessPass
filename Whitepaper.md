# AccessPass, Smart Contract dApp in Rustlang for Encrypted Video Tokens using ERC1155

In this whitepaper, we introduce a smart contract and dApp for creating and managing encrypted video tokens using the ERC1155 standard. The smart contract allows users to buy and sell semi-fungible tokens that grant access to an encrypted video. The dApp provides a user-friendly interface for interacting with the smart contract and decrypting and playing the video using the access key provided in the token.

### Motivation

The use of decentralized applications (dApps) on the Ethereum network is growing rapidly, as it allows for the creation of decentralized, trustless, and transparent systems for a wide range of applications. One of the areas where dApps can have a significant impact is the media industry, by providing a platform for creators to monetize their content and for users to access high-quality content without intermediaries.

However, the distribution of media content on the Ethereum network poses several challenges, including security, scalability, and copyright protection. Encrypting the content and storing it on a decentralized storage platform can address the security and scalability issues, but it does not solve the problem of copyright protection. The use of a smart contract and non-fungible tokens (NFTs) can provide a solution by creating a system for managing the access and ownership of the content.

### Solution

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
   -     Rust compiler: https://www.rust-lang.org/tools/install
   -     Rust package manager (Cargo): https://doc.rust-lang.org/cargo/getting-started/installation.html
   -     Rust WebAssembly compiler (wasm-pack): https://rustwasm.github.io/wasm-pack/installer/
   -     Ethereum wallet: https://www.ethereum.org/greeter
   -     Web3 provider: https://web3js.readthedocs.io/en/v1.2.11/getting-started.html#installing-web3
   -     Arweave client: https://www.arweave.org/developers/tools/arweave-cli

    Clone this repository and navigate to the root directory:
``
    git clone https://github.com/thegoodwei/AccessPass
    cd encrypted-video-tokens
``

3. Build the smart contract and the dApp:
``
wasm-pack build
``
This will generate the WASM bytecode and the ABI for the smart contract and the dApp.

4. Deploy the smart contract to the Ethereum network:
- Use your Ethereum wallet to create a new account or import an existing one.
- Use the Web3 provider to send a transaction to the Ethereum network, deploying the smart contract and specifying the required parameters:
  - `thumbnail`: The thumbnail image for the video, encoded as a base64 string.
  - `royalty_percentage`: The percentage of the sale price to be transferred as royalties.
  - `nft_type`: The type of the NFT, represented as a bytes32 string.

5. Deploy the dApp to a web server or a decentralized storage platform, such as IPFS or Arweave.

6. Use the dApp to interact with the smart contract and manage the encrypted video tokens.

## Conclusion

In this whitepaper, we have introduced a smart contract and dApp for creating and managing encrypted video tokens using the ERC1155 standard. The smart contract allows users to buy and sell semi-fungible tokens that grant access to an encrypted video, and it implements a royalty system for the creators of the video. The dApp provides a user-friendly interface for interacting with the smart contract and decrypting and playing the video using the access key provided in the token.

Our solution addresses the challenges of distributing media content on the Ethereum network by providing a scalable, secure, and transparent platform for creators to monetize their content and for users to access high-quality content with the assurance of copyright protection.

We hope that this solution will contribute to the growth and adoption of dApps in the media industry, removing bias from content publishing incentives to enable a new renaissance of independent creators.
