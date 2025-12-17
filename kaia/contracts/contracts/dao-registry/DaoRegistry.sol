// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.0;

import "./DaoRegistryStateV1.sol";
import "../VersionManager.sol";
import "../library/BytesLib.sol";
import "../standards/eip/EIP2771.sol";
import "../standards/eip/NativeMetaTransaction.sol";

contract DaoRegistry is VersionManager("v2.0"), EIP2771, NativeMetaTransaction {
    IDaoRegistryStateV1 private _state;
    AddressArray.Data _activityHooks;
    bool private _ready;
    string constant NAME = "NAME";
    address private _prev;

    event setReadyEvent(bool check);
    event changeProposalEvent(string proposalAppName, uint256 proposalId);
    event changeActivityHookEvent(address addr);
    event upgradeEvent(address newRegistry);
    event withdrawalEvent(address addr);
    event upgradeHookEvent(address state, bool ready);
    event changeInitialDataEvent(string name, string value, address daoManagerAddress);

    constructor(string memory n, address state, address operator) EIP2771(address(0)) {
        _initializeEIP712(n);

        if (state != address(0)) {
            _state = IDaoRegistryStateV1(state);
            _state.addOperator(address(this));
            _state.addOperator(operator);
            _state.addNamedString(NAME, n);
            _state.setStateReady(true);
            _state.setDaoManagerAddress(msg.sender);
        }

        emit changeInitialDataEvent(NAME, n, msg.sender);
    }

    /// @notice This function calls extensions as registry forwarder.
    /// @dev If you use this function to call extension, must verirfy proper permission of the transaction in advance.
    /// @param extension is an address of callee
    /// @param functionSignature is encoded signature such as `abi.encodeWithSignature("funcName(address,uint256)",addr,1)`
    function callAsRegistry(address extension, bytes memory functionSignature) internal {
        bytes memory d = abi.encodePacked(functionSignature, address(this));
        (bool success, ) = extension.call(d);

        require(success, string(abi.encodePacked("failed to call ", BytesLib.toString(abi.encodePacked(extension)))));
    }

    // function upgrade(address newRegistry) public hasPermission(ACTIVITY_REGISTRY_UPGRADE_FLAG) {
    //     _state.migrate(newRegistry);
    //     RegisteredExtension[] memory e = _state.listExtensions();

    //     for (uint i = 0; i < e.length; i++) {
    //         (bool succ, ) = e[i].addr.call(
    //             abi.encodePacked(abi.encodeWithSignature("upgradeRegistry(address)", newRegistry), address(this))
    //         );
    //         require(succ, string(abi.encodePacked("failed to call ", e[i].name)));
    //     }
    //     (bool success, ) = newRegistry.call(abi.encodeWithSignature("upgradeHook(address)", address(_state)));
    //     require(success, "failed to pass state to a new registry; check if you implement `upgradeHook(address)` function");
    
    //     emit upgradeEvent(newRegistry);
    // }

    // function withdrawal(address payable addr) public hasPermission(ACTIVITY_WITHDRAWAL_FLAG) {
    //     uint256 balance = address(this).balance;
    //     addr.transfer(balance);
    //     emit withdrawalEvent(addr);
    // }

    function getStateAddr() external view returns (address[] memory) {
        address[] memory addrs = new address[](2);
        addrs[0] = _prev;
        addrs[1] = address(_state);
        return addrs;
    }

    function upgradeHook(address state) external {
        require(!_ready, "it was already initialized");
        _ready = true;
        _state = IDaoRegistryStateV1(state);

        emit upgradeHookEvent(state, _ready);
    }

    function setReady(bool check) external {
        require(!_ready, "it was already ready");
        _ready = check;
        emit setReadyEvent(check);
    }

    function name() external view returns (string memory) {
        return _state.getNamedString(NAME);
    }

    function getBalance() external view returns (uint256) {
        return address(this).balance;
    }

    function addressOfExtension(string memory name_) public view returns (address) {
        return addressOf(name_);
    }

    function addressOf(string memory name_) internal view override returns (address) {
        RegisteredExtension memory ext = _state.getExtension(name_);

        return ext.addr;
    }

    function ready() external view returns (bool) {
        return _ready;
    }

    function registerActivityHook(address addr) internal {
        AddressArray.add(_activityHooks, addr, false);

        emit changeActivityHookEvent(addr);
    }

    function deregisterActivityHook(address addr) internal {
        if (AddressArray.exists(_activityHooks, addr)) {
            AddressArray.del(_activityHooks, addr);
        }

        emit changeActivityHookEvent(addr);
    }

    function listProposals() external view returns (ProposalSummary[] memory) {
        return _state.listProposals();
    }

    function getProposal(string memory proposalAppName, uint256 proposalId) external view returns (ProposalSummary memory) {
        (ProposalSummary memory pro, ) = _state.getProposal(proposalAppName, proposalId);
        return pro;
    }

    function submitProposal(ProposalSummary memory proposal) external {
        _state.addProposal(proposal);
        emit changeProposalEvent(proposal.proposalAppName, proposal.proposalId);
    }

    function voteProposal(string memory proposalAppName, uint256 proposalId) external {
        (ProposalSummary memory proposal, bool exists) = _state.getProposal(proposalAppName, proposalId);
        require(exists, "unknown proposal");
        proposal.numberOfVotes = proposal.numberOfVotes + 1;
        _state.updateProposal(proposal);
        emit changeProposalEvent(proposalAppName, proposalId);
    }

    function finishVoting(string memory proposalAppName, uint256 proposalId, uint16 voteStatus) external {
        (ProposalSummary memory proposal, bool exists) = _state.getProposal(proposalAppName, proposalId);
        require(exists, "unknown proposal");
        proposal.voteStatus = voteStatus;
        _state.updateProposal(proposal);
        emit changeProposalEvent(proposalAppName, proposalId);
    }

    modifier checkDaoManager() {
        require(_state.getDaoManagerAddress() == msg.sender, "only daoManager can call Function");
        _;
    }

    modifier shouldRegisteredApp() {
        bool exists = _state.existsExtensionByAddress(msg.sender);
        require(!_ready || exists, "unknown app");
        _;
    }
}