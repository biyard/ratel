// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/proxy/Clones.sol";
import "./TeamDao.sol";
import "./TeamDaoRewardExtension.sol";

contract TeamDaoFactory {
    address public immutable daoImplementation;
    address public immutable extImplementation;
    
    address[] public deployedDAOs;

    event TeamCreated(address indexed dao, address indexed rewardExtension);

    constructor(address _daoImpl, address _extImpl) {
        daoImplementation = _daoImpl;
        extImplementation = _extImpl;
    }

    /**
     * @dev Create DAO + Extension Bundle.
     */
    function createSpace(
        address[] calldata _admins
    ) external returns (address) {
        
        // 1. Clone Shells
        address daoClone = Clones.clone(daoImplementation);
        address extClone = Clones.clone(extImplementation);

        // 2. Init Extension
        RewardExtension(extClone).initialize(daoClone);

        // 3. Init DAO (Link Extension)
        // No mainToken passed here
        TeamDAO(payable(daoClone)).initialize(_admins, extClone);

        // 4. Record
        deployedDAOs.push(daoClone);
        
        emit TeamCreated(daoClone, extClone);

        return daoClone;
    }

    function getDeployedDAOs() external view returns (address[] memory) {
        return deployedDAOs;
    }
}