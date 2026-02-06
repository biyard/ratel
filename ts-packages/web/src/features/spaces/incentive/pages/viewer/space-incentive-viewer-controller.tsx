import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { SpaceIncentiveResponse } from '@/features/spaces/incentive/hooks/use-space-incentive';
import { State } from '@/types/state';
import {
  SpaceIncentiveIncentiveResponseBody,
  useSpaceIncentiveIncentive,
} from '@/features/spaces/incentive/hooks/use-space-incentive-incentive';
import { SpaceIncentiveService } from '@/contracts/SpaceIncentiveService';
import { config } from '@/config';
import { ethers } from 'ethers';
import { useUserInfo } from '@/hooks/use-user-info';
import { showErrorToast, showInfoToast, showSuccessToast } from '@/lib/toast';
import {
  getKaiaSigner,
  KaiaWalletError,
} from '@/lib/service/kaia-wallet-service';
import { useUpdateSpaceIncentiveMutation } from '@/features/spaces/incentive/hooks/use-update-space-incentive-mutation';

export class SpaceIncentiveViewerController {
  constructor(
    public spacePk: string,
    public space: Space | undefined,
    public incentive: SpaceIncentiveResponse | null | undefined,
    public t: TFunction<'SpaceIncentiveEditor', undefined>,
    public provider: ethers.JsonRpcProvider | null,
    public chainRecipientCount: State<string | null>,
    public incentiveData: SpaceIncentiveIncentiveResponseBody | undefined,
    public incentiveLoading: boolean,
    public currentUserEvm: string | null,
    public isIncentiveRecipient: State<boolean>,
    public isIncentiveClaimed: State<boolean>,
    public isClaiming: State<boolean>,
    public claimAmountRaw: State<string | null>,
    public selectedToken: string | null,
    public tokenBalance: string | null,
    public tokenDecimals: number | null,
    public updateIncentiveMutation: ReturnType<
      typeof useUpdateSpaceIncentiveMutation
    >,
  ) {}

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
    const decimals = this.tokenDecimals ?? 0;
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
    if (!this.provider || !this.incentive?.contract_address) {
      return;
    }
    try {
      const service = new SpaceIncentiveService(this.provider);
      const count = await service.getIncentiveRecipientCount(
        this.incentive.contract_address,
      );
      this.chainRecipientCount.set(String(count));
    } catch (error) {
      console.error('Failed to fetch incentive recipient count:', error);
      this.chainRecipientCount.set(null);
    }
  };

  handleClaimIncentive = async (incentiveSk: string) => {
    const incentive = this.incentive;
    if (!incentive?.contract_address || !this.currentUserEvm) return;
    if (!this.canClaimIncentive) {
      showErrorToast(this.t('error_incentive_claim_not_allowed'));
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
      const incentiveService = new SpaceIncentiveService(provider);
      await incentiveService.connectWallet();
      await incentiveService.claimIncentive(
        incentive.contract_address,
        this.selectedToken,
      );

      await this.updateIncentiveMutation.mutateAsync({
        spacePk: this.spacePk,
        incentiveSk,
        incentiveDistributed: true,
      });

      this.isIncentiveClaimed.set(true);
      showSuccessToast(this.t('toast_incentive_claimed'));
    } catch (error) {
      console.error('Failed to claim incentive:', error);
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
          this.t('error_incentive_claim_failed', { message: error.message }),
        );
      } else {
        showErrorToast(this.t('error_register_failed_unknown'));
      }
    } finally {
      this.isClaiming.set(false);
    }
  };
}

export function useSpaceIncentiveViewerController(
  spacePk: string,
  incentive?: SpaceIncentiveResponse | null,
  selectedToken?: string | null,
  tokenBalance?: string | null,
  tokenDecimals?: number | null,
) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceIncentiveEditor');
  const chainRecipientCount = useState<string | null>(null);
  const isIncentiveRecipient = useState(false);
  const isIncentiveClaimed = useState(false);
  const isClaiming = useState(false);
  const claimAmountRaw = useState<string | null>(null);
  const { data: user } = useUserInfo();
  const updateIncentiveMutation = useUpdateSpaceIncentiveMutation();
  const { data: incentiveData, isLoading: incentiveLoading } =
    useSpaceIncentiveIncentive(spacePk, Boolean(incentive?.contract_address));
  const provider = useMemo(() => {
    if (!config.rpc_url) {
      return null;
    }
    return new ethers.JsonRpcProvider(config.rpc_url);
  }, []);

  const ctrl = new SpaceIncentiveViewerController(
    spacePk,
    space,
    incentive,
    t,
    provider,
    new State(chainRecipientCount),
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
    updateIncentiveMutation,
  );

  useEffect(() => {
    void ctrl.fetchRecipientCount();
  }, [incentive?.contract_address, provider]);

  useEffect(() => {
    const loadClaimStatus = async () => {
      if (!provider || !incentive?.contract_address || !user?.evm_address) {
        isIncentiveRecipient[1](false);
        isIncentiveClaimed[1](false);
        return;
      }
      try {
        const service = new SpaceIncentiveService(provider);
        const [recipient, incentiveClaimed] = await Promise.all([
          service.isIncentiveRecipient(
            incentive.contract_address,
            user.evm_address,
          ),
          service.isIncentiveClaimed(
            incentive.contract_address,
            user.evm_address,
          ),
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
  }, [incentive?.contract_address, provider, user?.evm_address]);

  useEffect(() => {
    const loadClaimAmount = async () => {
      if (!provider || !incentive?.contract_address || !selectedToken) {
        claimAmountRaw[1](null);
        return;
      }
      try {
        const service = new SpaceIncentiveService(provider);
        const amount = await service.getIncentiveAmount(
          incentive.contract_address,
          selectedToken,
        );
        claimAmountRaw[1](amount.toString());
      } catch (error) {
        console.error('Failed to fetch claim amount:', error);
        claimAmountRaw[1](null);
      }
    };
    void loadClaimAmount();
  }, [incentive?.contract_address, provider, selectedToken]);

  return ctrl;
}
