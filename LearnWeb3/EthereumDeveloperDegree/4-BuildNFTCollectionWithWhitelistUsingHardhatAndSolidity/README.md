# Build an NFT collection with a whitelist using Hardhat and Solidity

Link: https://learnweb3.io/degrees/ethereum-developer-degree/sophomore/build-an-nft-collection-with-a-whitelist-using-hardhat-and-solidity/

## Step 1 - Whitelist Contract

In the hardhat folder, run the following commands:

```bash
npm init --yes
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox
npx hardhat # Create a JavaScript Project
npm install dotenv
``` 

## Step 2

1. Setup your `.env` file with the following variables:

```bash
PRIVATE_KEY="..."
RPC_URL="..."
ETHERSCAN_API_KEY="..."
```

2. Setup your `hardhat.config.js` file
3. Deploy your contract to the Sepolia testnet

```bash
npx hardhat run scripts/deploy.js --network sepolia
```

Whitelist Contract Address: 0x2a95f25ea2C08a98f46E8b68A6fCcDe26F931e49
https://sepolia.etherscan.io/address/0x2a95f25ea2C08a98f46E8b68A6fCcDe26F931e49#code

## Step 3

1. Go to Write Contract tab on Sepolia etherscan to interact with the contract

## Step 4 - NFT Contract

```bash
npm install @openzeppelin/contracts
```

1. Deploy your contract to the Sepolia testnet

```bash
npx hardhat run scripts/deploy-nft.js --network sepolia
```

NFT Contract Address: 0xF2B039CeceF5b88dd9bAdfb0a0E98E77d5C52259
https://sepolia.etherscan.io/address/0xF2B039CeceF5b88dd9bAdfb0a0E98E77d5C52259#code

Mint NFT with whitelist address: https://sepolia.etherscan.io/tx/0x3191f08b726125d7a85361de61abca8151f26548ff1ccd597da79afaea6438b6