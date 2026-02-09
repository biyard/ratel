// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract SpaceIncentiveBitHarness {
    function findIndex(
        uint256[] calldata scores,
        bool[] calldata excluded,
        uint256 target
    ) external pure returns (uint256) {
        require(scores.length == excluded.length, "length mismatch");
        uint256 n = scores.length;
        uint256[] memory bit = new uint256[](n + 1);
        for (uint256 i = 0; i < n; i++) {
            uint256 w = excluded[i] ? 0 : scores[i];
            if (w > 0) {
                _bitAdd(bit, i + 1, w);
            }
        }
        uint256 total = _bitSum(bit, n);
        require(total > 0, "empty scores");
        require(target < total, "target out of range");
        return _bitFind(bit, target + 1);
    }

    function _bitAdd(uint256[] memory bit, uint256 idx, uint256 delta) internal pure {
        uint256 n = bit.length - 1;
        while (idx <= n) {
            bit[idx] += delta;
            idx += idx & (~idx + 1);
        }
    }

    function _bitSum(uint256[] memory bit, uint256 idx) internal pure returns (uint256) {
        uint256 sum = 0;
        while (idx > 0) {
            sum += bit[idx];
            idx -= idx & (~idx + 1);
        }
        return sum;
    }

    function _bitFind(uint256[] memory bit, uint256 target) internal pure returns (uint256) {
        uint256 idx = 0;
        uint256 bitMask = 1;
        uint256 n = bit.length - 1;
        while (bitMask <= n) {
            bitMask <<= 1;
        }
        uint256 sum = 0;
        for (uint256 step = bitMask; step > 0; step >>= 1) {
            uint256 next = idx + step;
            if (next <= n && sum + bit[next] < target) {
                sum += bit[next];
                idx = next;
            }
        }
        return idx;
    }
}
