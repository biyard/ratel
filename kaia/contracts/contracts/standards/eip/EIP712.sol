// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.0;

// EIP-712: Ethereum typed structured data hashing and signing
abstract contract EIP712 {
    struct EIP712Domain {
        string name;
        string version;
        address verifyingContract;
        bytes32 salt;
    }

    string public constant ERC712_VERSION = "1";

    bytes32 internal constant EIP712_DOMAIN_TYPEHASH =
        keccak256(bytes("EIP712Domain(string name,string version,address verifyingContract,bytes32 salt)"));
    bytes32 internal domainSeperator;

    function _initializeEIP712(string memory name) internal {
        _setDomainSeperator(name);
    }

    function _setDomainSeperator(string memory name) internal {
        domainSeperator = keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPEHASH,
                keccak256(bytes(name)),
                keccak256(bytes(ERC712_VERSION)),
                address(this),
                bytes32(getChainId())
            )
        );
    }

    function getDomainSeperator() public view returns (bytes32) {
        return domainSeperator;
    }

    function getChainId() public view returns (uint256 id) {
        assembly {
            id := chainid()
        }
        return id;
    }

    function toTypedMessageHash(bytes32 messageHash) public view returns (bytes32) {
        return keccak256(abi.encodePacked("\x19\x01", getDomainSeperator(), messageHash));
    }
}