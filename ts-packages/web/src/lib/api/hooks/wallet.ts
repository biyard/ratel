import { useState } from 'react';
import { ethers } from 'ethers';
import { logger } from '@/lib/logger';
import { usePopup } from '@/lib/contexts/popup-service';

interface EthereumProvider {
  isMetaMask?: boolean;
  request: (args: { method: string; params?: unknown[] }) => Promise<unknown>;
  on?: (...args: unknown[]) => void;
  removeListener?: (...args: unknown[]) => void;
}

export function useWallet() {
  const [address, setAddress] = useState<string | null>(null);
  const popup = usePopup();

  const connectWallet = async () => {
    const ethereum = (window as unknown as { ethereum?: EthereumProvider })
      .ethereum;

    if (!ethereum) {
      popup
        .open('Please Install MetaMask Extension')
        .withTitle('Wallet Not Found');
      return;
    }

    try {
      const provider = new ethers.BrowserProvider(ethereum);
      const signer = await provider.getSigner();
      const walletAddress = await signer.getAddress();

      setAddress(walletAddress);

      ethereum.on?.('accountsChanged', (accounts: string[]) => {
        if (accounts.length > 0) {
          setAddress(accounts[0]);
          logger.debug('Evm_address updated due to account change');
        } else {
          setAddress(null);
          logger.debug('Evm_address cleared due to account change');
        }
      });

      ethereum.on?.('chainChanged', () => {
        setAddress(null);
        logger.debug('Evm_address cleared due to chain change');
      });

      logger.debug('Evm_address updated successfully');
    } catch (err) {
      logger.error('Wallet connection failed:', err);
    }
  };

  return { address, connectWallet };
}
