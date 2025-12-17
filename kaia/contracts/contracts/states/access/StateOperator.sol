// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.0;

import "../../library/AddressArray.sol";

contract StateOperator {
    using AddressArray for AddressArray.Data;
    AddressArray.Data private _operators;

    bool private _allowPublicRead;
    bool private _allowPublicWrite;
    bool private _ready = false;

    constructor(address owner) {
        require(owner != address(0), "invalid owner");
        bool ok = _operators.add(owner, true); 
        require(ok, "internal error");
        _allowPublicRead = true;
    }

    function migrate(address to) public canWrite {
        addOperator(to);
        delOperator(msg.sender);
    }

    function allowPublicRead(bool allowPublicReadCheck) public canWrite {
        _allowPublicRead = allowPublicReadCheck;
    }

    function allowPublicWrite(bool allowPublicWriteCheck) public canWrite {
        _allowPublicWrite = allowPublicWriteCheck;
    }

    // NOTE: deprecated function. Use `setStateReady`.
    function allowReady(bool ready) public canWrite {
        _ready = ready;
    }

    function setStateReady(bool ready) public canWrite {
        _ready = ready;
    }

    function getStateReady() public view canRead returns (bool) {
        return _ready;
    }

    function publicRead() public view canRead returns (bool) {
        return _allowPublicRead;
    }

    function publicWrite() public view canRead returns (bool) {
        return _allowPublicWrite;
    }

    function addDaoStateOperator(address o) public canWrite {
        _operators.add(o, false);
    }

    function addOperator(address o) public canWrite {
        _operators.add(o, false);
    }

    function delOperator(address o) public canWrite {
        _operators.del(o);
    }

    function listOperators() public view canRead returns (address[] memory operators) {
        return _operators.list();
    }

    function lengthOfOperators() public view canRead returns (uint256) {
        return _operators.length();
    }

    function existsOperator(address o) public view canRead returns (bool) {
        return _operators.exists(o);
    }

    modifier canWrite() {
        require(!_ready || _allowPublicWrite || _operators.exists(msg.sender), "no write permission");
        _;
    }

    modifier canRead() {
        require(!_ready || _allowPublicRead || _operators.exists(msg.sender), "no read permission");
        _;
    }
}