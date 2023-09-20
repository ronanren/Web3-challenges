# Indexing data using The Graph's Indexer

Link: https://learnweb3.io/degrees/ethereum-developer-degree/junior/indexing-data-using-the-graphs-indexer/

## Steps

1. Create subgraphs: https://thegraph.com/hosted-service
```bash	
npm install -g @graphprotocol/graph-cli
graph init --contract-name RandomWinnerGame --product hosted-service ronanren/learnweb3  --from-contract 0xD0fDCe43Cc8954981bf3Cd5e64fF6aa1F1e52bC4 --abi ./abi.json --network mumbai graph
```
2. Get the access token from https://thegraph.com/hosted-service/dashboard
3. Deploy the subgraph
```bash
graph auth --product hosted-service ACCESS_TOKEN
cd graph
npm run deploy
```

Deployed to https://thegraph.com/explorer/subgraph/ronanren/learnweb3

Queries (HTTP): https://api.thegraph.com/subgraphs/name/ronanren/learnweb3

4. Deploy changes to the subgraph
```bash
npm run codegen
npm run deploy
```

## Website

```bash
npx create-next-app@latest
cd my-app
npm install
npm run dev
```