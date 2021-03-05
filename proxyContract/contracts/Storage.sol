// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

//variable

contract Storage {

    // _uintStorage["Number"] = 1;
    mapping (string => uint256) _uintStorage;

    mapping (string => address) _addressStorage;

    mapping (string => bool) _boolStorage;

    mapping (string => string) _stringStorage;

    mapping (string => bytes4) _bytesStorage;

    address public owner;

    bool public _initialized;
    

}

