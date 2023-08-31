# Testing smart contracts on a local blockchain node using Hardhat

Link: https://learnweb3.io/degrees/ethereum-developer-degree/junior/testing-smart-contracts-on-a-local-blockchain-node-using-hardhat/

## Hardhat 

```bash
npm init --yes
npm install --save-dev hardhat
npx hardhat
npx hardhat node
npx hardhat run scripts/deploy.js --network localhost
```

## Testing

1. Go to https://remix.ethereum.org/
2. Import an test account using private key of the local node on Metamask
3. Create the Greeter.sol file
4. Compile the contract
5. Deploy the contract
6. Interact with the contract on Remix (see logs in the local node console)