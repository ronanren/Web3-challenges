# Build your own decentralized exchange like Uniswap v1

## 📝 Requirements

- Build an exchange that allows swapping ETH <> TOKEN
- DEX must charge a 1% fee on swaps
- When user adds liquidity, they must be given an LP Token that represents their share of the pool
- LP must be able to burn their LP tokens to receive back ETH and TOKEN

## Steps

```bash	
npm init --yes
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox
npx hardhat # Create a JavaScript Project
npm install @openzeppelin/contracts dotenv
npx hardhat run scripts/deploy.js --network sepolia
```

Token deployed to: 0xeF443Af1a1CBf3041840da213FFF8b297486Cf33

Exchange deployed to: 0xb14E71157c33b79501Ea420344a7364C02Da6c96

1. Give Allowance of the Token to the Exchange address (0xb14E71157c33b79501Ea420344a7364C02Da6c96) and 10000000000000000000 as amount (10 tokens) 
2. Add Liquidity to the Exchange (0xb14E71157c33b79501Ea420344a7364C02Da6c96) with 0.1 ETH and 10000000 as amount 
3. Swap ETH to Token with ethToTokenSwap method with 0.01 ETH as amount and 0 as minTokensToReceive
4. Swap Token to ETH with tokenToEthSwap method with 900818 as amount and 0 as minEthToReceive
5. Remove Liquidity from the Exchange (0xb14E71157c33b79501Ea420344a7364C02Da6c96) with 5000000000000 as amount
