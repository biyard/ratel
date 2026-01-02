// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.0;

import "./QuestionType.sol";

struct QuestionMeta {
    uint256 questionId;
    address proposer;
    uint64 createdAt;
    QuestionType qtype;
    bool required;
    bool included;
    uint32 approve;
    uint32 reject;
    string prompt;
}