const hre = require("hardhat");

const contractAddress = "0x2a95f25ea2C08a98f46E8b68A6fCcDe26F931e49";

async function sleep(ms) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

async function main() {
	const nftContract = await hre.ethers.deployContract("CryptoDevs", [contractAddress]);

	await nftContract.waitForDeployment();

	console.log("NFT Contract Address:", nftContract.target);

	// Sleep for 30 seconds while Etherscan indexes the new contract deployment
	await sleep(30 * 1000);

	await hre.run("verify:verify", {
		address: nftContract.target,
		constructorArguments: [contractAddress],
	});
}

main()
	.then(() => process.exit(0))
	.catch((error) => {
		console.error(error);
		process.exit(1);
	});