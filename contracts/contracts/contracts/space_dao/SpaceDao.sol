// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC20 {
    function transfer(address to, uint256 value) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

contract SpaceDAO {
    address[] public admins;
    mapping(address => bool) public isAdmin;
    IERC20 public immutable usdt;
    uint256 public withdrawalAmount;

    error InvalidAdmin(address admin);
    error InvalidWithdrawalAmount();
    error InvalidTokenAddress();
    error InsufficientBalance(uint256 needed, uint256 available);

    modifier onlyAdmin() {
        require(isAdmin[msg.sender], "SpaceDAO: admin only");
        _;
    }

    constructor(
        address[] memory _admins,
        address _usdt,
        uint256 _withdrawalAmount
    ) {
        if (_usdt == address(0)) {
            revert InvalidTokenAddress();
        }
        if (_withdrawalAmount == 0) {
            revert InvalidWithdrawalAmount();
        }

        usdt = IERC20(_usdt);
        withdrawalAmount = _withdrawalAmount;

        for (uint256 i = 0; i < _admins.length; i++) {
            address admin = _admins[i];
            if (admin == address(0) || isAdmin[admin]) {
                revert InvalidAdmin(admin);
            }
            isAdmin[admin] = true;
            admins.push(admin);
        }

        if (admins.length == 0) {
            revert InvalidAdmin(address(0));
        }
    }

    function deposit(uint256 amount) external {
        require(amount > 0, "SpaceDAO: amount is zero");
        require(usdt.transferFrom(msg.sender, address(this), amount), "SpaceDAO: transfer failed");
    }

    function distributeWithdrawal(address[] calldata recipients) external onlyAdmin {
        uint256 count = recipients.length;
        require(count > 0, "SpaceDAO: empty recipients");

        uint256 totalNeeded = withdrawalAmount * count;
        uint256 balance = usdt.balanceOf(address(this));
        if (balance < totalNeeded) {
            revert InsufficientBalance(totalNeeded, balance);
        }

        for (uint256 i = 0; i < count; i++) {
            address to = recipients[i];
            if (to == address(0)) {
                revert InvalidAdmin(to);
            }
            require(usdt.transfer(to, withdrawalAmount), "SpaceDAO: transfer failed");
        }
    }
}
