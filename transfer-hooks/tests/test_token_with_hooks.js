const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("TokenWithHooks", function () {
    let TokenWithHooks, token, owner, addr1, addr2;

    beforeEach(async function () {
        TokenWithHooks = await ethers.getContractFactory("TokenWithHooks");
        [owner, addr1, addr2, royaltyReceiver] = await ethers.getSigners();
        token = await TokenWithHooks.deploy("TestToken", "TTK", 18, 1000000, royaltyReceiver.address, 5);
        await token.deployed();
    });

    it("Should transfer tokens and apply royalties", async function () {
        await token.transfer(addr1.address, 1000);
        expect(await token.balanceOf(addr1.address)).to.equal(950);
        expect(await token.balanceOf(royaltyReceiver.address)).to.equal(50);
    });
});