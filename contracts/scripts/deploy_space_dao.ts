import { ethers, network } from "hardhat";

function parseAdmins(raw: string | undefined): string[] {
  if (!raw) return [];
  return raw
    .split(",")
    .map((v) => v.trim())
    .filter((v) => v.length > 0);
}

async function main() {
  console.log(`Deploying SpaceDAO on network: ${network.name}`);

  const [deployer] = await ethers.getSigners();
  console.log("Deployer:", deployer.address);

  const admins = parseAdmins(process.env.SPACE_DAO_ADMINS);
  const usdtAddress = process.env.USDT_ADDRESS;

  if (!usdtAddress) {
    throw new Error("USDT_ADDRESS is required.");
  }

  let withdrawalAmount: bigint;
  const rawWithdrawal = process.env.WITHDRAWAL_AMOUNT_RAW;
  if (rawWithdrawal && rawWithdrawal.trim().length > 0) {
    withdrawalAmount = BigInt(rawWithdrawal);
  } else {
    const amount = process.env.WITHDRAWAL_AMOUNT;
    if (!amount) {
      throw new Error(
        "WITHDRAWAL_AMOUNT or WITHDRAWAL_AMOUNT_RAW is required."
      );
    }
    const decimals = Number(process.env.USDT_DECIMALS ?? "6");
    withdrawalAmount = ethers.parseUnits(amount, decimals);
  }

  if (admins.length < 3) {
    throw new Error("SPACE_DAO_ADMINS must include at least 3 addresses.");
  }

  console.log("Admins:", admins);
  console.log("USDT:", usdtAddress);
  console.log("WithdrawalAmount:", withdrawalAmount.toString());

  const SpaceDAO = await ethers.getContractFactory("SpaceDAO");
  const spaceDao = await SpaceDAO.deploy(admins, usdtAddress, withdrawalAmount);
  await spaceDao.waitForDeployment();

  const addr = await spaceDao.getAddress();
  console.log("SpaceDAO deployed at:", addr);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
