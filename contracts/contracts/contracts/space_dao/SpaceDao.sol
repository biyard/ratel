// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC20 {
    function transfer(address to, uint256 value) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

contract SpaceDAO {
    address[] private _admins;
    mapping(address => bool) private _isAdmin;

    enum SamplingMode {
        Random,
        Ranking,
        Mixed
    }

    struct SamplingConfig {
        SamplingMode mode;
        uint256 randomCount;
    }

    SamplingConfig private _samplingConfig;
    address[] private _sampled;

    event Sampled(SamplingMode mode, uint256 count);
    event SamplingConfigUpdated(SamplingMode mode, uint256 randomCount);
    event Distributed(address indexed token, uint256 count, uint256 value);

    modifier onlyAdmin() {
        require(_isAdmin[msg.sender], "SpaceDAO: admin only");
        _;
    }

    constructor(address[] memory admins, SamplingConfig memory samplingConfig) {
        require(admins.length >= 3, "SpaceDAO: at least 3 admins required");
        for (uint256 i = 0; i < admins.length; i++) {
            address admin = admins[i];
            require(admin != address(0), "SpaceDAO: invalid admin");
            require(!_isAdmin[admin], "SpaceDAO: duplicate admin");
            _isAdmin[admin] = true;
            _admins.push(admin);
        }
        _samplingConfig = samplingConfig;
    }

    function getAdmins() external view returns (address[] memory) {
        return _admins;
    }

    function getIsAdmin(address account) external view returns (bool) {
        return _isAdmin[account];
    }

    function getSamplingConfig() external view returns (SamplingConfig memory) {
        return _samplingConfig;
    }

    function setSamplingCount(uint256 randomCount) external onlyAdmin {
        require(randomCount > 0, "SpaceDAO: invalid sample count");
        _samplingConfig = SamplingConfig({mode: _samplingConfig.mode, randomCount: randomCount});
        emit SamplingConfigUpdated(_samplingConfig.mode, randomCount);
    }

    function getSampledAddresses() external view returns (address[] memory) {
        return _sampled;
    }

    function sample(address[] calldata candidates)
        external
        onlyAdmin
        returns (address[] memory)
    {
        require(candidates.length > 0, "SpaceDAO: empty candidates");
        require(_samplingConfig.mode == SamplingMode.Random, "SpaceDAO: mode not supported");

        uint256 count = _samplingConfig.randomCount;
        if (count == 0) {
            revert("SpaceDAO: invalid sample count");
        }
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

        delete _sampled;
        for (uint256 i = 0; i < picked.length; i++) {
            _sampled.push(picked[i]);
        }

        emit Sampled(_samplingConfig.mode, picked.length);
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
            require(erc20.transfer(to, value), "SpaceDAO: transfer failed");
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
