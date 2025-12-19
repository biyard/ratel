// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.20;

import "../states/access/StateOperator.sol";
import "../states/access/IStateOperator.sol";

interface ISurveyDaoStateV1 is IStateOperator {
    enum Stage { VoteConfig, SuggestQuestions, FinalizeVoting, Finalized }
    enum QuestionType { SingleChoice, MultiChoice, ShortText, LongText, Number, Rating5 }

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
    mapping(uint256 => ISurveyDaoStateV1.TemplateMeta) private _templates;

    mapping(uint256 => uint256) private _questionCount;
    mapping(uint256 => mapping(uint256 => ISurveyDaoStateV1.QuestionMeta)) private _questions;
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

    function getTemplate(uint256 templateId) external view canRead returns (ISurveyDaoStateV1.TemplateMeta memory) {
        ISurveyDaoStateV1.TemplateMeta memory t = _templates[templateId];
        require(t.templateId != 0, "TEMPLATE_NOT_FOUND");
        return t;
    }

    function questionCount(uint256 templateId) external view canRead returns (uint256) {
        require(_templates[templateId].templateId != 0, "TEMPLATE_NOT_FOUND");
        return _questionCount[templateId];
    }

    function getQuestion(uint256 templateId, uint256 questionId) external view canRead returns (ISurveyDaoStateV1.QuestionMeta memory) {
        require(_templates[templateId].templateId != 0, "TEMPLATE_NOT_FOUND");
        ISurveyDaoStateV1.QuestionMeta memory q = _questions[templateId][questionId];
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
        require(creator != address(0), "BAD_CREATOR");
        require(voteEnd > voteStart, "BAD_VOTE_WINDOW");

        _templateCount += 1;
        templateId = _templateCount;

        ISurveyDaoStateV1.TemplateMeta storage t = _templates[templateId];
        t.templateId = templateId;
        t.creator = creator;
        t.createdAt = uint64(block.timestamp);

        t.stage = ISurveyDaoStateV1.Stage.VoteConfig;
        t.locked = false;

        t.topic = topic;
        t.purpose = purpose;
        t.background = background;
        t.responseMethod = responseMethod;

        t.configVoteStart = voteStart;
        t.configVoteEnd = voteEnd;
    }

    function writeSetStage(uint256 templateId, ISurveyDaoStateV1.Stage stage_) external canWrite {
        ISurveyDaoStateV1.TemplateMeta storage t = _templates[templateId];
        require(t.templateId != 0, "TEMPLATE_NOT_FOUND");
        t.stage = stage_;
    }

    function writeSetLocked(uint256 templateId, bool locked_) external canWrite {
        ISurveyDaoStateV1.TemplateMeta storage t = _templates[templateId];
        require(t.templateId != 0, "TEMPLATE_NOT_FOUND");
        t.locked = locked_;
    }

    function writeRecordConfigVote(uint256 templateId, address voter, bool support) external canWrite {
        ISurveyDaoStateV1.TemplateMeta storage t = _templates[templateId];
        require(t.templateId != 0, "TEMPLATE_NOT_FOUND");
        require(!_votedConfig[templateId][voter], "ALREADY_VOTED");

        _votedConfig[templateId][voter] = true;

        if (support) t.configYes += 1;
        else t.configNo += 1;
    }

    function writeAddQuestion(
        uint256 templateId,
        address proposer,
        ISurveyDaoStateV1.QuestionType qtype,
        bool required,
        string calldata prompt,
        string[] calldata options
    ) external canWrite returns (uint256 questionId) {
        ISurveyDaoStateV1.TemplateMeta storage t = _templates[templateId];
        require(t.templateId != 0, "TEMPLATE_NOT_FOUND");
        require(!t.locked, "TEMPLATE_LOCKED");

        _questionCount[templateId] += 1;
        questionId = _questionCount[templateId];

        ISurveyDaoStateV1.QuestionMeta storage q = _questions[templateId][questionId];
        q.questionId = questionId;
        q.proposer = proposer;
        q.createdAt = uint64(block.timestamp);
        q.qtype = qtype;
        q.required = required;
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
        require(_templates[templateId].templateId != 0, "TEMPLATE_NOT_FOUND");

        ISurveyDaoStateV1.QuestionMeta storage q = _questions[templateId][questionId];
        require(q.questionId != 0, "QUESTION_NOT_FOUND");
        require(!_votedQuestion[templateId][questionId][voter], "ALREADY_VOTED");

        _votedQuestion[templateId][questionId][voter] = true;

        if (approve) q.approve += 1;
        else q.reject += 1;
    }

    function writeSetFinalizeWindow(uint256 templateId, uint64 start, uint64 end) external canWrite {
        ISurveyDaoStateV1.TemplateMeta storage t = _templates[templateId];
        require(t.templateId != 0, "TEMPLATE_NOT_FOUND");
        require(end > start, "BAD_FINALIZE_WINDOW");
        t.finalizeVoteStart = start;
        t.finalizeVoteEnd = end;
    }

    function writeMarkIncluded(uint256 templateId, uint256 questionId, bool included) external canWrite {
        require(_templates[templateId].templateId != 0, "TEMPLATE_NOT_FOUND");

        ISurveyDaoStateV1.QuestionMeta storage q = _questions[templateId][questionId];
        require(q.questionId != 0, "QUESTION_NOT_FOUND");

        q.included = included;
    }
}
