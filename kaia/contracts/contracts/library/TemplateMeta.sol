// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.0;

import "./Stage.sol";

struct TemplateMeta {
    uint256 templateId;
    address creator;
    uint64 createdAt;
    Stage stage;
    bool locked;
    string topic;
    string purpose;
    string background;
    string responseMethod;
    uint64 configVoteStart;
    uint64 configVoteEnd;
    uint32 configYes;
    uint32 configNo;
    uint64 finalizeVoteStart;
    uint64 finalizeVoteEnd;
}