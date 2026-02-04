// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC20 {
    function transfer(address to, uint256 value) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

contract SpaceDAO {
    address[] private _admins;
    mapping(address => bool) private _isAdmin;

    enum RewardDistributionMode {
        Random,
        Ranking,
        Mixed
    }

    struct RewardDistributionConfig {
        RewardDistributionMode mode;
        uint256 numOfTargets;
    }

    RewardDistributionConfig private _rewardDistributionConfig;
    address[] private _rewardRecipients;
    mapping(address => bool) private _isRewardRecipient;
    mapping(address => bool) private _isRewarded;
    uint256 private _rewardEpoch;
    mapping(address => uint256) private _rewardAmountByToken;
    mapping(address => uint256) private _rewardAmountEpoch;

    event RewardDistributionSelected(RewardDistributionMode mode, uint256 count);
    event RewardDistributionConfigUpdated(RewardDistributionMode mode, uint256 numOfTargets);
    event RewardClaimed(address indexed token, address indexed recipient, uint256 value);

    modifier onlyAdmin() {
        require(_isAdmin[msg.sender], "SpaceDAO: admin only");
        _;
    }

    constructor(address[] memory admins, RewardDistributionConfig memory rewardDistributionConfig) {
        require(admins.length > 0, "SpaceDAO: empty admins");
        for (uint256 i = 0; i < admins.length; i++) {
            address admin = admins[i];
            require(admin != address(0), "SpaceDAO: invalid admin");
            require(!_isAdmin[admin], "SpaceDAO: duplicate admin");
            _isAdmin[admin] = true;
            _admins.push(admin);
        }
        _rewardDistributionConfig = rewardDistributionConfig;
    }

    function getAdmins() external view returns (address[] memory) {
        return _admins;
    }

    function getIsAdmin(address account) external view returns (bool) {
        return _isAdmin[account];
    }

    function getRewardDistributionConfig() external view returns (RewardDistributionConfig memory) {
        return _rewardDistributionConfig;
    }

    function setRewardRecipientCount(uint256 numOfTargets) external onlyAdmin {
        require(numOfTargets > 0, "SpaceDAO: invalid recipient count");
        _rewardDistributionConfig = RewardDistributionConfig({
            mode: _rewardDistributionConfig.mode,
            numOfTargets: numOfTargets
        });
        emit RewardDistributionConfigUpdated(_rewardDistributionConfig.mode, numOfTargets);
    }

    function getRewardRecipients() external view returns (address[] memory) {
        return _rewardRecipients;
    }

    function isRewardRecipient(address account) external view returns (bool) {
        return _isRewardRecipient[account];
    }

    function isRewarded(address account) external view returns (bool) {
        return _isRewarded[account];
    }

    function getClaimAmount(address token) external view returns (uint256) {
        if (token == address(0)) {
            return 0;
        }
        uint256 count = _rewardRecipients.length;
        if (count == 0) {
            return 0;
        }
        if (_rewardAmountEpoch[token] == _rewardEpoch) {
            return _rewardAmountByToken[token];
        }
        uint256 balance = IERC20(token).balanceOf(address(this));
        return balance / count;
    }

    function selectRewardRecipients(address[] calldata candidates)
        external
        onlyAdmin
        returns (address[] memory)
    {
        require(candidates.length > 0, "SpaceDAO: empty candidates");
        require(
            _rewardDistributionConfig.mode == RewardDistributionMode.Random,
            "SpaceDAO: mode not supported"
        );

        uint256 count = _rewardDistributionConfig.numOfTargets;
        require(count > 0, "SpaceDAO: invalid recipient count");

        if (count > candidates.length) {
            count = candidates.length;
        }

        address[] memory pool = new address[](candidates.length);
        for (uint256 i = 0; i < candidates.length; i++) {
            pool[i] = candidates[i];
        }

        address[] memory picked = new address[](count);
        for (uint256 i = 0; i < count; i++) {
            uint256 j = _randomIndex(pool.length - i, i) + i;
            address tmp = pool[i];
            pool[i] = pool[j];
            pool[j] = tmp;
            picked[i] = pool[i];
        }

        for (uint256 i = 0; i < _rewardRecipients.length; i++) {
            address prev = _rewardRecipients[i];
            _isRewardRecipient[prev] = false;
            _isRewarded[prev] = false;
        }
        delete _rewardRecipients;
        for (uint256 i = 0; i < picked.length; i++) {
            address pickedAddr = picked[i];
            require(pickedAddr != address(0), "SpaceDAO: invalid recipient");
            _isRewardRecipient[pickedAddr] = true;
            _isRewarded[pickedAddr] = false;
            _rewardRecipients.push(pickedAddr);
        }
        _rewardEpoch += 1;

        emit RewardDistributionSelected(_rewardDistributionConfig.mode, picked.length);
        return picked;
    }

    function claimReward(address token) external {
        require(token != address(0), "SpaceDAO: invalid token");

        address recipient = msg.sender;
        require(_isRewardRecipient[recipient], "SpaceDAO: not selected");
        require(!_isRewarded[recipient], "SpaceDAO: reward finished");

        IERC20 erc20 = IERC20(token);
        uint256 count = _rewardRecipients.length;
        require(count > 0, "SpaceDAO: invalid recipient count");
        uint256 value = _rewardAmountByToken[token];
        if (_rewardAmountEpoch[token] != _rewardEpoch) {
            uint256 balance = erc20.balanceOf(address(this));
            value = balance / count;
            require(value > 0, "SpaceDAO: invalid value");
            _rewardAmountByToken[token] = value;
            _rewardAmountEpoch[token] = _rewardEpoch;
        }
        require(erc20.balanceOf(address(this)) >= value, "SpaceDAO: insufficient balance");

        require(erc20.transfer(recipient, value), "SpaceDAO: transfer failed");
        _isRewarded[recipient] = true;

        emit RewardClaimed(token, recipient, value);
    }

    function _randomIndex(uint256 range, uint256 nonce) internal view returns (uint256) {
        if (range == 0) {
            return 0;
        }
        return uint256(
            keccak256(abi.encodePacked(block.prevrandao, block.timestamp, msg.sender, nonce))
        ) % range;
    }
}
