// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.0;

contract EIP2771 {
    address internal _forwarder;

    constructor(address forwarder) {
        _forwarder = forwarder;
    }

    function setForwarder(address forwarder) public {
        require(msg.sender == _forwarder, "it must be called by the current forwarder");
        _forwarder = forwarder;
    }

    function _msgSender() internal view returns (address payable sender) {
        sender = payable(msg.sender);

        if (msg.sender == _forwarder) {
            assembly {
                sender := shr(96, calldataload(sub(calldatasize(), 20)))
            }
        }

        return payable(sender);
    }

    function _msgValue() internal view returns (uint256) {
        uint256 value = msg.value;

        if (msg.sender == _forwarder) {
            assembly {
                value := calldataload(sub(calldatasize(), 52))
            }
        }

        return value;
    }

    function _msgData() internal view returns (bytes calldata) {
        if (msg.sender == _forwarder) {
            return msg.data[:msg.data.length - 52];
        }

        return msg.data;
    }
}