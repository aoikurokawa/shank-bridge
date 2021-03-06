// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

contract Bank {

    address owner;
    mapping (address => uint) balances;

    bool private _paused;

    modifier onlyOwner {
        require(owner == owner);
        _;
    }

    //allow to execute when contract 
    modifier whenNotPaused {
        require(!_paused);
        _;
    }

    modifier whenPaused {
        require(_paused);
        _;
    }

    constructor() internal {
        _paused = false;
    }

    function pause() public onlyOwner whenNotPaused {
        _paused = true;
    }

    function unPause() public onlyOwner whenPaused {
        _paused = false;
    }

    function withdrawAll() public whenNotPaused {
        uint amountToWithdraw = balances[msg.sender];
        balances[msg.sender] = 0;
        require(msg.sender.call.value(amountToWithdraw));
    }

    //where do i put paused modifier function 
}
