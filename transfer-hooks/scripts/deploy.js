async function main() {
    const [deployer] = await ethers.getSigners();
    console.log("Deploying contracts with the account:", deployer.address);

    const TokenWithHooks = await ethers.getContractFactory("TokenWithHooks");
    const token = await TokenWithHooks.deploy("TestToken", "TTK", 18, 1000000, deployer.address, 5);

    console.log("TokenWithHooks deployed to:", token.address);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });