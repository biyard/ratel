// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.0;

contract VersionManager {
    string private _version;

    constructor(string memory ver) {
        _version = ver;
    }

    function version() public view returns (string memory) {
        return _version;
    }
}