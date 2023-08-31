# Secure on-chain randomness using Chainlink VRFs

Link: https://learnweb3.io/degrees/ethereum-developer-degree/junior/secure-on-chain-randomness-using-chainlink-vrfs/

## Requirements

- We will build a lottery game today
- Each game will have a max number of players and an entry fee
- After max number of players have entered the game, one winner is chosen at random
- The winner will get maxplayers*entryfee amount of ether for winning the game

## Steps

```bash
npm init -y
npm install --save-dev hardhat
npx hardhat
npm install @openzeppelin/contracts
npm install @chainlink/contracts
npm install dotenv
```

Create a .env file and add the following:

```bash
QUICKNODE_HTTP_URL="add-quicknode-http-provider-url-here"
PRIVATE_KEY="add-the-private-key-here"
POLYGONSCAN_KEY="polygonscan-api-key-token-here"
```

```bash
npx hardhat compile
npx hardhat run scripts/deploy.js --network mumbai
```

Contract Address: 0xD0fDCe43Cc8954981bf3Cd5e64fF6aa1F1e52bC4

## Testing

1. Claim LINK tokens from the faucet: https://faucets.chain.link/mumbai and send it to the contract address
2. Connect your wallet to interact with the contract: https://mumbai.polygonscan.com/address/0xD0fDCe43Cc8954981bf3Cd5e64fF6aa1F1e52bC4#code
3. Start a new game by calling the `startGame` function with 2 max_players and 10 WEI entry_fee: https://mumbai.polygonscan.com/tx/0x3e85e28e2630e918106fc3b067eb635883654343ac1173a6915d4372d26cf177
4. Join the game with 2 different wallets with 0.00000000000000001 as payableAmount

First player: https://mumbai.polygonscan.com/tx/0x60078771aa21ff588189a5725b0f73a67a2aae156ae0b75faadec97e15fdacd8#eventlog

Second player: https://mumbai.polygonscan.com/tx/0xc483b85666989bcdfc98982c1fdc8e8645f4bd189f1722ac4f4bf913f5ef0e6c#eventlog

I received the amount to the winner: https://mumbai.polygonscan.com/tx/0x043cae5d4c2926d20e14468c1dc5d117c9b5831435b48ec99dc73b7f5f0374d9