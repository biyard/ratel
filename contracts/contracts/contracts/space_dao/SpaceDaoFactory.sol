// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/proxy/Clones.sol";
import "./SpaceDao.sol";
import "./SpaceDaoRewardExtension.sol";

contract SpaceFactory {
    address public immutable daoImplementation;
    address public immutable extImplementation;
    
    address[] public deployedDAOs;

    event SpaceCreated(address indexed dao, address indexed rewardExtension);

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
        SpaceDAO(payable(daoClone)).initialize(_admins, extClone);

        // 4. Record
        deployedDAOs.push(daoClone);
        
        emit SpaceCreated(daoClone, extClone);

        return daoClone;
    }

    function getDeployedDAOs() external view returns (address[] memory) {
        return deployedDAOs;
    }
}