// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

library BytesLib {
    function toString(bytes memory self) internal pure returns (string memory) {
        bytes memory alphabet = "0123456789abcdef";

        bytes memory str = new bytes(2 + self.length * 2);
        str[0] = "0";
        str[1] = "x";
        for (uint i = 0; i < self.length; i++) {
            str[2 + i * 2] = alphabet[uint(uint8(self[i] >> 4))];
            str[3 + i * 2] = alphabet[uint(uint8(self[i] & 0x0f))];
        }
        return string(str);
    }
}