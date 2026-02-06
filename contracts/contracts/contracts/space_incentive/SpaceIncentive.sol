// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC20 {
    function transfer(address to, uint256 value) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

contract SpaceIncentive {
    address[] private _admins;
    mapping(address => bool) private _isAdmin;

    enum IncentiveDistributionMode {
        Random,
        Ranking,
        Mixed
    }

    struct IncentiveDistributionConfig {
        IncentiveDistributionMode mode;
        uint256 numOfTargets;
        uint16 rankingBps;
    }

    IncentiveDistributionConfig private _incentiveDistributionConfig;
    address[] private _incentiveRecipients;
    mapping(address => bool) private _isIncentiveRecipient;
    mapping(address => bool) private _isIncentiveClaimed;
    uint256 private _incentiveEpoch;
    mapping(address => uint256) private _incentiveAmountByToken;
    mapping(address => uint256) private _incentiveAmountEpoch;

    event IncentiveDistributionSelected(IncentiveDistributionMode mode, uint256 count);
    event IncentiveDistributionConfigUpdated(
        IncentiveDistributionMode mode,
        uint256 numOfTargets,
        uint16 rankingBps
    );
    event IncentiveClaimed(address indexed token, address indexed recipient, uint256 value);

    modifier onlyAdmin() {
        require(_isAdmin[msg.sender], "SpaceIncentive: admin only");
        _;
    }

    constructor(address[] memory admins, IncentiveDistributionConfig memory incentiveDistributionConfig) {
        require(admins.length > 0, "SpaceIncentive: empty admins");
        require(
            incentiveDistributionConfig.rankingBps <= 10000,
            "SpaceIncentive: invalid ranking bps"
        );
        for (uint256 i = 0; i < admins.length; i++) {
            address admin = admins[i];
            require(admin != address(0), "SpaceIncentive: invalid admin");
            require(!_isAdmin[admin], "SpaceIncentive: duplicate admin");
            _isAdmin[admin] = true;
            _admins.push(admin);
        }
        _incentiveDistributionConfig = incentiveDistributionConfig;
    }

    function getAdmins() external view returns (address[] memory) {
        return _admins;
    }

    function getIsAdmin(address account) external view returns (bool) {
        return _isAdmin[account];
    }

    function getIncentiveDistributionConfig() external view returns (IncentiveDistributionConfig memory) {
        return _incentiveDistributionConfig;
    }

    function setIncentiveRecipientCount(uint256 numOfTargets) external onlyAdmin {
        require(numOfTargets > 0, "SpaceIncentive: invalid recipient count");
        _incentiveDistributionConfig = IncentiveDistributionConfig({
            mode: _incentiveDistributionConfig.mode,
            numOfTargets: numOfTargets,
            rankingBps: _incentiveDistributionConfig.rankingBps
        });
        emit IncentiveDistributionConfigUpdated(
            _incentiveDistributionConfig.mode,
            numOfTargets,
            _incentiveDistributionConfig.rankingBps
        );
    }

    function setIncentiveRankingBps(uint16 rankingBps) external onlyAdmin {
        require(rankingBps <= 10000, "SpaceIncentive: invalid ranking bps");
        _incentiveDistributionConfig = IncentiveDistributionConfig({
            mode: _incentiveDistributionConfig.mode,
            numOfTargets: _incentiveDistributionConfig.numOfTargets,
            rankingBps: rankingBps
        });
        emit IncentiveDistributionConfigUpdated(
            _incentiveDistributionConfig.mode,
            _incentiveDistributionConfig.numOfTargets,
            rankingBps
        );
    }

    function getIncentiveRecipients() external view returns (address[] memory) {
        return _incentiveRecipients;
    }

    function isIncentiveRecipient(address account) external view returns (bool) {
        return _isIncentiveRecipient[account];
    }

    function isIncentiveClaimed(address account) external view returns (bool) {
        return _isIncentiveClaimed[account];
    }

    function getIncentiveAmount(address token) external view returns (uint256) {
        if (token == address(0)) {
            return 0;
        }
        uint256 count = _incentiveRecipients.length;
        if (count == 0) {
            return 0;
        }
        if (_incentiveAmountEpoch[token] == _incentiveEpoch) {
            return _incentiveAmountByToken[token];
        }
        uint256 balance = IERC20(token).balanceOf(address(this));
        return balance / count;
    }

    function selectIncentiveRecipients(address[] calldata candidates, uint256[] calldata scores)
        external
        onlyAdmin
        returns (address[] memory)
    {
        require(candidates.length > 0, "SpaceIncentive: empty candidates");
        require(candidates.length == scores.length, "SpaceIncentive: invalid scores");

        uint256 count = _incentiveDistributionConfig.numOfTargets;
        require(count > 0, "SpaceIncentive: invalid recipient count");

        if (count > candidates.length) {
            count = candidates.length;
        }

        uint256 positiveCount = 0;
        for (uint256 i = 0; i < scores.length; i++) {
            if (scores[i] > 0) {
                positiveCount += 1;
            }
        }
        if (count > positiveCount) {
            count = positiveCount;
        }

        bool[] memory excluded = new bool[](candidates.length);
        address[] memory picked;
        if (_incentiveDistributionConfig.mode == IncentiveDistributionMode.Ranking) {
            picked = _selectByRanking(candidates, scores, count, excluded);
        } else if (_incentiveDistributionConfig.mode == IncentiveDistributionMode.Mixed) {
            picked = _selectByMixed(candidates, scores, count, excluded);
        } else {
            picked = _selectByWeightedRandom(candidates, scores, count, excluded, 0);
        }

        for (uint256 i = 0; i < _incentiveRecipients.length; i++) {
            address prev = _incentiveRecipients[i];
            _isIncentiveRecipient[prev] = false;
            _isIncentiveClaimed[prev] = false;
        }
        delete _incentiveRecipients;
        for (uint256 i = 0; i < picked.length; i++) {
            address pickedAddr = picked[i];
            require(pickedAddr != address(0), "SpaceIncentive: invalid recipient");
            _isIncentiveRecipient[pickedAddr] = true;
            _isIncentiveClaimed[pickedAddr] = false;
            _incentiveRecipients.push(pickedAddr);
        }
        _incentiveEpoch += 1;

        emit IncentiveDistributionSelected(_incentiveDistributionConfig.mode, picked.length);
        return picked;
    }

    function claimIncentive(address token) external {
        require(token != address(0), "SpaceIncentive: invalid token");

        address recipient = msg.sender;
        require(_isIncentiveRecipient[recipient], "SpaceIncentive: not selected");
        require(!_isIncentiveClaimed[recipient], "SpaceIncentive: incentive finished");

        IERC20 erc20 = IERC20(token);
        uint256 count = _incentiveRecipients.length;
        require(count > 0, "SpaceIncentive: invalid recipient count");
        uint256 value = _incentiveAmountByToken[token];
        if (_incentiveAmountEpoch[token] != _incentiveEpoch) {
            uint256 balance = erc20.balanceOf(address(this));
            value = balance / count;
            require(value > 0, "SpaceIncentive: invalid value");
            _incentiveAmountByToken[token] = value;
            _incentiveAmountEpoch[token] = _incentiveEpoch;
        }
        require(erc20.balanceOf(address(this)) >= value, "SpaceIncentive: insufficient balance");
        _isIncentiveClaimed[recipient] = true;
        require(erc20.transfer(recipient, value), "SpaceIncentive: transfer failed");

        emit IncentiveClaimed(token, recipient, value);
    }

    function _randomIndex(uint256 range, uint256 nonce) internal view returns (uint256) {
        if (range == 0) {
            return 0;
        }
        return uint256(
            keccak256(abi.encodePacked(block.prevrandao, block.timestamp, msg.sender, nonce))
        ) % range;
    }

    function _selectByRanking(
        address[] memory candidates,
        uint256[] memory scores,
        uint256 count,
        bool[] memory excluded
    ) internal pure returns (address[] memory) {
        address[] memory picked = new address[](count);
        uint256 pickedCount = 0;
        for (uint256 k = 0; k < count; k++) {
            uint256 bestScore = 0;
            uint256 bestIdx = type(uint256).max;
            for (uint256 i = 0; i < candidates.length; i++) {
                if (excluded[i]) {
                    continue;
                }
                uint256 score = scores[i];
                if (bestIdx == type(uint256).max || score > bestScore) {
                    bestScore = score;
                    bestIdx = i;
                }
            }
            if (bestIdx == type(uint256).max || bestScore == 0) {
                break;
            }
            excluded[bestIdx] = true;
            picked[pickedCount] = candidates[bestIdx];
            pickedCount += 1;
        }
        if (pickedCount == count) {
            return picked;
        }

        address[] memory trimmed = new address[](pickedCount);
        for (uint256 i = 0; i < pickedCount; i++) {
            trimmed[i] = picked[i];
        }
        return trimmed;
    }

    function _selectByMixed(
        address[] memory candidates,
        uint256[] memory scores,
        uint256 count,
        bool[] memory excluded
    ) internal view returns (address[] memory) {
        uint256 rankCount = (count * _incentiveDistributionConfig.rankingBps) / 10000;
        if (rankCount > count) {
            rankCount = count;
        }
        uint256 randomCount = count - rankCount;

        address[] memory picked = new address[](count);
        uint256 pickedCount = 0;
        if (rankCount > 0) {
            address[] memory ranked = _selectByRanking(
                candidates,
                scores,
                rankCount,
                excluded
            );
            for (uint256 i = 0; i < ranked.length; i++) {
                picked[pickedCount] = ranked[i];
                pickedCount += 1;
            }
        }

        if (randomCount > 0) {
            address[] memory randoms = _selectByWeightedRandom(
                candidates,
                scores,
                randomCount,
                excluded,
                rankCount
            );
            for (uint256 i = 0; i < randoms.length; i++) {
                picked[pickedCount] = randoms[i];
                pickedCount += 1;
            }
        }

        if (pickedCount == count) {
            return picked;
        }

        address[] memory trimmed = new address[](pickedCount);
        for (uint256 i = 0; i < pickedCount; i++) {
            trimmed[i] = picked[i];
        }
        return trimmed;
    }

    function _selectByWeightedRandom(
        address[] memory candidates,
        uint256[] memory scores,
        uint256 count,
        bool[] memory excluded,
        uint256 nonceOffset
    ) internal view returns (address[] memory) {
        uint256 n = scores.length;
        uint256[] memory bit = new uint256[](n + 1);
        uint256[] memory weights = new uint256[](n);
        for (uint256 i = 0; i < n; i++) {
            uint256 w = excluded[i] ? 0 : scores[i];
            weights[i] = w;
            if (w > 0) {
                _bitAdd(bit, i + 1, w);
            }
        }

        address[] memory picked = new address[](count);
        uint256 pickedCount = 0;
        for (uint256 k = 0; k < count; k++) {
            uint256 totalWeight = _bitSum(bit, n);
            if (totalWeight == 0) {
                break;
            }

            uint256 rand = _randomIndex(totalWeight, nonceOffset + k);
            uint256 chosen = _bitFind(bit, rand + 1);
            require(chosen < n, "SpaceIncentive: no candidates");

            excluded[chosen] = true;
            picked[pickedCount] = candidates[chosen];
            pickedCount += 1;

            uint256 w = weights[chosen];
            if (w > 0) {
                weights[chosen] = 0;
                _bitSub(bit, chosen + 1, w);
            }
        }
        if (pickedCount == count) {
            return picked;
        }

        address[] memory trimmed = new address[](pickedCount);
        for (uint256 i = 0; i < pickedCount; i++) {
            trimmed[i] = picked[i];
        }
        return trimmed;
    }

    function _bitAdd(uint256[] memory bit, uint256 idx, uint256 delta) internal pure {
        uint256 n = bit.length - 1;
        while (idx <= n) {
            bit[idx] += delta;
            idx += idx & (~idx + 1); // index += lowbit(index)
        }
    }

    function _bitSub(uint256[] memory bit, uint256 idx, uint256 delta) internal pure {
        uint256 n = bit.length - 1;
        while (idx <= n) {
            bit[idx] -= delta;
            idx += idx & (~idx + 1); // index += lowbit(index)
        }
    }

    function _bitSum(uint256[] memory bit, uint256 idx) internal pure returns (uint256) {
        uint256 sum = 0;
        while (idx > 0) {
            sum += bit[idx];
            idx -= idx & (~idx + 1); //index -= lowbit(index)
        }
        return sum;
    }

    function _bitFind(uint256[] memory bit, uint256 target) internal pure returns (uint256) {
        uint256 idx = 0;
        uint256 bitMask = 1;
        uint256 n = bit.length - 1;
        while (bitMask <= n) {
            bitMask <<= 1; //2의 거듭제곱
        }
        uint256 sum = 0;
        for (uint256 step = bitMask; step > 0; step >>= 1) { // bitmask, bitmask/2, bitmask/4, ...
            uint256 next = idx + step;
            if (next <= n && sum + bit[next] < target) {
                sum += bit[next];
                idx = next;
            }
        }
        return idx; // 0-based index
    }
}
