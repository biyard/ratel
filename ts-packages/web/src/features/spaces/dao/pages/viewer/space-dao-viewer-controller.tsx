import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { SpaceDaoResponse } from '@/features/spaces/dao/hooks/use-space-dao';
import { SpaceDaoService } from '@/contracts/SpaceDaoService';
import { config } from '@/config';
import { ethers } from 'ethers';
import { showErrorToast, showInfoToast, showSuccessToast } from '@/lib/toast';
import {
  getKaiaSigner,
  KaiaWalletError,
} from '@/lib/service/kaia-wallet-service';
import { State } from '@/types/state';
import {
  SpaceDaoSampleListResponse,
  useSpaceDaoSamples,
} from '@/features/spaces/dao/hooks/use-space-dao-samples';
import { useUpdateSpaceDaoSamplesMutation } from '@/features/spaces/dao/hooks/use-update-space-dao-samples-mutation';

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
    public sampleBookmark: State<string | null>,
    public sampleHistory: State<(string | null)[]>,
    public samples: SpaceDaoSampleListResponse | undefined,
    public samplesLoading: boolean,
    public isDistributingPage: State<boolean>,
  ) {}

  fetchBalance = async () => {
    if (!this.provider || !this.dao?.contract_address) {
      return;
    }

    this.balanceLoading.set(true);

    try {
      const service = new SpaceDaoService(this.provider);
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
      const service = new SpaceDaoService(walletProvider);
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

  get canDistributeReward() {
    return this.space?.isAdmin?.() ?? false;
  }

  get canPrevSample() {
    return this.sampleHistory.get().length > 0;
  }

  get canNextSample() {
    return Boolean(this.samples?.bookmark);
  }

  get visibleSamples() {
    return this.samples?.items ?? [];
  }

  handleNextSample = () => {
    const next = this.samples?.bookmark ?? null;
    if (!next) return;
    const history = [...this.sampleHistory.get()];
    history.push(this.sampleBookmark.get());
    this.sampleHistory.set(history);
    this.sampleBookmark.set(next);
  };

  handlePrevSample = () => {
    const history = [...this.sampleHistory.get()];
    if (history.length === 0) return;
    const prev = history.pop() ?? null;
    this.sampleHistory.set(history);
    this.sampleBookmark.set(prev);
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
  const sampleBookmark = useState<string | null>(null);
  const sampleHistory = useState<(string | null)[]>([]);
  const isDistributingPage = useState(false);
  const { data: samples, isLoading: samplesLoading } = useSpaceDaoSamples(
    spacePk,
    sampleBookmark[0],
    50,
    Boolean(dao?.contract_address),
  );
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
    new State(sampleBookmark),
    new State(sampleHistory),
    samples,
    samplesLoading,
    new State(isDistributingPage),
  );

  useEffect(() => {
    void ctrl.fetchBalance();
  }, [dao?.contract_address, provider]);

  return ctrl;
}
