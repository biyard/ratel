// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.0;

import "./SurveyDaoStateV1.sol";
import "../VersionManager.sol";

import "../library/QuestionMeta.sol";
import "../library/Stage.sol";
import "../library/QuestionType.sol";
import "../library/TemplateMeta.sol";


contract SurveyDao is VersionManager("v2.0") {
    using Address for address;

    ISurveyDaoStateV1 public immutable state;

    event TemplateCreated(uint256 indexed templateId, address indexed creator);
    event ConfigVoted(uint256 indexed templateId, address indexed voter, bool support);
    event StageAdvanced(uint256 indexed templateId, Stage stage);
    event QuestionProposed(uint256 indexed templateId, uint256 indexed questionId, address indexed proposer);
    event QuestionVoted(uint256 indexed templateId, uint256 indexed questionId, address indexed voter, bool approve);
    event TemplateFinalized(uint256 indexed templateId);

    modifier onlyDaoManager() {
        require(msg.sender == state.daoManager(), "ONLY_DAO_MANAGER");
        _;
    }

    constructor(address stateAddr) {
        require(stateAddr != address(0), "BAD_STATE");
        state = ISurveyDaoStateV1(stateAddr);
    }

    // create template
    function createTemplate(
        string calldata topic,
        string calldata purpose,
        string calldata background,
        string calldata responseMethod,
        uint64 configVoteDurationSecs
    ) external returns (uint256 templateId) {
        require(msg.sender != address(0), "BAD_CREATOR");

        uint64 start = uint64(block.timestamp);
        uint64 end = start + configVoteDurationSecs;
        require(end > start, "BAD_VOTE_WINDOW");

        templateId = state.writeCreateTemplate(
            msg.sender,
            topic,
            purpose,
            background,
            responseMethod,
            start,
            end
        );
        emit TemplateCreated(templateId, msg.sender);
    }

    // vote template
    function voteConfig(uint256 templateId, bool support) external {
        TemplateMeta memory t = state.getTemplate(templateId);
        require(t.stage == Stage.VoteConfig, "BAD_STAGE");
        require(block.timestamp >= t.configVoteStart && block.timestamp <= t.configVoteEnd, "VOTE_CLOSED");
        require(!state.hasVotedConfig(templateId, msg.sender), "ALREADY_VOTED");

        state.writeRecordConfigVote(templateId, msg.sender, support);
        emit ConfigVoted(templateId, msg.sender, support);
    }

    // go to suggest question
    function advanceToSuggestQuestions(uint256 templateId) external onlyDaoManager {
        TemplateMeta memory t = state.getTemplate(templateId);
        require(t.stage == Stage.VoteConfig, "BAD_STAGE");
        require(block.timestamp > t.configVoteEnd, "VOTE_NOT_ENDED");
        require(t.configYes > t.configNo, "CONFIG_REJECTED");

        state.writeSetStage(templateId, Stage.SuggestQuestions);
        emit StageAdvanced(templateId, Stage.SuggestQuestions);
    }

    // propose question
    function proposeQuestion(
        uint256 templateId,
        QuestionType qtype,
        bool required,
        string calldata prompt,
        string[] calldata options
    ) external returns (uint256 questionId) {
        TemplateMeta memory t = state.getTemplate(templateId);
        require(t.stage == Stage.SuggestQuestions, "BAD_STAGE");
        require(!t.locked, "TEMPLATE_LOCKED");

        if (qtype == QuestionType.SingleChoice || qtype == QuestionType.MultiChoice) {
            require(options.length >= 2, "NEED_OPTIONS");
        }

        questionId = state.writeAddQuestion(templateId, msg.sender, qtype, required, prompt, options);
        emit QuestionProposed(templateId, questionId, msg.sender);
    }

    // 템플릿 확정
    function openFinalizeVoting(uint256 templateId, uint64 finalizeDurationSecs) external onlyDaoManager {
        TemplateMeta memory t = state.getTemplate(templateId);
        require(t.stage == Stage.SuggestQuestions, "BAD_STAGE");
        require(state.questionCount(templateId) > 0, "NO_QUESTIONS");

        uint64 start = uint64(block.timestamp);
        uint64 end = start + finalizeDurationSecs;
        require(end > start, "BAD_FINALIZE_WINDOW");

        state.writeSetFinalizeWindow(templateId, start, end);
        state.writeSetStage(templateId, Stage.FinalizeVoting);

        emit StageAdvanced(templateId, Stage.FinalizeVoting);
    }

    // 문항별 투표
    function voteQuestion(uint256 templateId, uint256 questionId, bool approve) external {
        TemplateMeta memory t = state.getTemplate(templateId);
        require(t.stage == Stage.FinalizeVoting, "BAD_STAGE");
        require(block.timestamp >= t.finalizeVoteStart && block.timestamp <= t.finalizeVoteEnd, "VOTE_CLOSED");
        require(!state.hasVotedQuestion(templateId, questionId, msg.sender), "ALREADY_VOTED");

        state.getQuestion(templateId, questionId);

        state.writeRecordQuestionVote(templateId, questionId, msg.sender, approve);
        emit QuestionVoted(templateId, questionId, msg.sender, approve);
    }

    // Finalize template
    function finalizeTemplate(
        uint256 templateId,
        uint32 minApprove
    ) external onlyDaoManager {
        TemplateMeta memory t = state.getTemplate(templateId);
        require(t.stage == Stage.FinalizeVoting, "BAD_STAGE");
        require(block.timestamp > t.finalizeVoteEnd, "VOTE_NOT_ENDED");

        uint256 qc = state.questionCount(templateId);
        for (uint256 qid = 1; qid <= qc; qid++) {
            QuestionMeta memory q = state.getQuestion(templateId, qid);
            bool included = (q.approve >= minApprove) && (q.approve > q.reject);
            state.writeMarkIncluded(templateId, qid, included);
        }

        state.writeSetStage(templateId, Stage.Finalized);
        state.writeSetLocked(templateId, true);

        emit TemplateFinalized(templateId);
        emit StageAdvanced(templateId, Stage.Finalized);
    }

    function getTemplate(uint256 templateId) external view returns (TemplateMeta memory) {
        return state.getTemplate(templateId);
    }

    function getQuestion(uint256 templateId, uint256 questionId) external view returns (QuestionMeta memory) {
        return state.getQuestion(templateId, questionId);
    }

    function getQuestionOptions(uint256 templateId, uint256 questionId) external view returns (string[] memory) {
        return state.getQuestionOptions(templateId, questionId);
    }
}

library Address {
    function isContract(address account) internal view returns (bool) {
        return account.code.length > 0;
    }
}
