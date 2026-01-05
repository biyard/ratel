// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IndexData.sol";
import "./ExtensionType.sol";

struct RegisteredExtension {
    string name;
    uint revision;
    address addr;
    uint256 permissions;
    uint256 hookPermissions;
    ExtensionType extensionType;
}

library RegisteredExtensionLib {
    struct Data {
        string[] srcs;
        RegisteredExtension[] dsts;
        mapping(address => string) reverseExtensionName;
        mapping(address => bool) reverses;
        mapping(string => RegisteredExtension) data;
        mapping(string => IndexData) indexes;
    }

    function add(Data storage self, RegisteredExtension memory dst) internal {
        string memory src = dst.name;
        require(!self.indexes[src].exists, "already registered extension");

        self.indexes[src] = IndexData(true, self.srcs.length);
        self.srcs.push(src);
        self.dsts.push(dst);
        self.reverses[dst.addr] = true;
        self.reverseExtensionName[dst.addr] = dst.name;

        self.data[src] = dst;
    }

    function update(Data storage self, RegisteredExtension memory dst) internal {
        string memory src = dst.name;
        require(self.indexes[src].exists, "could not found the extension");
        uint ind = self.indexes[src].index;
        self.reverses[self.dsts[ind].addr] = false;
        self.reverseExtensionName[dst.addr] = "";
        self.dsts[ind] = dst;
        self.reverses[self.dsts[ind].addr] = true;
        self.reverseExtensionName[dst.addr] = dst.name;
        self.data[src] = dst;
    }

    function del(Data storage self, string memory src) internal {
        require(self.indexes[src].exists, "not found address");

        uint ind = self.indexes[src].index;

        string memory name = self.srcs[self.srcs.length - 1];
        self.indexes[name] = IndexData(true, ind);

        delete self.reverses[self.dsts[ind].addr];
        delete self.reverseExtensionName[self.dsts[ind].addr];
        delete self.indexes[src];

        self.srcs[ind] = self.srcs[self.srcs.length - 1];
        self.dsts[ind] = self.dsts[self.dsts.length - 1];
        self.srcs.pop();
        self.dsts.pop();

        delete self.data[src];
    }

    function reverseExtensionNames(Data storage self, address src) internal view returns (string memory) {
        return self.reverseExtensionName[src];
    }

    function reverseExists(Data storage self, address src) internal view returns (bool) {
        return self.reverses[src];
    }

    function list(Data storage self) internal view returns (RegisteredExtension[] memory) {
        return self.dsts;
    }

    function get(Data storage self, string memory src) internal view returns (RegisteredExtension memory) {
        return self.data[src];
    }
}