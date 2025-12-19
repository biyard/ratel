// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.20;

import "./DaoRegistryStateV1.sol";
import "../VersionManager.sol";
import "../library/BytesLib.sol";
import "../standards/eip/EIP2771.sol";
import "../standards/eip/NativeMetaTransaction.sol";

interface ISurveyDaoStateLike {
    function daoManager() external view returns (address);
    function existsOperator(address who) external view returns (bool);
}

interface ISurveyDaoLike {
    function state() external view returns (address);
}

contract DaoRegistry is VersionManager("v2.0"), EIP2771, NativeMetaTransaction {
    IDaoRegistryStateV1 private _state;
    address private _owner;

    string constant NAME = "NAME";

    event changeInitialDataEvent(string name, string value, address daoManagerAddress);

    event SurveyDaoRegistered(
        uint256 indexed daoId,
        string name,
        address indexed daoManager,
        address indexed operator,
        address surveyDao,
        address surveyState
    );

    constructor(string memory n, address stateAddr, address operator) EIP2771(address(0)) {
        _initializeEIP712(n);
        
        require(stateAddr != address(0), "BAD_STATE");
        require(operator != address(0), "BAD_OPERATOR");

        _state = IDaoRegistryStateV1(stateAddr);

        _state.addOperator(address(this));
        _state.addOperator(operator);
        _state.addNamedString(NAME, n);
        _state.setStateReady(true);
        _state.setDaoManagerAddress(msg.sender);

        _owner = msg.sender;

        emit changeInitialDataEvent(NAME, n, msg.sender);
    }

    modifier onlyOwner() {
        require(msg.sender == _owner, "ONLY_OWNER");
        _;
    }

    function owner() external view returns (address) {
        return _owner;
    }

    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "BAD_OWNER");
        _owner = newOwner;
    }

    function registerSurveyDao(
        string calldata name,
        address operator,
        address surveyDao,
        address surveyState
    ) external onlyOwner returns (uint256 daoId) {
        require(operator != address(0), "BAD_OPERATOR");
        require(surveyDao != address(0), "BAD_DAO");
        require(surveyState != address(0), "BAD_STATE");

        address mgr = ISurveyDaoStateLike(surveyState).daoManager();
        require(mgr != address(0), "BAD_MANAGER");

        require(ISurveyDaoLike(surveyDao).state() == surveyState, "STATE_MISMATCH");

        require(ISurveyDaoStateLike(surveyState).existsOperator(surveyDao), "DAO_NOT_OPERATOR");
        require(ISurveyDaoStateLike(surveyState).existsOperator(operator), "OP_NOT_OPERATOR");

        daoId = _state.writeAddSurveyDao(
            name,
            mgr,
            operator,
            surveyDao,
            surveyState
        );

        emit SurveyDaoRegistered(daoId, name, mgr, operator, surveyDao, surveyState);
    }

    function surveyDaoCount() external view returns (uint256) {
        return _state.surveyDaoCount();
    }

    function getSurveyDao(uint256 daoId) external view returns (IDaoRegistryStateV1.SurveyDaoInfo memory) {
        return _state.getSurveyDao(daoId);
    }

    function listSurveyDaoIdsByManager(address daoManager) external view returns (uint256[] memory) {
        return _state.listSurveyDaoIdsByManager(daoManager);
    }

    function addressOfExtension(string memory name_) public view returns (address) {
        return addressOf((name_));
    }

    function addressOf(string memory name_) internal view override returns (address) {
        RegisteredExtension memory ext = _state.getExtension((name_));
        return ext.addr;
    }
}
