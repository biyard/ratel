import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { SpaceDaoResponse } from '@/features/spaces/dao/hooks/use-space-dao';
import { BlockchainService } from '@/contracts/BlockchainService';
import { config } from '@/config';
import { ethers } from 'ethers';
import { showErrorToast, showInfoToast, showSuccessToast } from '@/lib/toast';
import {
  getKaiaSigner,
  KaiaWalletError,
} from '@/lib/service/kaia-wallet-service';
import { State } from '@/types/state';

export class SpaceDaoViewerController {
  constructor(
    public spacePk: string,
    public space: Space | undefined,
    public dao: SpaceDaoResponse | null | undefined,
    public t: TFunction<'SpaceDaoEditor', undefined>,
    public provider: ethers.JsonRpcProvider | null,
    public balance: State<string | null>,
    public balanceLoading: State<boolean>,
    public isDepositOpen: State<boolean>,
    public depositAmount: State<string>,
    public isDepositing: State<boolean>,
  ) {}

  fetchBalance = async () => {
    if (!this.provider || !this.dao?.contract_address) {
      return;
    }

    this.balanceLoading.set(true);

    try {
      const service = new BlockchainService(this.provider);
      const raw = await service.getSpaceBalance(this.dao.contract_address);
      const formatted = ethers.formatUnits(raw, 6);
      this.balance.set(formatted);
    } catch (error) {
      console.error('Failed to fetch Space DAO balance:', error);
      this.balance.set(null);
    } finally {
      this.balanceLoading.set(false);
    }
  };

  handleOpenDeposit = () => {
    this.isDepositOpen.set(true);
  };

  handleCloseDeposit = () => {
    if (!this.isDepositing.get()) {
      this.isDepositOpen.set(false);
    }
  };

  handleDepositAmountChange = (value: string) => {
    this.depositAmount.set(value);
  };

  handleConfirmDeposit = async () => {
    const dao = this.dao;
    const amount = Number(this.depositAmount.get());
    if (!dao?.contract_address || !Number.isFinite(amount) || amount <= 0) {
      showErrorToast(this.t('error_invalid_deposit_amount'));
      return;
    }

    this.isDepositing.set(true);

    try {
      showInfoToast(this.t('toast_connecting_wallet'));
      const signer = await getKaiaSigner(
        config.env === 'prod' ? 'mainnet' : 'testnet',
      );
      const walletProvider = signer.provider;

      showInfoToast(this.t('toast_depositing'));
      const service = new BlockchainService(walletProvider);
      await service.connectWallet();
      await service.spaceDeposit(
        dao.contract_address,
        this.depositAmount.get(),
      );

      showSuccessToast(this.t('toast_deposit_success'));
      this.isDepositOpen.set(false);
      this.depositAmount.set('');
      await this.fetchBalance();
    } catch (error) {
      console.error('Failed to deposit to Space DAO:', error);

      if (error instanceof KaiaWalletError) {
        if (error.code === 'USER_REJECTED') {
          showErrorToast(this.t('error_wallet_rejected'));
        } else if (error.code === 'METAMASK_NOT_INSTALLED') {
          showErrorToast(this.t('error_wallet_not_installed'));
        } else {
          showErrorToast(
            this.t('error_wallet_generic', { message: error.message }),
          );
        }
      } else if (error instanceof Error) {
        showErrorToast(
          this.t('error_deposit_failed', { message: error.message }),
        );
      } else {
        showErrorToast(this.t('error_register_failed_unknown'));
      }
    } finally {
      this.isDepositing.set(false);
    }
  };
}

export function useSpaceDaoViewerController(
  spacePk: string,
  dao?: SpaceDaoResponse | null,
) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceDaoEditor');
  const balance = useState<string | null>(null);
  const balanceLoading = useState(false);
  const isDepositOpen = useState(false);
  const depositAmount = useState('');
  const isDepositing = useState(false);
  const provider = useMemo(() => {
    if (!config.rpc_url) {
      return null;
    }
    return new ethers.JsonRpcProvider(config.rpc_url);
  }, []);

  const ctrl = new SpaceDaoViewerController(
    spacePk,
    space,
    dao,
    t,
    provider,
    new State(balance),
    new State(balanceLoading),
    new State(isDepositOpen),
    new State(depositAmount),
    new State(isDepositing),
  );

  useEffect(() => {
    void ctrl.fetchBalance();
  }, [dao?.contract_address, provider]);

  return ctrl;
}
