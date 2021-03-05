// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

import "./Storage.sol";

contract Proxy is Storage {
    address currentAddress;

    constructor(address _currentAddress) public {
        currentAddress = _currentAddress;
    }

    function upgrade(address _newAddress) public {
        currentAddress = _newAddress;
    }

    //function does not have name, this fallback function
    //delegatecall every function call
    function () external payable {
        //redirect to currentaddress, low level language
        address implementation = currentAddress;
        require(currentAddress != address(0));
        bytes memory data = msg.data;

        assembly {
            let result := delegatecall(
                gas,
                implementation,
                add(data, 0x20),
                mload(data),
                0,
                0
            )
            let size := returndatasize
            let ptr := mload(0x40)
            returndatacopy(ptr, 0, size)
            switch result
                case 0 {
                    revert(ptr, size)
                }
                default {
                    return(ptr, size)
                }
        }
    }
}
