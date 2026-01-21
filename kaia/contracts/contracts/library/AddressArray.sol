// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IndexData.sol";

library AddressArray {
    struct Data {
        address[] addresses;
        mapping(address => IndexData) indexes;
    }

    function add(Data storage self, address addr, bool isUpsert) internal returns (bool) {
        if (self.indexes[addr].exists && !isUpsert) return false;

        if (!self.indexes[addr].exists) {
            self.indexes[addr] = IndexData(true, self.addresses.length);
            self.addresses.push(addr);
        } else {
            uint256 ind = self.indexes[addr].index;
            self.addresses[ind] = addr;
        }

        return true;
    }

    function del(Data storage self, address addr) internal {
        require(self.indexes[addr].exists, "not found address");
        uint256 ind = self.indexes[addr].index;

        address addr1 = self.addresses[self.addresses.length - 1];
        self.indexes[addr1] = IndexData(true, ind);

        delete self.indexes[addr];

        self.addresses[ind] = self.addresses[self.addresses.length - 1];
        self.addresses.pop();
    }

    function exists(Data storage self, address addr) internal view returns (bool) {
        return self.indexes[addr].exists;
    }

    function list(Data storage self) internal view returns (address[] memory) {
        return self.addresses;
    }

    function length(Data storage self) internal view returns (uint256) {
        return self.addresses.length;
    }

    function get(Data storage self, uint256 ind) internal view returns (address) {
        return self.addresses[ind];
    }

    function set(Data storage self, uint256 ind, address addr) internal {
        address temp = self.addresses[ind];
        delete self.indexes[temp];
        self.addresses[ind] = addr;
        self.indexes[addr] = IndexData(true, ind);
    }
}