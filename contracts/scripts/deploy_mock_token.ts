import { ethers } from "hardhat";

const DEFAULT_RECIPIENT = "0x38C9AC4bb2225F32C56775091D8ADFd0a0AD35E4";
const DEFAULT_AMOUNT = "1000";

async function main() {
  const [deployer] = await ethers.getSigners();
  const recipient = process.env.TOKEN_RECIPIENT ?? DEFAULT_RECIPIENT;
  const amount = process.env.TOKEN_AMOUNT ?? DEFAULT_AMOUNT;

  console.log("Deployer:", deployer.address);
  console.log("Recipient:", recipient);
  console.log("Amount:", amount);

  const Token = await ethers.getContractFactory("MockToken");
  const token = await Token.deploy();
  await token.waitForDeployment();

  const tokenAddress = await token.getAddress();
  const code = await ethers.provider.getCode(tokenAddress);
  if (code === "0x") {
    throw new Error(`Deployed code not found at ${tokenAddress}`);
  }
  const decimals = 18n;
  const transferAmount = ethers.parseUnits(amount, decimals);

  console.log("Token deployed:", tokenAddress);
  console.log("Decimals:", decimals.toString());

  const tokenWithSigner = token.connect(deployer);
  const transferGas = await tokenWithSigner
    .getFunction("transfer")
    .estimateGas(recipient, transferAmount);
  const tx = await tokenWithSigner.getFunction("transfer").send(
    recipient,
    transferAmount,
    {
      gasLimit: transferGas + transferGas / 2n,
    },
  );
  await tx.wait();

  console.log("Transfer complete:", tx.hash);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
