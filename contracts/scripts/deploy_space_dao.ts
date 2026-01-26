import { ethers, network } from "hardhat";

async function main() {
  console.log(`Deploying SpaceDAO Contracts on network: ${network.name}`);

  const [deployer] = await ethers.getSigners();
  console.log("Deployer:", deployer.address);

  console.log("Deploying SpaceDAO...");
  const SpaceDAO = await ethers.getContractFactory("SpaceDAO");
  const spaceDaoLogic = await SpaceDAO.deploy();
  await spaceDaoLogic.waitForDeployment();
  const spaceDaoAddr = await spaceDaoLogic.getAddress();
  console.log("SpaceDAO deployed at:", spaceDaoAddr);

  console.log("Deploying RewardExtension...");
  const RewardExtension = await ethers.getContractFactory("RewardExtension");
  const rewardExtLogic = await RewardExtension.deploy();
  await rewardExtLogic.waitForDeployment();
  const rewardExtAddr = await rewardExtLogic.getAddress();
  console.log("RewardExtension deployed at:", rewardExtAddr);


  console.log("SpaceDAO Address:", spaceDaoAddr);
  console.log("RewardExtension Address:", rewardExtAddr);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});