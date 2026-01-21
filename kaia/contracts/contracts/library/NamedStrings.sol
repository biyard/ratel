// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IndexData.sol";

library NamedStrings {
    struct Data {
        string[] srcs;
        string[] dsts;
        mapping(string => string) data;
        mapping(string => IndexData) indexes;
    }

    function add(Data storage self, string memory src, string memory dst, bool isUpsert) internal {
        require(!self.indexes[src].exists || isUpsert, "failed to insert");

        if (!self.indexes[src].exists) {
            self.indexes[src] = IndexData(true, self.srcs.length);
            self.srcs.push(src);
            self.dsts.push(dst);
        } else {
            uint256 ind = self.indexes[src].index;
            self.dsts[ind] = dst;
        }
        self.data[src] = dst;
    }

    function del(Data storage self, string memory src) internal {
        require(self.indexes[src].exists, "not found address");

        uint ind = self.indexes[src].index;

        string memory name = self.srcs[self.srcs.length - 1];
        self.indexes[name] = IndexData(true, ind);

        delete self.indexes[src];

        self.srcs[ind] = self.srcs[self.srcs.length - 1];
        self.dsts[ind] = self.dsts[self.dsts.length - 1];
        self.srcs.pop();
        self.dsts.pop();

        delete self.data[src];
    }

    function exists(Data storage self, string memory src) internal view returns (bool) {
        return self.indexes[src].exists;
    }

    function list(Data storage self) internal view returns (string[] memory, string[] memory) {
        return (self.srcs, self.dsts);
    }

    function get(Data storage self, string memory src) internal view returns (string memory) {
        return self.data[src];
    }
}