import { ethers } from "ethers";

const BRAND_TOKEN_ABI = [
  "function claim(uint256 month, uint256 amount, uint256 maxClaimable, uint256 nonce, uint256 deadline, bytes signature) external",
  "function balanceOf(address) view returns (uint256)",
  "function decimals() view returns (uint8)",
  "function symbol() view returns (string)",
];

const KAIA_NETWORKS = {
  1001: {
    chainId: "0x3e9",
    chainName: "Kaia Kairos Testnet",
    nativeCurrency: { name: "KAIA", symbol: "KAIA", decimals: 18 },
    rpcUrls: ["https://public-en-kairos.node.kaia.io"],
    blockExplorerUrls: ["https://kairos.kaiascan.io"],
  },
  8217: {
    chainId: "0x2019",
    chainName: "Kaia Mainnet",
    nativeCurrency: { name: "KAIA", symbol: "KAIA", decimals: 18 },
    rpcUrls: ["https://public-en.node.kaia.io"],
    blockExplorerUrls: ["https://kaiascan.io"],
  },
};

async function getEthereum() {
  const ethereum = window?.ethereum;
  if (!ethereum || typeof ethereum.request !== "function") {
    throw new Error("No wallet extension found. Please install Kaia Wallet or MetaMask.");
  }
  return ethereum;
}

async function ensureNetwork(ethereum, chainId) {
  const network = KAIA_NETWORKS[chainId];
  if (!network) {
    throw new Error("Unsupported chain: " + chainId);
  }

  const targetChainId = network.chainId;
  let currentChainId = await ethereum.request({ method: "eth_chainId" });

  if (parseInt(currentChainId, 16) === parseInt(targetChainId, 16)) {
    return;
  }

  try {
    await ethereum.request({
      method: "wallet_switchEthereumChain",
      params: [{ chainId: targetChainId }],
    });
  } catch (switchError) {
    if (switchError?.code === 4902) {
      await ethereum.request({
        method: "wallet_addEthereumChain",
        params: [network],
      });
    } else if (switchError?.code === 4001) {
      throw new Error("User rejected network switch");
    } else {
      throw switchError;
    }
  }
}

async function connectAndGetSigner(chainId) {
  const ethereum = await getEthereum();
  await ensureNetwork(ethereum, chainId);

  let accounts;
  try {
    accounts = await ethereum.request({ method: "eth_requestAccounts" });
  } catch (err) {
    if (err?.code === 4001) {
      throw new Error("User rejected wallet connection");
    }
    throw err;
  }

  const account = Array.isArray(accounts) && accounts.length > 0 ? accounts[0] : null;
  if (!account) {
    throw new Error("No accounts available");
  }

  const provider = new ethers.BrowserProvider(ethereum);
  const signer = await provider.getSigner();
  return { signer, address: account };
}

/**
 * Get the connected wallet address (or null if not connected).
 */
async function getWalletAddress() {
  try {
    const ethereum = await getEthereum();
    const accounts = await ethereum.request({ method: "eth_accounts" });
    return Array.isArray(accounts) && accounts.length > 0 ? accounts[0] : null;
  } catch (_e) {
    return null;
  }
}

/**
 * Execute on-chain BrandToken.claim() with server-signed parameters.
 *
 * @param {object} params
 * @param {string} params.contract_address - BrandToken contract address
 * @param {number} params.chain_id - Target chain (1001 or 8217)
 * @param {string} params.month_index - Month index (uint256 as string)
 * @param {string} params.amount - Claim amount (uint256 as string)
 * @param {string} params.max_claimable - Max claimable (uint256 as string)
 * @param {string} params.nonce - Nonce (uint256 as string)
 * @param {string} params.deadline - Deadline timestamp (uint256 as string)
 * @param {string} params.signature - Server EIP-712 signature (0x-prefixed hex)
 * @returns {Promise<{tx_hash: string, address: string}>}
 */
async function claimTokens(params) {
  const { signer, address } = await connectAndGetSigner(params.chain_id);

  const contract = new ethers.Contract(
    params.contract_address,
    BRAND_TOKEN_ABI,
    signer,
  );

  const tx = await contract.claim(
    BigInt(params.month_index),
    BigInt(params.amount),
    BigInt(params.max_claimable),
    BigInt(params.nonce),
    BigInt(params.deadline),
    params.signature,
  );

  const receipt = await tx.wait();
  return {
    tx_hash: receipt?.hash || tx.hash,
    address,
  };
}

/**
 * Connect wallet + switch to the correct network. Returns the address.
 * This triggers Kaia Wallet / MetaMask popup.
 */
async function connectWallet(chainId) {
  const { address } = await connectAndGetSigner(chainId);
  return address;
}

const claim = {
  getWalletAddress,
  connectWallet,
  claimTokens,
};

export default claim;
