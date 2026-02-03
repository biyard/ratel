// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

contract TeamDAO is ReentrancyGuard {
    // --- Custom Errors ---
    error TeamDAO__NotAdmin();
    error TeamDAO__InvalidPairCount();
    error TeamDAO__InvalidTokenAddress();
    error TeamDAO__ProposalAlreadyExecuted();
    error TeamDAO__AlreadyApproved();
    error TeamDAO__TransferFailed();
    error TeamDAO__InsufficientAdmins();
    error TeamDAO__DuplicateAdmin();
    error TeamDAO__ZeroAddress();

    // --- Enums & Structs ---
    enum ProposalStatus {
        Pending,
        Executed
    }

    struct TransferPair {
        address recipient;
        uint256 amount;
    }

    struct Proposal {
        uint256 id;
        address token;
        TransferPair[] pairs;
        uint256 approvalCount;
        ProposalStatus status;
        mapping(address => bool) approvedBy;
    }

    // --- State Variables ---
    address[] public admins;
    mapping(address => bool) public isAdmin;
    Proposal[] public proposals;

    // --- Events ---
    event Initialized(address[] admins);
    event ProposalCreated(
        uint256 indexed id,
        address indexed proposer,
        uint256 count
    );
    event Approved(uint256 indexed id, address indexed approver);
    event BatchExecuted(uint256 indexed id);

    // --- Modifiers ---
    modifier onlyAdmin() {
        if (!isAdmin[msg.sender]) revert TeamDAO__NotAdmin();
        _;
    }

    constructor(address[] memory _admins) {
        if (_admins.length < 3) revert TeamDAO__InsufficientAdmins();

        for (uint i = 0; i < _admins.length; i++) {
            address admin = _admins[i];
            if (admin == address(0)) revert TeamDAO__ZeroAddress();
            if (isAdmin[admin]) revert TeamDAO__DuplicateAdmin();

            admins.push(admin);
            isAdmin[admin] = true;
        }

        emit Initialized(_admins);
    }

    function getRequiredApprovals() public view returns (uint256) {
        return (admins.length / 2) + 1;
    }

    /**
     * @dev Create a new batch transfer proposal
     */
    function proposeBatch(
        address _token,
        TransferPair[] calldata _pairs
    ) external onlyAdmin {
        if (_pairs.length == 0 || _pairs.length > 100)
            revert TeamDAO__InvalidPairCount();
        if (_token == address(0)) revert TeamDAO__InvalidTokenAddress();

        uint256 id = proposals.length;
        Proposal storage p = proposals.push();

        p.id = id;
        p.token = _token;
        p.status = ProposalStatus.Pending;
        p.approvalCount = 1;
        p.approvedBy[msg.sender] = true;

        for (uint i = 0; i < _pairs.length; i++) {
            p.pairs.push(_pairs[i]);
        }

        emit ProposalCreated(id, msg.sender, _pairs.length);
    }

    /**
     * @dev Approve and execute a proposal
     */
    function approveAndExecute(uint256 _id) external onlyAdmin {
        Proposal storage p = proposals[_id];

        if (p.status != ProposalStatus.Pending)
            revert TeamDAO__ProposalAlreadyExecuted();
        if (p.approvedBy[msg.sender]) revert TeamDAO__AlreadyApproved();

        p.approvedBy[msg.sender] = true;
        p.approvalCount++;

        emit Approved(_id, msg.sender);

        if (p.approvalCount >= getRequiredApprovals()) {
            _executeBatch(p);
        }
    }

    /**
     * @dev Execute a proposal (Internal function)
     */
    function _executeBatch(Proposal storage p) internal nonReentrant {
        p.status = ProposalStatus.Executed;

        IERC20 token = IERC20(p.token);
        uint256 len = p.pairs.length;

        for (uint i = 0; i < len; i++) {
            TransferPair storage pair = p.pairs[i];
            if (!token.transfer(pair.recipient, pair.amount)) {
                revert TeamDAO__TransferFailed();
            }
        }

        emit BatchExecuted(p.id);
    }

    // --- View Functions ---

    function getProposalInfo(
        uint256 _id
    )
        external
        view
        returns (
            address token,
            uint256 count,
            uint256 approvals,
            ProposalStatus status
        )
    {
        Proposal storage p = proposals[_id];
        return (p.token, p.pairs.length, p.approvalCount, p.status);
    }

    function checkAdmin(address _user) external view returns (bool) {
        return isAdmin[_user];
    }
}
