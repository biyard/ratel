// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.0;

import "./EIP712.sol";

// OpenSea-compatible meta transaction
abstract contract NativeMetaTransaction is EIP712 {
    bytes32 private constant META_TRANSACTION_TYPEHASH =
        keccak256(bytes("MetaTransaction(uint256 nonce,address from,bytes functionSignature)"));
    event MetaTransactionExecuted(address userAddress, address payable relayerAddress, bytes functionSignature);
    mapping(address => uint256) nonces;

    struct MetaTransaction {
        uint256 nonce;
        address from;
        bytes functionSignature;
    }

    function addressOf(string memory name) internal view virtual returns (address);

    function executeMetaTransaction(
        string memory callee,
        address userAddress,
        bytes memory functionSignature,
        bytes32 sigR,
        bytes32 sigS,
        uint8 sigV
    ) public payable returns (bytes memory) {
        uint256 chainId;
        assembly {
            chainId := chainid()
        }
        if (chainId == 1001 || chainId == 8217 || chainId == 82051) {
            require(userAddress == msg.sender, "sender must be same with userAddress");
        } else {
            MetaTransaction memory metaTx = MetaTransaction({
                nonce: nonces[userAddress],
                from: userAddress,
                functionSignature: functionSignature
                });
            require(
                    verify(userAddress, metaTx, sigR, sigS, sigV),
                    "signer and signature do not match"
            );
            nonces[userAddress] = nonces[userAddress] + 1;
            emit MetaTransactionExecuted(userAddress, payable(msg.sender), functionSignature);
        }

        address callContract = addressOf(callee);
        (bool success, bytes memory returnData) = callContract.call(abi.encodePacked(functionSignature, msg.value, userAddress));
        require(success, string(abi.encodePacked("function call has been failed; ", abi.encodePacked(returnData))));

        return returnData;
    }

    function getNonce(address addr) external view returns (uint256) {
        return nonces[addr];
    }

    function getTypedMessageHash(address userAddress, bytes memory functionSignature) external view returns (bytes32) {
        MetaTransaction memory metaTx = MetaTransaction({
            nonce: nonces[userAddress],
            from: userAddress,
            functionSignature: functionSignature
        });

        return toTypedMessageHash(hashMetaTransaction(metaTx));
    }

    function hashMetaTransaction(MetaTransaction memory metaTx) internal pure returns (bytes32) {
        return
            keccak256(abi.encode(META_TRANSACTION_TYPEHASH, metaTx.nonce, metaTx.from, keccak256(metaTx.functionSignature)));
    }

    function verify(
        address signer,
        MetaTransaction memory metaTx,
        bytes32 sigR,
        bytes32 sigS,
        uint8 sigV
    ) internal view returns (bool) {
        require(signer != address(0), "NativeMetaTransaction: INVALID_SIGNER");
        bytes32 message = toTypedMessageHash(hashMetaTransaction(metaTx));
        address recovered = ecrecover(message, sigV, sigR, sigS);
        require(
            signer == recovered,
            string(abi.encodePacked("mismatched address;", addressToString(recovered), ", ", toString(message)))
        );
        return signer == recovered;
    }

    function addressToString(address x) internal pure returns (string memory) {
        bytes memory s = new bytes(40);
        for (uint i = 0; i < 20; i++) {
            bytes1 b = bytes1(uint8(uint(uint160(x)) / (2 ** (8 * (19 - i)))));
            bytes1 hi = bytes1(uint8(b) / 16);
            bytes1 lo = bytes1(uint8(b) - 16 * uint8(hi));
            s[2 * i] = char(hi);
            s[2 * i + 1] = char(lo);
        }
        return string(abi.encodePacked("0x", s));
    }

    function toString(bytes32 self) internal pure returns (string memory) {
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

    function char(bytes1 b) internal pure returns (bytes1 c) {
        if (uint8(b) < 10) return bytes1(uint8(b) + 0x30);
        else return bytes1(uint8(b) + 0x57);
    }
}