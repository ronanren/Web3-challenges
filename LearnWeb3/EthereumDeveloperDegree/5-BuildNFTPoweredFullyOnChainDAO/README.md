# Build an NFT-powered fully on-chain DAO to invest in NFT collections as a group

## 📝 Requirements

- Anyone who owns a CryptoDevs NFT can create a proposal to purchase a different NFT from an NFT marketplace
- Everyone with a CryptoDevs NFT can vote for or against the active proposals
- Each NFT counts as one vote for each proposal
- Voters cannot vote multiple times on the same proposal with the same NFT
- If majority of the voters vote in favor of the proposal by the deadline, the NFT purchase happens automatically from the marketplace

## Onchain-dao

```bash
npm init --yes
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox
npx hardhat # Create a JavaScript Project
npm install @openzeppelin/contracts
npm install dotenv
npx hardhat run scripts/deploy.js --network sepolia
```

CryptoDevsNFT deployed to: 0xd4f1dC82f8ce84895B1fdac5150f42908BFd4835

FakeNFTMarketplace deployed to: 0x511bE2019Ad10886E7DF8B63d9c0613DBCC2024b

CryptoDevsDAO deployed to: 0xC4F35d376A749ff7Dcf7Bf790efC26653B9eEbEB

## Frontend

```bash
npx create-next-app@latest frontend
npm install @rainbow-me/rainbowkit wagmi viem
```

- Change projectId of Wallet Connect in _app.js

```bash
npm run dev
```