// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IndexData.sol";

uint16 constant VOTE_STATUS_OPEN = 1;
uint16 constant VOTE_STATUS_VOTING = 2;
uint16 constant VOTE_STATUS_ACCEPTED = 3;
uint16 constant VOTE_STATUS_REJECTED = 4;
uint16 constant VOTE_STATUS_REVIWING = 5;
uint16 constant VOTE_STATUS_PENDED = 6;

struct ProposalSummary {
    uint256 proposalId;
    address proposer;
    string title;
    string proposalAppName;
    string voteAppName;
    string subCategory;
    uint256 submittedAt;
    uint16 voteStatus;
    uint256 numberOfVotes;
}

library ProposalSummaryLib {
    struct Data {
        bytes[] srcs;
        ProposalSummary[] dsts;
        mapping(bytes => ProposalSummary) data;
        mapping(bytes => IndexData) indexes;
    }

    function add(Data storage self, ProposalSummary memory dst, bool isUpsert) internal returns (bytes memory) {
        bytes memory uuid = abi.encodePacked(dst.proposalAppName, dst.proposalId);

        require(!self.indexes[uuid].exists || isUpsert, "failed to insert");

        if (!self.indexes[uuid].exists) {
            self.indexes[uuid] = IndexData(true, self.srcs.length);
            self.srcs.push(uuid);
            self.dsts.push(dst);
        } else {
            uint256 ind = self.indexes[uuid].index;
            self.dsts[ind] = dst;
        }

        self.data[uuid] = dst;

        return uuid;
    }

    function del(Data storage self, string memory proposalAppName, uint256 proposalId) internal {
        bytes memory uuid = abi.encodePacked(proposalAppName, proposalId);

        require(self.indexes[uuid].exists, "not found address");

        uint ind = self.indexes[uuid].index;
        self.indexes[uuid] = IndexData(true, ind);
        delete self.indexes[uuid];

        self.srcs[ind] = self.srcs[self.srcs.length - 1];
        self.dsts[ind] = self.dsts[self.dsts.length - 1];
        self.srcs.pop();
        self.dsts.pop();

        delete self.data[uuid];
    }

    function list(Data storage self) internal view returns (ProposalSummary[] memory) {
        return self.dsts;
    }

    function get(
        Data storage self,
        string memory proposalAppName,
        uint256 proposalId
    ) internal view returns (ProposalSummary memory, bool) {
        bytes memory uuid = abi.encodePacked(proposalAppName, proposalId);

        return (self.data[uuid], self.indexes[uuid].exists);
    }
}