import { ethers } from "ethers";

const KAIA_TESTNET_CHAIN_ID = "0x3e9"; // 1001
const KAIA_MAINNET_CHAIN_ID = "0x2019"; // 8217

const KAIA_NETWORKS = {
  mainnet: {
    chainId: KAIA_MAINNET_CHAIN_ID,
    chainName: "Kaia Mainnet",
    nativeCurrency: {
      name: "KAIA",
      symbol: "KAIA",
      decimals: 18,
    },
    rpcUrls: ["https://public-en.node.kaia.io"],
    blockExplorerUrls: ["https://kaiascan.io"],
  },
  testnet: {
    chainId: KAIA_TESTNET_CHAIN_ID,
    chainName: "Kaia Kairos Testnet",
    nativeCurrency: {
      name: "KAIA",
      symbol: "KAIA",
      decimals: 18,
    },
    rpcUrls: ["https://public-en-kairos.node.kaia.io"],
    blockExplorerUrls: ["https://kairos.kaiascan.io"],
  },
};

let artifactCache = null;

function parseEnvNetwork(env) {
  const value = String(env || "").toLowerCase();
  return value === "prod" ? "mainnet" : "testnet";
}

async function getEthereum() {
  const ethereum = window?.ethereum;
  if (!ethereum || typeof ethereum.request !== "function") {
    throw new Error("MetaMask is not installed");
  }
  return ethereum;
}

function chainsEqual(left, right) {
  try {
    return parseInt(left, 16) === parseInt(right, 16);
  } catch (_err) {
    return left === right;
  }
}

async function ensureKaiaNetwork(ethereum, networkName) {
  const network = KAIA_NETWORKS[networkName] || KAIA_NETWORKS.testnet;
  const targetChainId = network.chainId;

  let currentChainId = await ethereum.request({ method: "eth_chainId" });
  if (chainsEqual(currentChainId, targetChainId)) {
    return;
  }

  try {
    await ethereum.request({
      method: "wallet_switchEthereumChain",
      params: [{ chainId: targetChainId }],
    });
  } catch (switchError) {
    if (switchError?.code === 4902) {
      try {
        await ethereum.request({
          method: "wallet_addEthereumChain",
          params: [network],
        });
      } catch (addError) {
        const message =
          addError?.data?.cause?.message || addError?.message || "";
        if (
          message.includes("same RPC endpoint as existing network") ||
          addError?.code === -32603
        ) {
          // Ignore duplicate-network errors.
        } else if (addError?.code === 4001) {
          throw new Error("User rejected network add");
        } else {
          throw addError;
        }
      }
    } else if (switchError?.code === 4001) {
      throw new Error("User rejected network switch");
    } else {
      throw switchError;
    }
  }

  currentChainId = await ethereum.request({ method: "eth_chainId" });
  if (!chainsEqual(currentChainId, targetChainId)) {
    throw new Error(
      `Failed to switch network (current=${currentChainId}, target=${targetChainId})`,
    );
  }
}

async function loadSpaceIncentiveArtifact() {
  if (artifactCache) {
    return artifactCache;
  }

  const response = await fetch("/assets/space-incentive-artifact.json", {
    cache: "no-store",
  });

  if (!response.ok) {
    throw new Error(
      `Failed to load SpaceIncentive artifact (status=${response.status})`,
    );
  }

  const artifact = await response.json();
  if (!artifact?.abi || !artifact?.bytecode) {
    throw new Error("Invalid SpaceIncentive artifact");
  }

  artifactCache = artifact;
  return artifactCache;
}

async function deploySpaceIncentive(params = {}) {
  const env = String(params.env || "local");
  const networkName = parseEnvNetwork(env);
  const ethereum = await getEthereum();

  await ensureKaiaNetwork(ethereum, networkName);

  let accounts;
  try {
    accounts = await ethereum.request({ method: "eth_requestAccounts" });
  } catch (err) {
    if (err?.code === 4001) {
      throw new Error("User rejected wallet connection");
    }
    throw err;
  }

  const account =
    Array.isArray(accounts) && accounts.length > 0 ? accounts[0] : null;
  if (!account) {
    throw new Error("Wallet connection cancelled or no accounts");
  }

  const admins = [String(account).toLowerCase()];

  const incentiveRecipientCount = Number(params.incentiveRecipientCount ?? 10);
  if (
    !Number.isFinite(incentiveRecipientCount) ||
    incentiveRecipientCount <= 0
  ) {
    throw new Error("Incentive recipient count must be greater than 0");
  }

  const mode = Number(params.mode ?? 0);
  const rankingBps = Number(params.rankingBps ?? 0);

  const artifact = await loadSpaceIncentiveArtifact();

  const provider = new ethers.BrowserProvider(ethereum);
  const signer = await provider.getSigner();

  const factory = new ethers.ContractFactory(
    artifact.abi,
    artifact.bytecode,
    signer,
  );

  const contract = await factory.deploy(admins, {
    mode,
    numOfTargets: incentiveRecipientCount,
    rankingBps,
  });

  await contract.waitForDeployment();

  const incentiveAddress = await contract.getAddress();
  const deploymentTx = contract.deploymentTransaction();
  const receipt = deploymentTx ? await deploymentTx.wait() : null;

  return {
    incentiveAddress,
    deployBlock: Number(receipt?.blockNumber || 0),
    transactionHash: deploymentTx?.hash || "",
    adminAddress: String(account),
  };
}

async function copyText(value) {
  if (!navigator?.clipboard?.writeText) {
    throw new Error("Clipboard API is not available");
  }

  await navigator.clipboard.writeText(String(value || ""));
  return true;
}

const incentive_pool = {
  deploySpaceIncentive,
  copyText,
};

export default incentive_pool;
