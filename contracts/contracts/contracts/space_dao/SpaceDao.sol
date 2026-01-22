// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/proxy/utils/Initializable.sol";

contract SpaceDAO is Initializable, ReentrancyGuard {
    
    // --- State Variables ---
    bool public isDaoActive; // 여전히 비상 정지용으로 남겨둠 (true = Active)
    
    address[] public admins;
    mapping(address => bool) public isAdmin;
    
    // Whitelist for extensions
    mapping(address => bool) public isExtension;

    // [Direct Link] Main Extension Address
    address public rewardExtension; 

    // --- Events ---
    event Initialized(address[] admins, address extension);
    event ExtensionCall(address indexed extension, address indexed target, uint256 value, bytes data);
    event Received(address indexed sender, uint256 amount);

    // --- Modifiers ---
    modifier onlyExtension() {
        require(isExtension[msg.sender], "SpaceDAO: Caller is not an extension");
        _;
    }

    modifier onlyActive() {
        require(isDaoActive, "SpaceDAO: DAO is inactive");
        _;
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() { _disableInitializers(); }

    /**
     * @dev Initialize without mainToken.
     */
    function initialize(
        address[] calldata _admins,
        address _initialExtension
    ) external initializer {
        require(_admins.length >= 3, "SpaceDAO: Must have at least 3 admins");

        for (uint i = 0; i < _admins.length; i++) {
            admins.push(_admins[i]);
            isAdmin[_admins[i]] = true;
        }

        isDaoActive = true;

        // Register Extension
        if (_initialExtension != address(0)) {
            isExtension[_initialExtension] = true;
            rewardExtension = _initialExtension;
        }

        emit Initialized(_admins, _initialExtension);
    }

    receive() external payable {
        emit Received(msg.sender, msg.value);
    }

    /**
     * @dev Executes actions requested by the Extension.
     */
    function executeCall(
        address _target,
        uint256 _value,
        bytes calldata _data
    ) external onlyExtension onlyActive nonReentrant returns (bytes memory) {
        // Just execute the call
        (bool success, bytes memory result) = _target.call{value: _value}(_data);
        require(success, "SpaceDAO: Low-level call failed");

        emit ExtensionCall(msg.sender, _target, _value, _data);

        return result;
    }

    // Helper to check admin status (called by Extension)
    function checkAdmin(address _user) external view returns (bool) {
        return isAdmin[_user];
    }
}