import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { useDaoData } from '@/features/teams/dao/hooks/use-dao-data';
import { UserType } from '@/lib/api/ratel/users.v3';
import { showErrorToast, showInfoToast, showSuccessToast } from '@/lib/toast';
import {
  getKaiaSigner,
  KaiaWalletError,
} from '@/lib/service/kaia-wallet-service';
import { SpaceDaoService } from '@/contracts/SpaceDaoService';
import { config } from '@/config';
import { useCreateSpaceDaoMutation } from '@/features/spaces/dao/hooks/use-create-space-dao-mutation';
import { SpaceDaoResponse } from '@/features/spaces/dao/hooks/use-space-dao';
import { ethers } from 'ethers';
import { State } from '@/types/state';
import {
  SpaceDaoRewardListResponse,
  useSpaceDaoReward,
} from '@/features/spaces/dao/hooks/use-space-dao-reward';
import { useUpdateSpaceDaoRewardMutation } from '@/features/spaces/dao/hooks/use-update-space-dao-reward-mutation';

export class SpaceDaoEditorController {
  constructor(
    public spacePk: string,
    public space: Space | undefined,
    public dao: SpaceDaoResponse | null | undefined,
    public eligibleAdmins: ReturnType<typeof useDaoData>['eligibleAdmins'],
    public permissions: ReturnType<typeof useDaoData>['permissions'],
    public t: TFunction<'SpaceDaoEditor', undefined>,
    public provider: ethers.JsonRpcProvider | null,
    public adminAddresses: State<string>,
    public rewardCount: State<string>,
    public chainRecipientCount: State<string | null>,
    public isPopupOpen: State<boolean>,
    public isRegistering: State<boolean>,
    public isUpdating: State<boolean>,
    public rewardBookmark: State<string | null>,
    public rewardHistory: State<(string | null)[]>,
    public reward: SpaceDaoRewardListResponse | undefined,
    public rewardLoading: boolean,
    public isDistributingPage: State<boolean>,
    public selectedToken: string | null,
    public tokenBalance: string | null,
    public tokenDecimals: number | null,
    public preferredToken: string | null,
    public preferredTokenBalance: string | null,
    public preferredTokenDecimals: number | null,
    public createSpaceDaoMutation: ReturnType<typeof useCreateSpaceDaoMutation>,
    public updateRewardMutation: ReturnType<
      typeof useUpdateSpaceDaoRewardMutation
    >,
  ) {}

  get isTeamSpace() {
    return (this.space?.authorType ?? null) === UserType.Team;
  }

  get canRegisterDao() {
    return this.eligibleAdmins.length >= 3 && this.permissions?.isAdmin();
  }

  get canSubmitInputs() {
    const count = Number(this.rewardCount.get());
    return Number.isFinite(count) && count > 0;
  }

  get canDistributeReward() {
    return this.permissions?.isAdmin() ?? false;
  }

  get canPrevReward() {
    if (this.space?.isFinished) {
      return false;
    }
    return this.rewardHistory.get().length > 0;
  }

  get canNextReward() {
    if (this.space?.isFinished) {
      return false;
    }
    return Boolean(this.reward?.bookmark);
  }

  get visibleRewardRecipients() {
    return this.reward?.items ?? [];
  }

  get rewardRecipients() {
    return this.reward;
  }

  get rewardRecipientsLoading() {
    return this.rewardLoading;
  }

  fetchRecipientCount = async () => {
    if (!this.provider || !this.dao?.contract_address) {
      return;
    }

    try {
      const service = new SpaceDaoService(this.provider);
      const count = await service.getRewardRecipientCount(
        this.dao.contract_address,
      );
      this.chainRecipientCount.set(String(count));
    } catch (error) {
      console.error('Failed to fetch reward recipient count:', error);
      this.chainRecipientCount.set(null);
    }
  };

  handleNextReward = () => {
    const next = this.reward?.bookmark ?? null;
    if (!next) return;
    const history = [...this.rewardHistory.get()];
    history.push(this.rewardBookmark.get());
    this.rewardHistory.set(history);
    this.rewardBookmark.set(next);
  };

  handlePrevReward = () => {
    const history = [...this.rewardHistory.get()];
    if (history.length === 0) return;
    const prev = history.pop() ?? null;
    this.rewardHistory.set(history);
    this.rewardBookmark.set(prev);
  };

  handleDistribute = async () => {
    if (!this.canDistributeReward) return;
    const dao = this.dao;
    if (!dao?.contract_address) return;

    const tokenAddress = this.preferredToken ?? this.selectedToken;
    const balance = this.preferredTokenBalance ?? this.tokenBalance;
    const decimals = this.preferredTokenDecimals ?? this.tokenDecimals;
    if (!tokenAddress || !balance || decimals == null) {
      showErrorToast(this.t('error_register_failed_unknown'));
      return;
    }
    const candidates = this.visibleRewardRecipients.filter(
      (item) => !item.reward_distributed,
    );
    if (candidates.length === 0) {
      return;
    }
    const recipients = candidates.map((item) => item.evm_address);
    const rewardSks = candidates.map((item) => item.sk);

    this.isDistributingPage.set(true);
    try {
      showInfoToast(this.t('toast_connecting_wallet'));
      const signer = await getKaiaSigner(
        config.env === 'prod' ? 'mainnet' : 'testnet',
      );
      const provider = signer.provider;
      const daoService = new SpaceDaoService(provider);
      await daoService.connectWallet();
      const totalBalance = ethers.toBigInt(balance);
      const totalCount = this.reward?.remaining_count ?? candidates.length;
      if (totalCount <= 0) {
        showErrorToast(this.t('error_register_failed_unknown'));
        return;
      }
      const perRecipient = totalBalance / BigInt(totalCount);
      if (perRecipient <= 0n) {
        showErrorToast(this.t('error_register_failed_unknown'));
        return;
      }
      await daoService.distribute(
        dao.contract_address,
        tokenAddress,
        recipients,
        perRecipient,
      );

      await this.updateRewardMutation.mutateAsync({
        spacePk: this.spacePk,
        rewardSks,
        rewardDistributed: true,
      });

      showSuccessToast(this.t('toast_reward_distributed'));
    } catch (error) {
      console.error('Failed to distribute reward:', error);
      if (error instanceof Error) {
        showErrorToast(
          this.t('error_reward_distribute_failed', { message: error.message }),
        );
      } else {
        showErrorToast(this.t('error_register_failed_unknown'));
      }
    } finally {
      this.isDistributingPage.set(false);
    }
  };

  handleOpenRegistrationPopup = () => {
    if (!this.canRegisterDao) {
      showErrorToast(this.t('error_insufficient_admins'));
      return;
    }
    if (!this.canSubmitInputs) {
      showErrorToast(this.t('error_missing_inputs'));
      return;
    }
    this.isPopupOpen.set(true);
  };

  handleClosePopup = () => {
    if (!this.isRegistering.get()) {
      this.isPopupOpen.set(false);
    }
  };

  handleRegisterDao = async (selectedAdminAddresses: string[]) => {
    if (selectedAdminAddresses.length < 3) {
      showErrorToast(this.t('error_invalid_admin_selection'));
      return;
    }

    this.isRegistering.set(true);

    try {
      showInfoToast(this.t('toast_connecting_wallet'));
      const signer = await getKaiaSigner(
        config.env === 'prod' ? 'mainnet' : 'testnet',
      );
      const provider = signer.provider;

      showInfoToast(this.t('toast_creating_dao'));
      const daoService = new SpaceDaoService(provider);
      await daoService.connectWallet();

      const count = Number(this.rewardCount.get());
      const result = await daoService.createSpaceDAO(
        selectedAdminAddresses,
        count,
      );

      await this.createSpaceDaoMutation.mutateAsync({
        spacePk: this.spacePk,
        req: {
          contract_address: result.daoAddress,
          deploy_block: result.deployBlock,
        },
      });

      this.chainRecipientCount.set(String(count));
      showSuccessToast(this.t('toast_registered'));
      this.isPopupOpen.set(false);
    } catch (error) {
      console.error('Failed to register Space DAO:', error);

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
          this.t('error_register_failed', { message: error.message }),
        );
      } else {
        showErrorToast(this.t('error_register_failed_unknown'));
      }
    } finally {
      this.isRegistering.set(false);
    }
  };

  handleUpdateDao = async (rewardCount: string) => {
    const dao = this.dao;
    const count = Number(rewardCount);

    if (!dao?.contract_address) {
      showErrorToast(this.t('error_register_failed_unknown'));
      return;
    }
    if (!Number.isFinite(count) || count <= 0) {
      showErrorToast(this.t('error_invalid_reward_count'));
      return;
    }

    this.isUpdating.set(true);
    try {
      showInfoToast(this.t('toast_connecting_wallet'));
      const signer = await getKaiaSigner(
        config.env === 'prod' ? 'mainnet' : 'testnet',
      );
      const provider = signer.provider;

      const daoService = new SpaceDaoService(provider);
      await daoService.connectWallet();
      await daoService.setRewardRecipientCount(dao.contract_address, count);
      this.chainRecipientCount.set(String(count));
      showSuccessToast(this.t('toast_updated'));
    } catch (error) {
      console.error('Failed to update Space DAO:', error);
      if (error instanceof Error) {
        showErrorToast(
          this.t('error_register_failed', { message: error.message }),
        );
      } else {
        showErrorToast(this.t('error_register_failed_unknown'));
      }
    } finally {
      this.isUpdating.set(false);
    }
  };
}

export function useSpaceDaoEditorController(
  spacePk: string,
  dao?: SpaceDaoResponse | null,
  selectedToken?: string | null,
  tokenBalance?: string | null,
  tokenDecimals?: number | null,
  preferredToken?: string | null,
  preferredTokenBalance?: string | null,
  preferredTokenDecimals?: number | null,
) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceDaoEditor');
  const adminAddresses = useState('');
  const rewardCount = useState('');
  const chainRecipientCount = useState<string | null>(null);
  const isPopupOpen = useState(false);
  const isRegistering = useState(false);
  const isUpdating = useState(false);
  const rewardBookmark = useState<string | null>(null);
  const rewardHistory = useState<(string | null)[]>([]);
  const isDistributingPage = useState(false);
  const createSpaceDaoMutation = useCreateSpaceDaoMutation();
  const updateRewardMutation = useUpdateSpaceDaoRewardMutation();
  const { data: reward, isLoading: rewardLoading } = useSpaceDaoReward(
    spacePk,
    rewardBookmark[0],
    50,
    Boolean(dao?.contract_address),
  );
  const provider = useMemo(() => {
    if (!config.rpc_url) {
      return null;
    }
    return new ethers.JsonRpcProvider(config.rpc_url);
  }, []);

  const teamUsername = space?.authorUsername ?? '';
  const { eligibleAdmins, permissions } = useDaoData(teamUsername);

  const ctrl = new SpaceDaoEditorController(
    spacePk,
    space,
    dao,
    eligibleAdmins,
    permissions,
    t,
    provider,
    new State(adminAddresses),
    new State(rewardCount),
    new State(chainRecipientCount),
    new State(isPopupOpen),
    new State(isRegistering),
    new State(isUpdating),
    new State(rewardBookmark),
    new State(rewardHistory),
    reward,
    rewardLoading,
    new State(isDistributingPage),
    selectedToken ?? null,
    tokenBalance ?? null,
    tokenDecimals ?? null,
    preferredToken ?? null,
    preferredTokenBalance ?? null,
    preferredTokenDecimals ?? null,
    createSpaceDaoMutation,
    updateRewardMutation,
  );

  useEffect(() => {
    void ctrl.fetchRecipientCount();
  }, [dao?.contract_address, provider]);

  return ctrl;
}
