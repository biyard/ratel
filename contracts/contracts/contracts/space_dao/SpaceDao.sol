// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC20 {
    function transfer(address to, uint256 value) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

contract SpaceDAO {
    address[] private _admins;
    mapping(address => bool) private _isAdmin;
    // 예치자 여부
    mapping(address => bool) private _isDepositor;
    // 예치자가 예치한 총 USDT
    mapping(address => uint256) private _depositorDeposits;
    // 예치자가 출금해간 누적 금액
    mapping(address => uint256) private _withdrawnByDepositor;
    IERC20 private _usdt;
    uint256 private _withdrawalAmount;
    // 예치한 총 USDT 누적액
    uint256 private _totalDepositorDeposited;
    // 분배된 총 Reward 누적액
    uint256 private _totalRewardDistributed;
    uint256 public constant REQUIRED_APPROVALS = 2;
    uint256 private _depositorCount;

    struct ShareWithdrawProposal {
        address proposer;
        uint256 amount;
        uint256 approvalCount;
        bool executed;
        mapping(address => bool) approvedBy;
    }

    ShareWithdrawProposal[] private _shareWithdrawProposals;

    modifier onlyAdmin() {
        require(_isAdmin[msg.sender], "SpaceDAO: admin only");
        _;
    }

    modifier onlyDepositor() {
        require(_isDepositor[msg.sender], "SpaceDAO: depositor only");
        _;
    }

    constructor(
        address[] memory admins,
        address usdt,
        uint256 withdrawalAmount
    ) {
        require(usdt != address(0), "SpaceDAO: invalid token address");
        require(withdrawalAmount > 0, "SpaceDAO: invalid withdrawal amount");
        require(admins.length >= 3, "SpaceDAO: at least 3 admins required");

        _usdt = IERC20(usdt);
        _withdrawalAmount = withdrawalAmount;

        for (uint256 i = 0; i < admins.length; i++) {
            address admin = admins[i];
            require(admin != address(0), "SpaceDAO: invalid admin");
            require(!_isAdmin[admin], "SpaceDAO: duplicate admin");
            _isAdmin[admin] = true;
            _admins.push(admin);
        }
    }

    function deposit(uint256 amount) external {
        require(amount > 0, "SpaceDAO: amount is zero");
        require(_usdt.transferFrom(msg.sender, address(this), amount), "SpaceDAO: transfer failed");
        if (!_isDepositor[msg.sender]) {
            _isDepositor[msg.sender] = true;
            _depositorCount += 1;
        }
        _depositorDeposits[msg.sender] += amount;
        _totalDepositorDeposited += amount;
    }

    function getAdmins() external view returns (address[] memory) {
        return _admins;
    }

    function getIsAdmin(address account) external view returns (bool) {
        return _isAdmin[account];
    }

                 function getUsdt() external view returns (address) {
        return address(_usdt);
    }

    function getWithdrawalAmount() external view returns (uint256) {
        return _withdrawalAmount;
    }

    function getAvailableShare(address depositor) public view returns (uint256) {
        uint256 depositorDeposit = _depositorDeposits[depositor];
        if (depositorDeposit == 0) {
            return 0;
        }
        if (_totalDepositorDeposited == 0 || _totalRewardDistributed >= _totalDepositorDeposited) {
            return 0;
        }
        uint256 remainingPool = _totalDepositorDeposited - _totalRewardDistributed;
        uint256 maxShare =
            (remainingPool * depositorDeposit) / _totalDepositorDeposited;
        uint256 withdrawn = _withdrawnByDepositor[depositor];
        if (withdrawn >= maxShare) {
            return 0;
        }
        return maxShare - withdrawn;
    }

    function getShareWithdrawProposal(uint256 id)
        external
        view
        returns (address proposer, uint256 amount, uint256 approvals, bool executed)
    {
        ShareWithdrawProposal storage p = _shareWithdrawProposals[id];
        return (p.proposer, p.amount, p.approvalCount, p.executed);
    }

    function proposeShareWithdrawal(uint256 amount) external onlyDepositor {
        require(amount > 0, "SpaceDAO: amount is zero");
        require(_depositorDeposits[msg.sender] > 0, "SpaceDAO: no deposit");

        uint256 available = getAvailableShare(msg.sender);
        require(amount <= available, "SpaceDAO: exceeds share");

        uint256 id = _shareWithdrawProposals.length;
        _shareWithdrawProposals.push();
        ShareWithdrawProposal storage p = _shareWithdrawProposals[id];
        p.proposer = msg.sender;
        p.amount = amount;
        p.approvedBy[msg.sender] = true;
        p.approvalCount = 1;
    }

    function approveShareWithdrawal(uint256 id) external onlyDepositor {
        ShareWithdrawProposal storage p = _shareWithdrawProposals[id];
        require(!p.executed, "SpaceDAO: already executed");
        require(!p.approvedBy[msg.sender], "SpaceDAO: already approved");

        p.approvedBy[msg.sender] = true;
        p.approvalCount += 1;

        uint256 quorum = (_depositorCount / 2) + 1;
        if (p.approvalCount >= quorum) {
            _executeShareWithdrawal(p);
        }
    }

    function _executeShareWithdrawal(ShareWithdrawProposal storage p) internal {
        require(!p.executed, "SpaceDAO: already executed");
        require(_usdt.balanceOf(address(this)) >= p.amount, "SpaceDAO: insufficient balance");

        uint256 available = getAvailableShare(p.proposer);
        require(p.amount <= available, "SpaceDAO: exceeds share");

        p.executed = true;
        _withdrawnByDepositor[p.proposer] += p.amount;
        require(_usdt.transfer(p.proposer, p.amount), "SpaceDAO: transfer failed");
    }

    function distributeWithdrawal(address[] calldata recipients) external onlyAdmin {
        uint256 count = recipients.length;
        require(count > 0, "SpaceDAO: empty recipients");

        uint256 totalNeeded = _withdrawalAmount * count;
        uint256 balance = _usdt.balanceOf(address(this));
        require(
            balance >= totalNeeded,
            "SpaceDAO: insufficient balance"
        );

        for (uint256 i = 0; i < count; i++) {
            address to = recipients[i];
            require(to != address(0), "SpaceDAO: invalid recipient");
            require(_usdt.transfer(to, _withdrawalAmount), "SpaceDAO: transfer failed");
        }

        _totalRewardDistributed += totalNeeded;
    }

    function getBalance() external view returns (uint256) {
        return _usdt.balanceOf(address(this));
    }

    function getDepositorDeposit(address depositor) external view returns (uint256) {
        return _depositorDeposits[depositor];
    }

    function getTotalDepositorDeposited() external view returns (uint256) {
        return _totalDepositorDeposited;
    }

    function getTotalRewardDistributed() external view returns (uint256) {
        return _totalRewardDistributed;
    }

    function getDepositorCount() external view returns (uint256) {
        return _depositorCount;
    }

    function setWithdrawalAmount(uint256 amount) external onlyAdmin {
        require(amount > 0, "SpaceDAO: invalid withdrawal amount");
        _withdrawalAmount = amount;
    }

    function addAdmin(address admin) external onlyAdmin {
        require(admin != address(0), "SpaceDAO: invalid admin");
        require(!_isAdmin[admin], "SpaceDAO: duplicate admin");
        _isAdmin[admin] = true;
        _admins.push(admin);
    }

    function setUsdtAddress(address usdt) external onlyAdmin {
        require(usdt != address(0), "Invalid USDT Address");
        _usdt = IERC20(usdt);
    }
}
