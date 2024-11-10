async function main() {
    const [owner, addr1] = await ethers.getSigners();
    const TokenWithHooks = await ethers.getContractFactory("TokenWithHooks");
    const token = await TokenWithHooks.attach("deployed_token_address");

    await token.transfer(addr1.address, 1000);
    console.log("Transfer complete");
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });