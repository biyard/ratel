// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/proxy/utils/Initializable.sol";

interface ISpaceDAO {
    function executeCall(address target, uint256 value, bytes calldata data) external returns (bytes memory);
    function checkAdmin(address user) external view returns (bool);
}

contract RewardExtension is Initializable {
    
    ISpaceDAO public dao;
    uint256 public constant REQUIRED_APPROVALS = 2;

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

    event ProposalCreated(uint256 indexed id, address indexed proposer, uint256 count);
    event Approved(uint256 indexed id, address indexed approver);
    event BatchExecuted(uint256 indexed id);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() { _disableInitializers(); }

    function initialize(address _dao) external initializer {
        dao = ISpaceDAO(_dao);
    }

    modifier onlyAdmin() {
        require(dao.checkAdmin(msg.sender), "RewardExt: Not a DAO admin");
        _;
    }

    // --- 1. Propose ---
    function proposeBatch(
        address _token,
        TransferPair[] calldata _pairs
    ) external onlyAdmin {
        require(_pairs.length > 0 && _pairs.length <= 100, "Invalid pairs");

        uint256 id = proposals.length;
        proposals.push();
        Proposal storage p = proposals[id];

        p.id = id;
        p.token = _token;
        for(uint i=0; i < _pairs.length; i++) {
            p.pairs.push(_pairs[i]);
        }
        
        // Auto-approve creator
        p.approvedBy[msg.sender] = true;
        p.approvalCount = 1;

        emit ProposalCreated(id, msg.sender, _pairs.length);
    }

    // --- 2. Approve & Execute ---
    function approveAndExecute(uint256 _id) external onlyAdmin {
        Proposal storage p = proposals[_id];
        require(!p.executed, "Already executed");
        require(!p.approvedBy[msg.sender], "Already approved");

        p.approvedBy[msg.sender] = true;
        p.approvalCount++;
        emit Approved(_id, msg.sender);

        if (p.approvalCount >= REQUIRED_APPROVALS) {
            _executeBatch(p);
        }
    }

    function _executeBatch(Proposal storage p) internal {
        p.executed = true;

        for (uint i = 0; i < p.pairs.length; i++) {
            address to = p.pairs[i].recipient;
            uint256 amt = p.pairs[i].amount;
            
            bytes memory data;
            address target;
            uint256 val = 0;

            if (p.token == address(0)) {
                // ETH
                target = to;
                val = amt;
                data = ""; 
            } else {
                // ERC20
                target = p.token;
                data = abi.encodeWithSelector(IERC20.transfer.selector, to, amt);
            }

            // Call DAO
            dao.executeCall(target, val, data);
        }

        emit BatchExecuted(p.id);
    }
    
    function getProposalInfo(uint256 _id) external view returns (address token, uint256 count, uint256 approvals, bool executed) {
        Proposal storage p = proposals[_id];
        return (p.token, p.pairs.length, p.approvalCount, p.executed);
    }
}