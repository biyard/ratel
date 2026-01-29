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
import { getKaiaAccount } from '@/lib/service/kaia-wallet-service';
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
    public withdrawAmount: State<string>,
    public isWithdrawing: State<boolean>,
    public proposals: State<
      {
        id: number;
        proposer: string;
        amount: string;
        approvals: number;
        executed: boolean;
        approvedByMe: boolean;
      }[]
    >,
    public proposalsLoading: State<boolean>,
    public isApprovingWithdrawal: State<boolean>,
    public availableShare: State<string | null>,
    public availableShareLoading: State<boolean>,
    public depositorCount: State<number | null>,
    public isDepositor: State<boolean>,
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

  fetchWithdrawData = async (walletAddress?: string) => {
    if (!this.provider || !this.dao?.contract_address) {
      return;
    }

    this.proposalsLoading.set(true);
    this.availableShareLoading.set(true);

    try {
      const service = new SpaceDaoService(this.provider);
      const [count, depositorCount] = await Promise.all([
        service.getShareWithdrawProposalCount(this.dao.contract_address),
        service.getDepositorCount(this.dao.contract_address),
      ]);

      const proposals = await Promise.all(
        Array.from({ length: count }, (_, id) =>
          service.getShareWithdrawProposal(this.dao!.contract_address, id),
        ),
      );

      const targetAddress =
        walletAddress ?? (await getKaiaAccount()) ?? null;
      const approvalsByMe = targetAddress
        ? await Promise.all(
            Array.from({ length: count }, (_, id) =>
              service.isShareWithdrawApproved(
                this.dao!.contract_address,
                id,
                targetAddress,
              ),
            ),
          )
        : [];

      const formatted = proposals.map((p, id) => ({
        id,
        proposer: p.proposer,
        amount: ethers.formatUnits(p.amount, 6),
        approvals: Number(p.approvals),
        executed: p.executed,
        approvedByMe: approvalsByMe[id] ?? false,
      }));

      this.proposals.set(formatted);
      this.depositorCount.set(depositorCount);

      if (targetAddress) {
        const raw = await service.getAvailableShare(
          this.dao.contract_address,
          targetAddress,
        );
        this.availableShare.set(ethers.formatUnits(raw, 6));
        const depositRaw = await service.getDepositorDeposit(
          this.dao.contract_address,
          targetAddress,
        );
        this.isDepositor.set(BigInt(depositRaw) > 0n);
      } else {
        this.availableShare.set(null);
        this.isDepositor.set(false);
      }
    } catch (error) {
      console.error('Failed to fetch withdrawal data:', error);
    } finally {
      this.proposalsLoading.set(false);
      this.availableShareLoading.set(false);
    }
  };

  handleWithdrawAmountChange = (value: string) => {
    this.withdrawAmount.set(value);
  };

  handleProposeWithdrawal = async () => {
    const dao = this.dao;
    const amount = Number(this.withdrawAmount.get());
    if (!dao?.contract_address || !Number.isFinite(amount) || amount <= 0) {
      showErrorToast(this.t('error_invalid_withdraw_amount'));
      return;
    }

    this.isWithdrawing.set(true);
    try {
      showInfoToast(this.t('toast_connecting_wallet'));
      const signer = await getKaiaSigner(
        config.env === 'prod' ? 'mainnet' : 'testnet',
      );
      const walletProvider = signer.provider;
      const address = signer.account;

      const service = new SpaceDaoService(walletProvider);
      await service.connectWallet();
      await service.proposeShareWithdrawal(
        dao.contract_address,
        this.withdrawAmount.get(),
      );

      showSuccessToast(this.t('toast_withdraw_proposed'));
      this.withdrawAmount.set('');
      await this.fetchWithdrawData(address);
      await this.fetchBalance();
    } catch (error) {
      console.error('Failed to propose withdrawal:', error);
      if (error instanceof Error) {
        showErrorToast(
          this.t('error_withdraw_failed', { message: error.message }),
        );
      } else {
        showErrorToast(this.t('error_register_failed_unknown'));
      }
    } finally {
      this.isWithdrawing.set(false);
    }
  };

  handleApproveWithdrawal = async (id: number) => {
    const dao = this.dao;
    if (!dao?.contract_address) {
      return;
    }

    this.isApprovingWithdrawal.set(true);
    try {
      showInfoToast(this.t('toast_connecting_wallet'));
      const signer = await getKaiaSigner(
        config.env === 'prod' ? 'mainnet' : 'testnet',
      );
      const walletProvider = signer.provider;
      const address = signer.account;

      const service = new SpaceDaoService(walletProvider);
      await service.connectWallet();
      await service.approveShareWithdrawal(dao.contract_address, id);

      showSuccessToast(this.t('toast_withdraw_approved'));
      await this.fetchWithdrawData(address);
      await this.fetchBalance();
    } catch (error) {
      console.error('Failed to approve withdrawal:', error);
      if (error instanceof Error) {
        showErrorToast(
          this.t('error_withdraw_approve_failed', { message: error.message }),
        );
      } else {
        showErrorToast(this.t('error_register_failed_unknown'));
      }
    } finally {
      this.isApprovingWithdrawal.set(false);
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
  const withdrawAmount = useState('');
  const isWithdrawing = useState(false);
  const proposals = useState<
    {
      id: number;
      proposer: string;
      amount: string;
      approvals: number;
      executed: boolean;
      approvedByMe: boolean;
    }[]
  >([]);
  const proposalsLoading = useState(false);
  const isApprovingWithdrawal = useState(false);
  const availableShare = useState<string | null>(null);
  const availableShareLoading = useState(false);
  const depositorCount = useState<number | null>(null);
  const isDepositor = useState(false);
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
    new State(withdrawAmount),
    new State(isWithdrawing),
    new State(proposals),
    new State(proposalsLoading),
    new State(isApprovingWithdrawal),
    new State(availableShare),
    new State(availableShareLoading),
    new State(depositorCount),
    new State(isDepositor),
    new State(sampleBookmark),
    new State(sampleHistory),
    samples,
    samplesLoading,
    new State(isDistributingPage),
  );

  useEffect(() => {
    void ctrl.fetchBalance();
    void ctrl.fetchWithdrawData();
  }, [dao?.contract_address, provider]);

  return ctrl;
}
