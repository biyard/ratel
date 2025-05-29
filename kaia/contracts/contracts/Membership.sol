// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

// Uncomment this line to use console.log
// import "hardhat/console.sol";
import {ERC1155} from "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract Membership is ERC1155, Ownable {
    string private _name;
    string private _symbol;

    mapping(address => bool) private _isHolder;
    mapping(uint256 => string) private _tokenURIs;

    constructor(address owner, string memory name_, string memory symbol_, string memory uri_) ERC1155(uri_) Ownable(owner) {
        _name = name_;
        _symbol = symbol_;
    }

    function name() public view virtual returns (string memory) {
        return _name;
    }

    function symbol() public view virtual returns (string memory) {
        return _symbol;
    }

    function setName(string memory name_) external onlyOwner {
        _name = name_;
    }

    function setSymbol(string memory symbol_) external onlyOwner {
        _symbol = symbol_;
    }

    function setURI(string memory newuri) external onlyOwner {
        _setURI(newuri);
    }

    function uri(uint256 id) public view override returns (string memory) {
        string memory tokenUri = _tokenURIs[id];
        if (bytes(tokenUri).length > 0) {
            return tokenUri;
        }
        return super.uri(id); 
    }

    function setTokenURI(uint256 id, string memory newuri) external onlyOwner {
        _tokenURIs[id] = newuri;
    }

    function mint(uint256 id, uint256 value) external mintOnce {
        _mint(msg.sender, id, value, new bytes(0));
    }

    function mintBatch(uint256[] memory ids, uint256[] memory values) external mintOnce {
        _mintBatch(msg.sender, ids, values, new bytes(0));
    }

    function isHolder(address addr) external view returns (bool) {
        return _isHolder[addr];
    }


    modifier mintOnce() {
        require(!_isHolder[msg.sender], "Already Minted");
        _isHolder[msg.sender] = true;
        _;
    }
}
