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
  SpaceDaoIncentiveResponseBody,
  useSpaceDaoIncentive,
} from '@/features/spaces/dao/hooks/use-space-dao-incentive';
import { useUpdateSpaceDaoIncentiveMutation } from '@/features/spaces/dao/hooks/use-update-space-dao-incentive-mutation';
import { useUserInfo } from '@/hooks/use-user-info';

export class SpaceDaoEditorController {
  constructor(
    public spacePk: string,
    public space: Space | undefined,
    public dao: SpaceDaoResponse | null | undefined,
    public eligibleAdmins: ReturnType<typeof useDaoData>['eligibleAdmins'],
    public teamMembers: ReturnType<typeof useDaoData>['members'],
    public permissions: ReturnType<typeof useDaoData>['permissions'],
    public t: TFunction<'SpaceDaoEditor', undefined>,
    public provider: ethers.JsonRpcProvider | null,
    public adminAddresses: State<string>,
    public incentiveCount: State<string>,
    public incentiveMode: State<number>,
    public rankingBps: State<string>,
    public chainRecipientCount: State<string | null>,
    public chainIncentiveMode: State<number | null>,
    public chainRankingBps: State<number | null>,
    public isPopupOpen: State<boolean>,
    public isRegistering: State<boolean>,
    public isUpdating: State<boolean>,
    public incentiveData: SpaceDaoIncentiveResponseBody | undefined,
    public incentiveLoading: boolean,
    public currentUserEvm: string | null,
    public isIncentiveRecipient: State<boolean>,
    public isIncentiveClaimed: State<boolean>,
    public isClaiming: State<boolean>,
    public claimAmountRaw: State<string | null>,
    public selectedToken: string | null,
    public tokenBalance: string | null,
    public tokenDecimals: number | null,
    public preferredToken: string | null,
    public preferredTokenBalance: string | null,
    public preferredTokenDecimals: number | null,
    public createSpaceDaoMutation: ReturnType<typeof useCreateSpaceDaoMutation>,
    public updateIncentiveMutation: ReturnType<
      typeof useUpdateSpaceDaoIncentiveMutation
    >,
  ) {}

  get isTeamSpace() {
    return (this.space?.authorType ?? null) === UserType.Team;
  }

  get canRegisterDao() {
    if (this.isTeamSpace) {
      if (!this.permissions?.isAdmin()) {
        return false;
      }
      return this.teamMembers.some((member) => Boolean(member.evm_address));
    }
    return Boolean(this.currentUserEvm);
  }

  get canSubmitInputs() {
    const count = Number(this.incentiveCount.get());
    if (!Number.isFinite(count) || count <= 0 || count > 100) {
      return false;
    }
    if (this.incentiveMode.get() !== 2) {
      return true;
    }
    const ratio = Number(this.rankingBps.get());
    return Number.isFinite(ratio) && ratio >= 0 && ratio <= 100;
  }

  get visibleIncentiveRecipients() {
    return this.incentiveRecipients?.items ?? [];
  }

  get incentiveRecipients() {
    const item = this.incentiveData?.item;
    return item ? { items: [item] } : undefined;
  }

  get incentiveRecipientsLoading() {
    return this.incentiveLoading;
  }

  get activeTokenAddress() {
    return this.preferredToken ?? this.selectedToken ?? null;
  }

  get activeTokenBalance() {
    return this.preferredTokenBalance ?? this.tokenBalance ?? null;
  }

  get activeTokenDecimals() {
    return this.preferredTokenDecimals ?? this.tokenDecimals ?? null;
  }

  get incentiveMeta() {
    return this.incentiveData;
  }

  get remainingIncentiveCount() {
    const meta = this.incentiveMeta;
    const remaining = meta?.remaining_count;
    if (remaining != null && remaining > 0) return remaining;
    const total = meta?.total_count;
    if (total != null && total > 0) return total;
    return 0;
  }

  get perRecipientAmount() {
    const raw = this.claimAmountRaw.get();
    if (!raw) return null;
    try {
      return BigInt(raw);
    } catch {
      return null;
    }
  }

  get perRecipientDisplay() {
    const perRecipient = this.perRecipientAmount;
    const decimals = this.activeTokenDecimals ?? 0;
    if (perRecipient == null) return null;
    try {
      return ethers.formatUnits(perRecipient, decimals);
    } catch {
      return perRecipient.toString();
    }
  }

  get canClaimIncentive() {
    if (!this.currentUserEvm) return false;
    if (!this.isIncentiveRecipient.get() || this.isIncentiveClaimed.get())
      return false;
    return this.perRecipientAmount != null && this.perRecipientAmount > 0n;
  }

  fetchRecipientCount = async () => {
    if (!this.provider || !this.dao?.contract_address) {
      return;
    }

    try {
      const service = new SpaceDaoService(this.provider);
      const config = await service.getIncentiveDistributionConfig(
        this.dao.contract_address,
      );
      this.chainRecipientCount.set(String(config.numOfTargets));
      this.chainIncentiveMode.set(config.mode);
      this.chainRankingBps.set(config.rankingBps);
    } catch (error) {
      console.error('Failed to fetch incentive recipient count:', error);
      this.chainRecipientCount.set(null);
      this.chainIncentiveMode.set(null);
      this.chainRankingBps.set(null);
    }
  };

  handleClaimIncentive = async (incentiveSk: string) => {
    const dao = this.dao;
    if (!dao?.contract_address || !this.currentUserEvm) return;
    if (!this.canClaimIncentive) {
      showErrorToast(this.t('error_incentive_claim_not_allowed'));
      return;
    }

    const tokenAddress = this.activeTokenAddress;
    if (!tokenAddress) {
      showErrorToast(this.t('error_register_failed_unknown'));
      return;
    }

    this.isClaiming.set(true);
    try {
      showInfoToast(this.t('toast_connecting_wallet'));
      const signer = await getKaiaSigner(
        config.env === 'prod' ? 'mainnet' : 'testnet',
      );
      const provider = signer.provider;
      const daoService = new SpaceDaoService(provider);
      await daoService.connectWallet();
      await daoService.claimIncentive(dao.contract_address, tokenAddress);

      await this.updateIncentiveMutation.mutateAsync({
        spacePk: this.spacePk,
        incentiveSk,
        incentiveDistributed: true,
      });

      this.isIncentiveClaimed.set(true);
      showSuccessToast(this.t('toast_incentive_claimed'));
    } catch (error) {
      console.error('Failed to claim incentive:', error);
      if (error instanceof Error) {
        showErrorToast(
          this.t('error_incentive_claim_failed', { message: error.message }),
        );
      } else {
        showErrorToast(this.t('error_register_failed_unknown'));
      }
    } finally {
      this.isClaiming.set(false);
    }
  };

  handleOpenRegistrationPopup = () => {
    if (!this.canRegisterDao) {
      showErrorToast(
        this.isTeamSpace
          ? this.t('error_insufficient_admins')
          : this.t('insufficient_admins_personal'),
      );
      return;
    }
    if (!this.canSubmitInputs) {
      showErrorToast(this.t('error_missing_inputs'));
      return;
    }
    const admins = this.getDefaultAdminAddresses();
    void this.handleRegisterDao(admins);
  };

  handleClosePopup = () => {
    if (!this.isRegistering.get()) {
      this.isPopupOpen.set(false);
    }
  };

  handleRegisterDao = async (selectedAdminAddresses: string[]) => {
    if (selectedAdminAddresses.length === 0) {
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

      const count = Number(this.incentiveCount.get());
      if (!Number.isFinite(count) || count <= 0 || count > 100) {
        showErrorToast(this.t('error_invalid_incentive_count'));
        return;
      }
      const mode = this.incentiveMode.get();
      const rankingRatio = Number(this.rankingBps.get());
      const rankingBps =
        mode === 2 && Number.isFinite(rankingRatio)
          ? Math.round(rankingRatio * 100)
          : 0;
      const result = await daoService.createSpaceIncentive(
        selectedAdminAddresses,
        count,
        rankingBps,
        mode,
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

  getDefaultAdminAddresses() {
    if (!this.isTeamSpace) {
      return this.currentUserEvm ? [this.currentUserEvm] : [];
    }
    const unique = new Set<string>();
    for (const member of this.teamMembers) {
      if (!member.evm_address) continue;
      unique.add(member.evm_address);
    }
    return Array.from(unique);
  }

  handleUpdateDao = async (incentiveCount: string, rankingRatio?: string) => {
    const dao = this.dao;
    const count = Number(incentiveCount);
    const ratioValue =
      rankingRatio != null && rankingRatio !== '' ? Number(rankingRatio) : null;

    if (!dao?.contract_address) {
      showErrorToast(this.t('error_register_failed_unknown'));
      return;
    }
    if (!Number.isFinite(count) || count <= 0 || count > 100) {
      showErrorToast(this.t('error_invalid_incentive_count'));
      return;
    }
    if (
      ratioValue != null &&
      (!Number.isFinite(ratioValue) || ratioValue < 0 || ratioValue > 100)
    ) {
      showErrorToast(this.t('error_invalid_ranking_ratio'));
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
      await daoService.setIncentiveRecipientCount(dao.contract_address, count);
      if (ratioValue != null) {
        await daoService.setIncentiveRankingBps(
          dao.contract_address,
          Math.round(ratioValue * 100),
        );
        this.chainRankingBps.set(Math.round(ratioValue * 100));
      }
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
  const incentiveCount = useState('');
  const incentiveMode = useState(0);
  const rankingBps = useState('30');
  const chainRecipientCount = useState<string | null>(null);
  const chainIncentiveMode = useState<number | null>(null);
  const chainRankingBps = useState<number | null>(null);
  const isPopupOpen = useState(false);
  const isRegistering = useState(false);
  const isUpdating = useState(false);
  const isIncentiveRecipient = useState(false);
  const isIncentiveClaimed = useState(false);
  const isClaiming = useState(false);
  const claimAmountRaw = useState<string | null>(null);
  const { data: user } = useUserInfo();
  const createSpaceDaoMutation = useCreateSpaceDaoMutation();
  const updateIncentiveMutation = useUpdateSpaceDaoIncentiveMutation();
  const { data: incentiveData, isLoading: incentiveLoading } =
    useSpaceDaoIncentive(spacePk, Boolean(dao?.contract_address));

  const provider = useMemo(() => {
    if (!config.rpc_url) {
      return null;
    }
    return new ethers.JsonRpcProvider(config.rpc_url);
  }, []);

  const teamUsername = space?.authorUsername ?? '';
  const isTeamSpace = (space?.authorType ?? null) === UserType.Team;
  const { eligibleAdmins, permissions, members } = useDaoData(
    teamUsername,
    Boolean(teamUsername) && isTeamSpace,
  );

  const ctrl = new SpaceDaoEditorController(
    spacePk,
    space,
    dao,
    eligibleAdmins,
    members,
    permissions,
    t,
    provider,
    new State(adminAddresses),
    new State(incentiveCount),
    new State(incentiveMode),
    new State(rankingBps),
    new State(chainRecipientCount),
    new State(chainIncentiveMode),
    new State(chainRankingBps),
    new State(isPopupOpen),
    new State(isRegistering),
    new State(isUpdating),
    incentiveData,
    incentiveLoading,
    user?.evm_address ?? null,
    new State(isIncentiveRecipient),
    new State(isIncentiveClaimed),
    new State(isClaiming),
    new State(claimAmountRaw),
    selectedToken ?? null,
    tokenBalance ?? null,
    tokenDecimals ?? null,
    preferredToken ?? null,
    preferredTokenBalance ?? null,
    preferredTokenDecimals ?? null,
    createSpaceDaoMutation,
    updateIncentiveMutation,
  );

  useEffect(() => {
    void ctrl.fetchRecipientCount();
  }, [dao?.contract_address, provider]);

  useEffect(() => {
    const loadClaimStatus = async () => {
      if (!provider || !dao?.contract_address || !user?.evm_address) {
        isIncentiveRecipient[1](false);
        isIncentiveClaimed[1](false);
        return;
      }
      try {
        const service = new SpaceDaoService(provider);
        const [recipient, incentiveClaimed] = await Promise.all([
          service.isIncentiveRecipient(dao.contract_address, user.evm_address),
          service.isIncentiveClaimed(dao.contract_address, user.evm_address),
        ]);
        isIncentiveRecipient[1](recipient);
        isIncentiveClaimed[1](incentiveClaimed);
      } catch (error) {
        console.error('Failed to fetch incentive claim status:', error);
        isIncentiveRecipient[1](false);
        isIncentiveClaimed[1](false);
      }
    };
    void loadClaimStatus();
  }, [dao?.contract_address, provider, user?.evm_address]);

  useEffect(() => {
    const loadClaimAmount = async () => {
      const token = preferredToken ?? selectedToken ?? null;
      if (!provider || !dao?.contract_address || !token) {
        claimAmountRaw[1](null);
        return;
      }
      try {
        const service = new SpaceDaoService(provider);
        const amount = await service.getIncentiveAmount(
          dao.contract_address,
          token,
        );
        claimAmountRaw[1](amount.toString());
      } catch (error) {
        console.error('Failed to fetch claim amount:', error);
        claimAmountRaw[1](null);
      }
    };
    void loadClaimAmount();
  }, [dao?.contract_address, provider, preferredToken, selectedToken]);

  return ctrl;
}
