import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { SpaceDaoResponse } from '@/features/spaces/dao/hooks/use-space-dao';
import { State } from '@/types/state';
import {
  SpaceDaoRewardResponseBody,
  useSpaceDaoReward,
} from '@/features/spaces/dao/hooks/use-space-dao-reward';
import { SpaceDaoService } from '@/contracts/SpaceDaoService';
import { config } from '@/config';
import { ethers } from 'ethers';
import { useUserInfo } from '@/hooks/use-user-info';
import { showErrorToast, showInfoToast, showSuccessToast } from '@/lib/toast';
import {
  getKaiaSigner,
  KaiaWalletError,
} from '@/lib/service/kaia-wallet-service';
import { useUpdateSpaceDaoRewardMutation } from '@/features/spaces/dao/hooks/use-update-space-dao-reward-mutation';

export class SpaceDaoViewerController {
  constructor(
    public spacePk: string,
    public space: Space | undefined,
    public dao: SpaceDaoResponse | null | undefined,
    public t: TFunction<'SpaceDaoEditor', undefined>,
    public provider: ethers.JsonRpcProvider | null,
    public chainRecipientCount: State<string | null>,
    public rewardData: SpaceDaoRewardResponseBody | undefined,
    public rewardLoading: boolean,
    public currentUserEvm: string | null,
    public isIncentiveRecipient: State<boolean>,
    public isIncentiveClaimed: State<boolean>,
    public isClaiming: State<boolean>,
    public claimAmountRaw: State<string | null>,
    public selectedToken: string | null,
    public tokenBalance: string | null,
    public tokenDecimals: number | null,
    public updateRewardMutation: ReturnType<
      typeof useUpdateSpaceDaoRewardMutation
    >,
  ) {}

  get visibleRewardRecipients() {
    return this.rewardRecipients?.items ?? [];
  }

  get rewardRecipients() {
    const item = this.rewardData?.item;
    return item ? { items: [item] } : undefined;
  }

  get rewardRecipientsLoading() {
    return this.rewardLoading;
  }

  get rewardMeta() {
    return this.rewardData;
  }

  get remainingRewardCount() {
    const meta = this.rewardMeta;
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
    const decimals = this.tokenDecimals ?? 0;
    if (perRecipient == null) return null;
    try {
      return ethers.formatUnits(perRecipient, decimals);
    } catch {
      return perRecipient.toString();
    }
  }

  get canClaimReward() {
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
      const count = await service.getIncentiveRecipientCount(
        this.dao.contract_address,
      );
      this.chainRecipientCount.set(String(count));
    } catch (error) {
      console.error('Failed to fetch reward recipient count:', error);
      this.chainRecipientCount.set(null);
    }
  };

  handleClaimReward = async (rewardSk: string) => {
    const dao = this.dao;
    if (!dao?.contract_address || !this.currentUserEvm) return;
    if (!this.canClaimReward) {
      showErrorToast(this.t('error_reward_claim_not_allowed'));
      return;
    }
    if (!this.selectedToken) {
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
      await daoService.claimIncentive(dao.contract_address, this.selectedToken);

      await this.updateRewardMutation.mutateAsync({
        spacePk: this.spacePk,
        rewardSk,
        rewardDistributed: true,
      });

      this.isIncentiveClaimed.set(true);
      showSuccessToast(this.t('toast_reward_claimed'));
    } catch (error) {
      console.error('Failed to claim reward:', error);
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
          this.t('error_reward_claim_failed', { message: error.message }),
        );
      } else {
        showErrorToast(this.t('error_register_failed_unknown'));
      }
    } finally {
      this.isClaiming.set(false);
    }
  };
}

export function useSpaceDaoViewerController(
  spacePk: string,
  dao?: SpaceDaoResponse | null,
  selectedToken?: string | null,
  tokenBalance?: string | null,
  tokenDecimals?: number | null,
) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceDaoEditor');
  const chainRecipientCount = useState<string | null>(null);
  const isIncentiveRecipient = useState(false);
  const isIncentiveClaimed = useState(false);
  const isClaiming = useState(false);
  const claimAmountRaw = useState<string | null>(null);
  const { data: user } = useUserInfo();
  const updateRewardMutation = useUpdateSpaceDaoRewardMutation();
  const { data: reward, isLoading: rewardLoading } = useSpaceDaoReward(
    spacePk,
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
    new State(chainRecipientCount),
    reward,
    rewardLoading,
    user?.evm_address ?? null,
    new State(isIncentiveRecipient),
    new State(isIncentiveClaimed),
    new State(isClaiming),
    new State(claimAmountRaw),
    selectedToken ?? null,
    tokenBalance ?? null,
    tokenDecimals ?? null,
    updateRewardMutation,
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
        const [recipient, rewarded] = await Promise.all([
          service.isIncentiveRecipient(dao.contract_address, user.evm_address),
          service.isIncentiveClaimed(dao.contract_address, user.evm_address),
        ]);
        isIncentiveRecipient[1](recipient);
        isIncentiveClaimed[1](rewarded);
      } catch (error) {
        console.error('Failed to fetch reward claim status:', error);
        isIncentiveRecipient[1](false);
        isIncentiveClaimed[1](false);
      }
    };
    void loadClaimStatus();
  }, [dao?.contract_address, provider, user?.evm_address]);

  useEffect(() => {
    const loadClaimAmount = async () => {
      if (!provider || !dao?.contract_address || !selectedToken) {
        claimAmountRaw[1](null);
        return;
      }
      try {
        const service = new SpaceDaoService(provider);
        const amount = await service.getIncentiveAmount(
          dao.contract_address,
          selectedToken,
        );
        claimAmountRaw[1](amount.toString());
      } catch (error) {
        console.error('Failed to fetch claim amount:', error);
        claimAmountRaw[1](null);
      }
    };
    void loadClaimAmount();
  }, [dao?.contract_address, provider, selectedToken]);

  return ctrl;
}
