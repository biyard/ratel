// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

contract TeamDAO is ReentrancyGuard {

    // --- State Variables ---
    bool public isDaoActive;

    address[] public admins;
    mapping(address => bool) public isAdmin;

    // Reward proposal structures
    struct TransferPair {
        address recipient;
        uint256 amount;
    }

    struct Proposal {
        uint256 id;
        address token;
        TransferPair[] pairs;
        uint256 approvalCount;
        bool executed;
        mapping(address => bool) approvedBy;
    }

    Proposal[] public proposals;

    // --- Events ---
    event Initialized(address[] admins);
    event Received(address indexed sender, uint256 amount);
    event ProposalCreated(uint256 indexed id, address indexed proposer, uint256 count);
    event Approved(uint256 indexed id, address indexed approver);
    event BatchExecuted(uint256 indexed id);

    // --- Modifiers ---
    modifier onlyActive() {
        require(isDaoActive, "TeamDAO: DAO is inactive");
        _;
    }

    modifier onlyAdmin() {
        require(isAdmin[msg.sender], "TeamDAO: Not an admin");
        _;
    }

    /**
     * @dev Constructor initializes the DAO with admins.
     */
    constructor(address[] memory _admins) {
        require(_admins.length >= 3, "TeamDAO: Must have at least 3 admins");

        for (uint i = 0; i < _admins.length; i++) {
            admins.push(_admins[i]);
            isAdmin[_admins[i]] = true;
        }

        isDaoActive = true;

        emit Initialized(_admins);
    }

    receive() external payable {
        emit Received(msg.sender, msg.value);
    }

    /**
     * @dev Get the number of approvals required for proposals (majority).
     */
    function getRequiredApprovals() public view returns (uint256) {
        return (admins.length / 2) + 1;
    }

    /**
     * @dev Propose a batch of ERC20 token transfers.
     */
    function proposeBatch(
        address _token,
        TransferPair[] calldata _pairs
    ) external onlyAdmin {
        require(_pairs.length > 0 && _pairs.length <= 100, "TeamDAO: Invalid pairs count");
        require(_token != address(0), "TeamDAO: Token address cannot be zero");

        uint256 id = proposals.length;
        proposals.push();
        Proposal storage p = proposals[id];

        p.id = id;
        p.token = _token;
        for(uint i = 0; i < _pairs.length; i++) {
            p.pairs.push(_pairs[i]);
        }

        // Auto-approve creator
        p.approvedBy[msg.sender] = true;
        p.approvalCount = 1;

        emit ProposalCreated(id, msg.sender, _pairs.length);
    }

    /**
     * @dev Approve a proposal and execute if majority is reached.
     */
    function approveAndExecute(uint256 _id) external onlyAdmin {
        Proposal storage p = proposals[_id];
        require(!p.executed, "TeamDAO: Already executed");
        require(!p.approvedBy[msg.sender], "TeamDAO: Already approved");

        p.approvedBy[msg.sender] = true;
        p.approvalCount++;
        emit Approved(_id, msg.sender);

        if (p.approvalCount >= getRequiredApprovals()) {
            _executeBatch(p);
        }
    }

    /**
     * @dev Execute batch ERC20 transfers.
     */
    function _executeBatch(Proposal storage p) internal nonReentrant {
        p.executed = true;

        IERC20 token = IERC20(p.token);
        for (uint i = 0; i < p.pairs.length; i++) {
            address to = p.pairs[i].recipient;
            uint256 amt = p.pairs[i].amount;

            require(token.transfer(to, amt), "TeamDAO: Token transfer failed");
        }

        emit BatchExecuted(p.id);
    }

    /**
     * @dev Get proposal information.
     */
    function getProposalInfo(uint256 _id) external view returns (
        address token,
        uint256 count,
        uint256 approvals,
        bool executed
    ) {
        Proposal storage p = proposals[_id];
        return (p.token, p.pairs.length, p.approvalCount, p.executed);
    }

    /**
     * @dev Check if an address is an admin.
     */
    function checkAdmin(address _user) external view returns (bool) {
        return isAdmin[_user];
    }
}