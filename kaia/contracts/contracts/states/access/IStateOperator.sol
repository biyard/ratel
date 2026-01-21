// SPDX-License-Identifier: Biyard
pragma solidity ^0.8.0;

interface IStateOperator {
    function allowPublicRead(bool publicRead) external;

    function allowPublicWrite(bool publicWrite) external;

    function allowReady(bool ready) external;

    function setStateReady(bool ready) external;

    function getStateReady() external view returns (bool);

    function publicRead() external view returns (bool);

    function publicWrite() external view returns (bool);

    function addDaoStateOperator(address o) external;

    function addOperator(address o) external;

    function delOperator(address o) external;

    function listOperators() external view returns (address[] memory operators);

    function lengthOfOperators() external view returns (uint256);

    function existsOperator(address o) external view returns (bool);

    function migrate(address to) external;
}