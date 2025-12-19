
// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.0;

import "../states/access/StateOperator.sol";
import "../states/access/IStateOperator.sol";

import "../library/RegisteredExtensionLib.sol";
import "../library/IndexData.sol";
import "../library/StringToAddress.sol";
import "../library/NamedStrings.sol";
import "../library/ProposalSummaryLib.sol";

interface IDaoRegistryStateV1 is IStateOperator {
    // DaoRegistryStateV1 Functions
    // function addExtension(RegisteredExtension memory ext) external;

    // function updateExtension(RegisteredExtension memory ext) external;

    // function removeExtension(string calldata name) external;

    function getExtension(string memory name) external view returns (RegisteredExtension memory);

    // function listExtensions() external view returns (RegisteredExtension[] memory);

    function addNamedString(string calldata name, string calldata value) external;

    // function existsNamedString(string calldata name) external view returns (bool);

    // function delNamedString(string calldata name) external;

    // function listNamedString() external view returns (string[] memory, string[] memory);

    // function getNamedString(string calldata name) external view returns (string memory);

    function addNamedAddress(string calldata name, address addr) external;

    function getNamedAddress(string calldata name) external view returns (address);

    // function existsExtensionByAddress(address addr) external view returns (bool);

    // function addProposal(ProposalSummary memory p) external returns (bytes memory);

    // function updateProposal(ProposalSummary memory p) external returns (bytes memory);

    // function delProposal(string memory proposalAppName, uint256 proposalId) external;

    // function getProposal(string memory proposalAppName, uint256 proposalId) external view returns (ProposalSummary memory, bool);

    // function listProposals() external view returns (ProposalSummary[] memory);

    function setDaoManagerAddress(address daoManager) external;

    function getDaoManagerAddress() external view returns (address);

    struct SurveyDaoInfo {
        string name;
        address daoManager;
        address operator;
        address surveyDao;
        address surveyState;
        uint64 createdAt;
    }

    function writeAddSurveyDao(
        string calldata name,
        address daoManager,
        address operator,
        address surveyDao,
        address surveyState
    ) external returns (uint256 daoId);

    function surveyDaoCount() external view returns (uint256);
    function getSurveyDao(uint256 daoId) external view returns (SurveyDaoInfo memory);
    function listSurveyDaoIdsByManager(address daoManager) external view returns (uint256[] memory);
}

contract DaoRegistryStateV1 is StateOperator {
    using NamedStrings for NamedStrings.Data;
    NamedStrings.Data private _namedStrings;

    using StringToAddress for StringToAddress.Data;
    StringToAddress.Data private _namedAddresses;

    using RegisteredExtensionLib for RegisteredExtensionLib.Data;
    RegisteredExtensionLib.Data private _extensions;
    RegisteredExtensionLib.Data private _extensionsState;

    using ProposalSummaryLib for ProposalSummaryLib.Data;
    ProposalSummaryLib.Data private _proposals;

    address private _daoManager;

    uint256 private _surveyDaoCount;
    mapping(uint256 => IDaoRegistryStateV1.SurveyDaoInfo) private _surveyDaos;
    mapping(address => uint256[]) private _surveyDaoIdsByManager;
    
    mapping(address => uint256[]) private _daoIdsByManager;
    mapping(uint256 => IDaoRegistryStateV1.SurveyDaoInfo) private _daos;
    
    constructor(address operator) StateOperator(operator) {}

    function setDaoManagerAddress(address daoManager) public canWrite {
        _daoManager = daoManager;
    }

    function getDaoManagerAddress() public view canRead returns (address) {
        return _daoManager;
    }

    function addNamedString(string calldata name, string calldata value) external {
        _namedStrings.add(name, value, true);
    }

    function getNamedAddress(string calldata name) public view canRead returns (address) {
        return _namedAddresses.get(name);
    }

    // function existsNamedString(string calldata name) public view canRead returns (bool) {
    //     return _namedStrings.exists(name);
    // }

    // function delNamedString(string calldata name) public canWrite {
    //     _namedStrings.del(name);
    // }

    // function listNamedString() public view canRead returns (string[] memory, string[] memory) {
    //     return _namedStrings.list();
    // }

    // function getNamedString(string calldata name) public view canRead returns (string memory) {
    //     return _namedStrings.get(name);
    // }

    function addNamedAddress(string calldata name, address addr) public canWrite {
        _namedAddresses.add(name, addr, false);
    }

    // function addExtension(RegisteredExtension memory ext) public canWrite {
    //     _extensions.add(ext);
    // }

    // function updateExtension(RegisteredExtension memory ext) public canWrite {
    //     _extensions.update(ext);
    // }

    // function removeExtension(string calldata name) public canWrite {
    //     _extensions.del(name);
    // }

    function getExtension(string memory name) public view canRead returns (RegisteredExtension memory) {
        return _extensions.get(name);
    }

    function writeAddSurveyDao(
        string calldata name,
        address daoManager,
        address operator,
        address surveyDao,
        address surveyState
    ) external canWrite returns (uint256 daoId) {
        _surveyDaoCount += 1;
        daoId = _surveyDaoCount;

        _surveyDaos[daoId] = IDaoRegistryStateV1.SurveyDaoInfo({
            name: name,
            daoManager: daoManager,
            operator: operator,
            surveyDao: surveyDao,
            surveyState: surveyState,
            createdAt: uint64(block.timestamp)
        });

        _surveyDaoIdsByManager[daoManager].push(daoId);
    }

    function surveyDaoCount() external view canRead returns (uint256) {
        return _surveyDaoCount;
    }

    function getSurveyDao(uint256 daoId)
        external
        view
        canRead
        returns (IDaoRegistryStateV1.SurveyDaoInfo memory)
    {
        return _surveyDaos[daoId];
    }

    function listSurveyDaoIdsByManager(address daoManager)
        external
        view
        canRead
        returns (uint256[] memory)
    {
        return _surveyDaoIdsByManager[daoManager];
    }

    // function listExtensions() public view canRead returns (RegisteredExtension[] memory) {
    //     return _extensions.list();
    // }

    // function addProposal(ProposalSummary memory p) external canWrite returns (bytes memory) {
    //     return _proposals.add(p, false);
    // }

    // function updateProposal(ProposalSummary memory p) external canWrite returns (bytes memory) {
    //     return _proposals.add(p, true);
    // }

    // function delProposal(string memory proposalAppName, uint256 proposalId) external canWrite {
    //     _proposals.del(proposalAppName, proposalId);
    // }

    // function getProposal(
    //     string memory proposalAppName,
    //     uint256 proposalId
    // ) external view canRead returns (ProposalSummary memory, bool) {
    //     return _proposals.get(proposalAppName, proposalId);
    // }

    // function listProposals() external view canRead returns (ProposalSummary[] memory) {
    //     return _proposals.list();
    // }
}
