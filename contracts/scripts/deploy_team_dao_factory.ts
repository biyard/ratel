import { ethers, network } from "hardhat";

async function main() {
  console.log(`Deploying SpaceDAO factory on network: ${network.name}`);
  console.log("Deployer:", (await ethers.getSigners())[0].address);

  const rewardExtensionAddr = process.env.REWARD_EXTENSION_ADDR;
  const teamDaoAddr = process.env.TEAM_DAO_ADDR;

  if (!teamDaoAddr || !rewardExtensionAddr) {
    throw new Error("Logic addresses are missing in the deployment file.");
  }

  console.log("TeamDao Address:", teamDaoAddr);
  console.log("RewardExtension Address:", rewardExtensionAddr);

  console.log("Deploying SpaceFactory...");
  const TeamDaoFactory = await ethers.getContractFactory("TeamDaoFactory");

  const factory = await TeamDaoFactory.deploy(teamDaoAddr, rewardExtensionAddr);
  await factory.waitForDeployment();
  const factoryAddr = await factory.getAddress();

  console.log("TeamDaoFactory deployed at:", factoryAddr);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
