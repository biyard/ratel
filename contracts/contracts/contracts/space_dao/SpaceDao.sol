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
    IERC20 private _usdt;
    uint256 private _withdrawalAmount;

    modifier onlyAdmin() {
        require(_isAdmin[msg.sender], "SpaceDAO: admin only");
        _;
    }

    constructor(
        address[] memory admins,
        address usdt,
        uint256 withdrawalAmount
    ) {
        require(usdt != address(0), "SpaceDAO: invalid token address");
        require(withdrawalAmount > 0, "SpaceDAO: invalid withdrawal amount");
        require(_admins.length >= 3, "SpaceDAO: at least 3 admins required");

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
    }

    function getBalance() external view returns (uint256) {
        return _usdt.balanceOf(address(this));
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
