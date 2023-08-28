# Build your own NFT collection with metadata stored on IPFS

## Requirements

- There should only exist 10 LearnWeb3 Punk NFT's and each one of them should be unique.
- User's should be able to mint only 1 NFT with one transaction.
- The metadata for the NFT's should be stored on IPFS
- There should be a website for your NFT Collection.
- The NFT contract should be deployed on Mumbai testnet

## IPFS

1. Connect to Pinata: https://app.pinata.cloud/pinmanager
2. Upload the LW3Punks images folder to Pinata
3. Upload the LW3Punks metadata folder to Pinata
3. Get the IPFS CID for these folders

Images: https://ipfs.io/ipfs/QmQBHarz2WFczTjz5GnhjHrbUPDnB48W5BM2v2h6HbE1rZ/

Metadata: https://ipfs.io/ipfs/QmPNXxCQo7YKsL22y8MewPknZnWySnm5y6QtLLjMqaEMDb/

## Contract

```bash
npm init -y
npm install --save-dev hardhat
npx hardhat
npm install @openzeppelin/contracts
npm install dotenv
```

Create a .env file and add the following:

```bash
QUICKNODE_HTTP_URL="add-quicknode-http-provider-url-here"
PRIVATE_KEY="add-the-private-key-here"
```

```bash
npx hardhat compile
npx hardhat run scripts/deploy.js --network mumbai
```

LW3Punks Contract Address: 0x962C3DDE84828D26b91670F0c56EB0a583631C06

## App

```bash
npx create-next-app@latest
npm install web3modal
npm install ethers@5
npm run dev
```

Tx buy NFT: https://mumbai.polygonscan.com/tx/0x3536d288bae5634e4c735f993cccbb0e92f6c640695c74b464855eab105f62b2