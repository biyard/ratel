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

    event RewardDistributionSelected(RewardDistributionMode mode, uint256 count);
    event RewardDistributionConfigUpdated(RewardDistributionMode mode, uint256 numOfTargets);
    event Distributed(address indexed token, uint256 count, uint256 value);

    modifier onlyAdmin() {
        require(_isAdmin[msg.sender], "SpaceDAO: admin only");
        _;
    }

    constructor(address[] memory admins, RewardDistributionConfig memory rewardDistributionConfig) {
        require(admins.length >= 3, "SpaceDAO: at least 3 admins required");
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

        emit RewardDistributionSelected(_rewardDistributionConfig.mode, picked.length);
        return picked;
    }

    function distribute(
        address token,
        address[] calldata recipients,
        uint256 value
    ) external onlyAdmin {
        require(token != address(0), "SpaceDAO: invalid token");
        require(recipients.length > 0, "SpaceDAO: empty recipients");
        require(value > 0, "SpaceDAO: invalid value");

        IERC20 erc20 = IERC20(token);
        uint256 total = value * recipients.length;
        require(erc20.balanceOf(address(this)) >= total, "SpaceDAO: insufficient balance");

        for (uint256 i = 0; i < recipients.length; i++) {
            address to = recipients[i];
            require(to != address(0), "SpaceDAO: invalid recipient");
            require(_isRewardRecipient[to], "SpaceDAO: not selected");
            require(!_isRewarded[to], "SpaceDAO: reward finished");
            require(erc20.transfer(to, value), "SpaceDAO: transfer failed");
            _isRewarded[to] = true;
        }

        emit Distributed(token, recipients.length, value);
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
