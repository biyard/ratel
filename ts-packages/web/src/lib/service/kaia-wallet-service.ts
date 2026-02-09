'use client';

import { ethers } from 'ethers';
import { config } from '@/config';

export const KAIA_TESTNET_CHAIN_ID = '0x3e9'; // 1001
export const KAIA_MAINNET_CHAIN_ID = '0x2019'; // 8217

export type KaiaNetwork = 'testnet' | 'mainnet';

export class KaiaWalletError extends Error {
  code: string;

  constructor(code: string, message: string) {
    super(message);
    this.code = code;
  }
}

function getTargetChainId(network: KaiaNetwork): string {
  return network === 'mainnet' ? KAIA_MAINNET_CHAIN_ID : KAIA_TESTNET_CHAIN_ID;
}

async function getEthereum() {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const { ethereum } = window as any;
  if (!ethereum || !ethereum.request) {
    throw new KaiaWalletError(
      'METAMASK_NOT_INSTALLED',
      'MetaMask is not installed',
    );
  }
  return ethereum;
}

function chainsEqual(a: string, b: string): boolean {
  try {
    return parseInt(a, 16) === parseInt(b, 16);
  } catch {
    return a === b;
  }
}

export async function ensureKaiaNetwork(target: KaiaNetwork = 'testnet') {
  const ethereum = await getEthereum();
  const targetChainId = getTargetChainId(target);

  let chainIdHex: string = await ethereum.request({ method: 'eth_chainId' });

  if (chainIdHex === targetChainId) {
    return { ethereum, chainId: chainIdHex };
  }

  try {
    await ethereum.request({
      method: 'wallet_switchEthereumChain',
      params: [{ chainId: targetChainId }],
    });
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (switchError: any) {
    if (switchError?.code === 4902) {
      const params =
        target === 'mainnet'
          ? {
              chainId: KAIA_MAINNET_CHAIN_ID,
              chainName: 'Kaia Mainnet',
              nativeCurrency: {
                name: 'KAIA',
                symbol: 'KAIA',
                decimals: 18,
              },
              rpcUrls: [config.rpc_url].filter(Boolean),
              blockExplorerUrls: [config.block_explorer_url].filter(Boolean),
            }
          : {
              chainId: KAIA_TESTNET_CHAIN_ID,
              chainName: 'Kaia Kairos Testnet',
              nativeCurrency: {
                name: 'KAIA',
                symbol: 'KAIA',
                decimals: 18,
              },
              rpcUrls: [config.rpc_url].filter(Boolean),
              blockExplorerUrls: [config.block_explorer_url].filter(Boolean),
            };

      try {
        await ethereum.request({
          method: 'wallet_addEthereumChain',
          params: [params],
        });
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
      } catch (addError: any) {
        const msg: string =
          addError?.data?.cause?.message || addError?.message || '';
        if (
          msg.includes('same RPC endpoint as existing network') ||
          addError?.code === -32603
        ) {
          // nothing
        } else if (addError?.code === 4001) {
          throw new KaiaWalletError(
            'USER_REJECTED',
            'User rejected network add',
          );
        } else {
          throw addError;
        }
      }
    } else if (switchError?.code === 4001) {
      throw new KaiaWalletError(
        'USER_REJECTED',
        'User rejected network switch',
      );
    } else {
      throw switchError;
    }
  }

  chainIdHex = await ethereum.request({ method: 'eth_chainId' });

  if (!chainsEqual(chainIdHex, targetChainId)) {
    throw new KaiaWalletError(
      'CHAIN_SWITCH_FAILED',
      `Failed to switch to Kaia network (current=${chainIdHex}, target=${targetChainId})`,
    );
  }

  return { ethereum, chainId: chainIdHex };
}

export async function getKaiaSigner(target: KaiaNetwork = 'testnet') {
  const { ethereum } = await ensureKaiaNetwork(target);

  let accounts: string[];
  try {
    accounts = await ethereum.request({
      method: 'eth_requestAccounts',
    });
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (e: any) {
    if (e?.code === 4001) {
      throw new KaiaWalletError(
        'USER_REJECTED',
        'User rejected wallet connection',
      );
    }
    throw e;
  }

  if (!accounts || accounts.length === 0) {
    throw new KaiaWalletError(
      'NO_ACCOUNTS',
      'Wallet connection cancelled or no accounts',
    );
  }

  const provider = new ethers.BrowserProvider(ethereum);
  const signer = await provider.getSigner();

  return {
    provider,
    signer,
    account: accounts[0] as string,
  };
}

export async function getKaiaAccount(): Promise<string | null> {
  try {
    const ethereum = await getEthereum();
    const accounts: string[] = await ethereum.request({
      method: 'eth_accounts',
    });
    if (!accounts || accounts.length === 0) {
      return null;
    }
    return accounts[0] as string;
  } catch {
    return null;
  }
}
