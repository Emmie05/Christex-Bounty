pragma solidity ^0.8.0;

import "@solana/spl-token/contracts/Token.sol";

contract TokenWithHooks is Token {
    address public royaltyReceiver;
    uint256 public royaltyPercentage;

    constructor(
        string memory name,
        string memory symbol,
        uint8 decimals,
        uint256 initialSupply,
        address _royaltyReceiver,
        uint256 _royaltyPercentage
    ) Token(name, symbol, decimals, initialSupply) {
        royaltyReceiver = _royaltyReceiver;
        royaltyPercentage = _royaltyPercentage;
    }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override {
        super._beforeTokenTransfer(from, to, amount);

        if (from != address(0) && to != address(0)) {
            uint256 royaltyAmount = (amount * royaltyPercentage) / 100;
            _transfer(from, royaltyReceiver, royaltyAmount);
        }
    }
}