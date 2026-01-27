import { useEffect, useMemo, useState } from 'react';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { SpaceDaoResponse } from '@/features/spaces/dao/hooks/use-space-dao';
import { BlockchainService } from '@/contracts/BlockchainService';
import { config } from '@/config';
import { ethers } from 'ethers';

export class SpaceDaoViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public balance: string | null,
    public balanceLoading: boolean,
  ) {}
}

export function useSpaceDaoViewerController(
  spacePk: string,
  dao?: SpaceDaoResponse | null,
) {
  const { data: space } = useSpaceById(spacePk);
  const [balance, setBalance] = useState<string | null>(null);
  const [balanceLoading, setBalanceLoading] = useState(false);
  const provider = useMemo(() => {
    if (!config.rpc_url) {
      return null;
    }
    return new ethers.JsonRpcProvider(config.rpc_url);
  }, []);

  useEffect(() => {
    if (!provider || !dao?.contract_address) {
      return;
    }

    let mounted = true;
    setBalanceLoading(true);

    const fetchBalance = async () => {
      try {
        const service = new BlockchainService(provider);
        const raw = await service.getSpaceBalance(dao.contract_address);
        const formatted = ethers.formatUnits(raw, 6);
        if (mounted) {
          setBalance(formatted);
        }
      } catch (error) {
        console.error('Failed to fetch Space DAO balance:', error);
        if (mounted) {
          setBalance(null);
        }
      } finally {
        if (mounted) {
          setBalanceLoading(false);
        }
      }
    };

    fetchBalance();

    return () => {
      mounted = false;
    };
  }, [dao?.contract_address, provider]);

  return new SpaceDaoViewerController(spacePk, space, balance, balanceLoading);
}
