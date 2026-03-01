import { ethers } from "ethers";
import TeamDaoArtifact from "./TeamDao.json";

const KAIA_TESTNET_CHAIN_ID = "0x3e9"; // 1001
const KAIA_MAINNET_CHAIN_ID = "0x2019"; // 8217

class KaiaWalletError extends Error {
  constructor(code, message) {
    super(message);
    this.code = code;
  }
}

function getTargetChainId(network) {
  return network === "mainnet" ? KAIA_MAINNET_CHAIN_ID : KAIA_TESTNET_CHAIN_ID;
}

async function getEthereum() {
  const { ethereum } = window;
  if (!ethereum || !ethereum.request) {
    throw new KaiaWalletError(
      "METAMASK_NOT_INSTALLED",
      "MetaMask is not installed"
    );
  }
  return ethereum;
}

function chainsEqual(a, b) {
  try {
    return parseInt(a, 16) === parseInt(b, 16);
  } catch {
    return a === b;
  }
}

async function ensureKaiaNetwork(network, rpcUrl, explorerUrl) {
  const ethereum = await getEthereum();
  const targetChainId = getTargetChainId(network);

  let chainIdHex = await ethereum.request({ method: "eth_chainId" });
  if (chainIdHex === targetChainId) {
    return { ethereum, chainId: chainIdHex };
  }

  try {
    await ethereum.request({
      method: "wallet_switchEthereumChain",
      params: [{ chainId: targetChainId }],
    });
  } catch (switchError) {
    if (switchError?.code === 4902) {
      const baseParams =
        network === "mainnet"
          ? {
              chainId: KAIA_MAINNET_CHAIN_ID,
              chainName: "Kaia Mainnet",
              nativeCurrency: { name: "KAIA", symbol: "KAIA", decimals: 18 },
            }
          : {
              chainId: KAIA_TESTNET_CHAIN_ID,
              chainName: "Kaia Kairos Testnet",
              nativeCurrency: { name: "KAIA", symbol: "KAIA", decimals: 18 },
            };

      const params = {
        ...baseParams,
        rpcUrls: [rpcUrl].filter(Boolean),
        blockExplorerUrls: [explorerUrl].filter(Boolean),
      };

      try {
        await ethereum.request({
          method: "wallet_addEthereumChain",
          params: [params],
        });
      } catch (addError) {
        const msg = addError?.data?.cause?.message || addError?.message || "";
        if (
          msg.includes("same RPC endpoint as existing network") ||
          addError?.code === -32603
        ) {
          // ignore
        } else if (addError?.code === 4001) {
          throw new KaiaWalletError(
            "USER_REJECTED",
            "User rejected network add"
          );
        } else {
          throw addError;
        }
      }
    } else if (switchError?.code === 4001) {
      throw new KaiaWalletError(
        "USER_REJECTED",
        "User rejected network switch"
      );
    } else {
      throw switchError;
    }
  }

  chainIdHex = await ethereum.request({ method: "eth_chainId" });
  if (!chainsEqual(chainIdHex, targetChainId)) {
    throw new KaiaWalletError(
      "CHAIN_SWITCH_FAILED",
      `Failed to switch to Kaia network (current=${chainIdHex}, target=${targetChainId})`
    );
  }

  return { ethereum, chainId: chainIdHex };
}

async function getKaiaSigner(network, rpcUrl, explorerUrl) {
  const { ethereum } = await ensureKaiaNetwork(network, rpcUrl, explorerUrl);

  let accounts = [];
  try {
    accounts = await ethereum.request({ method: "eth_requestAccounts" });
  } catch (e) {
    if (e?.code === 4001) {
      throw new KaiaWalletError(
        "USER_REJECTED",
        "User rejected wallet connection"
      );
    }
    throw e;
  }

  if (!accounts || accounts.length === 0) {
    throw new KaiaWalletError(
      "NO_ACCOUNTS",
      "Wallet connection cancelled or no accounts"
    );
  }

  const provider = new ethers.BrowserProvider(ethereum);
  const signer = await provider.getSigner();
  return { provider, signer, account: accounts[0] };
}

async function createDAO(admins, network, rpcUrl, explorerUrl) {
  if (!Array.isArray(admins)) {
    throw new Error("Admins must be an array");
  }
  if (admins.length < 3) {
    throw new Error("At least 3 admins are required to create a DAO");
  }

  const { signer } = await getKaiaSigner(network, rpcUrl, explorerUrl);

  const TeamDAO = new ethers.ContractFactory(
    TeamDaoArtifact.abi,
    TeamDaoArtifact.bytecode,
    signer
  );

  const dao = await TeamDAO.deploy(admins);
  const receipt = await dao.deploymentTransaction()?.wait();

  if (!receipt) {
    throw new Error("Deployment transaction failed");
  }

  const daoAddress = await dao.getAddress();

  return {
    daoAddress,
    transactionHash: receipt.hash,
  };
}

async function copyText(text) {
  if (navigator?.clipboard?.writeText) {
    await navigator.clipboard.writeText(text);
    return true;
  }

  const textarea = document.createElement("textarea");
  textarea.value = text;
  textarea.style.position = "fixed";
  textarea.style.opacity = "0";
  document.body.appendChild(textarea);
  textarea.focus();
  textarea.select();

  let success = false;
  try {
    success = document.execCommand("copy");
  } finally {
    document.body.removeChild(textarea);
  }

  if (!success) {
    throw new Error("Failed to copy text");
  }
  return true;
}

if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.ratel_team_dao = {
    initialize: (_conf) => {
      console.debug("Initializing ratel_team_dao with config");
    },
    createDAO,
    copyText,
  };
}
