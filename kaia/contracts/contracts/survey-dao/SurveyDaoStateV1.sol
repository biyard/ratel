// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.0;

import "../states/access/StateOperator.sol";
import "../states/access/IStateOperator.sol";

import "../library/QuestionMeta.sol";
import "../library/Stage.sol";
import "../library/QuestionType.sol";
import "../library/TemplateMeta.sol";

interface ISurveyDaoStateV1 is IStateOperator {
    function daoManager() external view returns (address);
    function setDaoManager(address mgr) external;

    function templateCount() external view returns (uint256);
    function getTemplate(uint256 templateId) external view returns (TemplateMeta memory);

    function questionCount(uint256 templateId) external view returns (uint256);
    function getQuestion(uint256 templateId, uint256 questionId) external view returns (QuestionMeta memory);
    function getQuestionOptions(uint256 templateId, uint256 questionId) external view returns (string[] memory);

    function hasVotedConfig(uint256 templateId, address voter) external view returns (bool);
    function hasVotedQuestion(uint256 templateId, uint256 questionId, address voter) external view returns (bool);

    function writeCreateTemplate(
        address creator,
        string calldata topic,
        string calldata purpose,
        string calldata background,
        string calldata responseMethod,
        uint64 voteStart,
        uint64 voteEnd
    ) external returns (uint256 templateId);

    function writeSetStage(uint256 templateId, Stage stage_) external;
    function writeSetLocked(uint256 templateId, bool locked_) external;

    function writeRecordConfigVote(uint256 templateId, address voter, bool support) external;

    function writeAddQuestion(
        uint256 templateId,
        address proposer,
        QuestionType qtype,
        bool required,
        string calldata prompt,
        string[] calldata options
    ) external returns (uint256 questionId);

    function writeRecordQuestionVote(
        uint256 templateId,
        uint256 questionId,
        address voter,
        bool approve
    ) external;

    function writeSetFinalizeWindow(uint256 templateId, uint64 start, uint64 end) external;
    function writeMarkIncluded(uint256 templateId, uint256 questionId, bool included) external;
}

contract SurveyDaoStateV1 is StateOperator {
    address private _daoManager;

    uint256 private _templateCount;
    mapping(uint256 => TemplateMeta) private _templates;

    mapping(uint256 => uint256) private _questionCount;
    mapping(uint256 => mapping(uint256 => QuestionMeta)) private _questions;
    mapping(uint256 => mapping(uint256 => string[])) private _questionOptions;

    mapping(uint256 => mapping(address => bool)) private _votedConfig;
    mapping(uint256 => mapping(uint256 => mapping(address => bool))) private _votedQuestion;

    constructor(address operator, address daoManager_) StateOperator(operator) {
        require(daoManager_ != address(0), "BAD_MANAGER");
        _daoManager = daoManager_;
    }

    function daoManager() external view canRead returns (address) {
        return _daoManager;
    }

    function setDaoManager(address mgr) external canWrite {
        require(mgr != address(0), "BAD_MANAGER");
        _daoManager = mgr;
    }

    function templateCount() external view canRead returns (uint256) {
        return _templateCount;
    }

    function getTemplate(uint256 templateId) external view canRead returns (TemplateMeta memory) {
        TemplateMeta memory t = _templates[templateId];
        require(t.templateId != 0, "TEMPLATE_NOT_FOUND");
        return t;
    }

    function questionCount(uint256 templateId) external view canRead returns (uint256) {
        require(_templates[templateId].templateId != 0, "TEMPLATE_NOT_FOUND");
        return _questionCount[templateId];
    }

    function getQuestion(uint256 templateId, uint256 questionId) external view canRead returns (QuestionMeta memory) {
        require(_templates[templateId].templateId != 0, "TEMPLATE_NOT_FOUND");
        QuestionMeta memory q = _questions[templateId][questionId];
        require(q.questionId != 0, "QUESTION_NOT_FOUND");
        return q;
    }

    function getQuestionOptions(uint256 templateId, uint256 questionId) external view canRead returns (string[] memory) {
        require(_templates[templateId].templateId != 0, "TEMPLATE_NOT_FOUND");
        require(_questions[templateId][questionId].questionId != 0, "QUESTION_NOT_FOUND");
        return _questionOptions[templateId][questionId];
    }

    function hasVotedConfig(uint256 templateId, address voter) external view canRead returns (bool) {
        return _votedConfig[templateId][voter];
    }

    function hasVotedQuestion(uint256 templateId, uint256 questionId, address voter) external view canRead returns (bool) {
        return _votedQuestion[templateId][questionId][voter];
    }

    function writeCreateTemplate(
        address creator,
        string calldata topic,
        string calldata purpose,
        string calldata background,
        string calldata responseMethod,
        uint64 voteStart,
        uint64 voteEnd
    ) external canWrite returns (uint256 templateId) {
        _templateCount += 1;
        templateId = _templateCount;

        TemplateMeta storage t = _templates[templateId];
        t.templateId = templateId;
        t.creator = creator;
        t.createdAt = uint64(block.timestamp);

        t.stage = Stage.VoteConfig;
        t.locked = false;

        t.topic = topic;
        t.purpose = purpose;
        t.background = background;
        t.responseMethod = responseMethod;

        t.configVoteStart = voteStart;
        t.configVoteEnd = voteEnd;

        t.configYes = 0;
        t.configNo = 0;
        t.finalizeVoteStart = 0;
        t.finalizeVoteEnd = 0;

        _questionCount[templateId] = 0;
    }

    function writeSetStage(uint256 templateId, Stage stage_) external canWrite {
        TemplateMeta storage t = _templates[templateId];
        t.stage = stage_;
    }

    function writeSetLocked(uint256 templateId, bool locked_) external canWrite {
        TemplateMeta storage t = _templates[templateId];
        t.locked = locked_;
    }

    function writeRecordConfigVote(uint256 templateId, address voter, bool support) external canWrite {
        _votedConfig[templateId][voter] = true;

        TemplateMeta storage t = _templates[templateId];
        if (support) t.configYes += 1;
        else t.configNo += 1;
    }

    function writeAddQuestion(
        uint256 templateId,
        address proposer,
        QuestionType qtype,
        bool required,
        string calldata prompt,
        string[] calldata options
    ) external canWrite returns (uint256 questionId) {
        _questionCount[templateId] += 1;
        questionId = _questionCount[templateId];

        QuestionMeta storage q = _questions[templateId][questionId];
        q.questionId = questionId;
        q.proposer = proposer;
        q.createdAt = uint64(block.timestamp);
        q.qtype = qtype;
        q.required = required;
        q.included = false;
        q.approve = 0;
        q.reject = 0;
        q.prompt = prompt;

        delete _questionOptions[templateId][questionId];
        for (uint256 i = 0; i < options.length; i++) {
            _questionOptions[templateId][questionId].push(options[i]);
        }
    }

    function writeRecordQuestionVote(
        uint256 templateId,
        uint256 questionId,
        address voter,
        bool approve
    ) external canWrite {
        _votedQuestion[templateId][questionId][voter] = true;

        QuestionMeta storage q = _questions[templateId][questionId];
        if (approve) q.approve += 1;
        else q.reject += 1;
    }

    function writeSetFinalizeWindow(uint256 templateId, uint64 start, uint64 end) external canWrite {
        TemplateMeta storage t = _templates[templateId];
        t.finalizeVoteStart = start;
        t.finalizeVoteEnd = end;
    }

    function writeMarkIncluded(uint256 templateId, uint256 questionId, bool included) external canWrite {
        QuestionMeta storage q = _questions[templateId][questionId];
        q.included = included;
    }
}
