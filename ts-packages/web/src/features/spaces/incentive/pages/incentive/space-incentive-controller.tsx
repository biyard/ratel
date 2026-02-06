import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { Space } from '@/features/spaces/types/space';
import { SpaceIncentiveResponse } from '@/features/spaces/incentive/hooks/use-space-incentive';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useUserInfo } from '@/hooks/use-user-info';
import { State } from '@/types/state';
import { SpaceIncentiveService } from '@/contracts/SpaceIncentiveService';
import { config } from '@/config';
import { ethers } from 'ethers';
import { showErrorToast, showInfoToast, showSuccessToast } from '@/lib/toast';
import {
  getKaiaSigner,
  KaiaWalletError,
} from '@/lib/service/kaia-wallet-service';
type IncentiveRecipientRow = {
  address: string;
  claimed: boolean;
};

export class SpaceIncentiveController {
  constructor(
    public spacePk: string,
    public space: Space | undefined,
    public incentive: SpaceIncentiveResponse | null | undefined,
    public t: TFunction<'SpaceIncentiveEditor', undefined>,
    public provider: ethers.JsonRpcProvider | null,
    public recipients: IncentiveRecipientRow[],
    public recipientsLoading: boolean,
    public markClaimed: (address: string) => void,
    public currentUserEvm: string | null,
    public isClaiming: State<boolean>,
    public claimAmountRaw: State<string | null>,
    public selectedToken: string | null,
    public tokenDecimals: number | null,
  ) {}

  get currentUserItem() {
    if (!this.currentUserEvm) return null;
    return (
      this.recipients.find(
        (item) =>
          item.address.toLowerCase() === this.currentUserEvm?.toLowerCase(),
      ) ?? null
    );
  }

  get totalCount() {
    return this.recipients.length;
  }

  get remainingCount() {
    return this.recipients.filter((item) => !item.claimed).length;
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

  get canClaim() {
    if (!this.currentUserEvm) return false;
    const item = this.currentUserItem;
    if (!item || item.claimed) return false;
    return this.perRecipientAmount != null && this.perRecipientAmount > 0n;
  }

  handleClaim = async () => {
    const incentive = this.incentive;
    const item = this.currentUserItem;
    if (!incentive?.contract_address || !this.currentUserEvm || !item) return;
    if (!this.canClaim) {
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

      this.markClaimed(item.address);
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

export function useSpaceIncentiveController(
  spacePk: string,
  incentive?: SpaceIncentiveResponse | null,
  selectedToken?: string | null,
  tokenDecimals?: number | null,
) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceIncentiveEditor');
  const isClaiming = useState(false);
  const claimAmountRaw = useState<string | null>(null);
  const { data: user } = useUserInfo();
  const [recipients, setRecipients] = useState<IncentiveRecipientRow[]>([]);
  const [recipientsLoading, setRecipientsLoading] = useState(false);

  const provider = useMemo(() => {
    if (!config.rpc_url) {
      return null;
    }
    return new ethers.JsonRpcProvider(config.rpc_url);
  }, []);

  const ctrl = new SpaceIncentiveController(
    spacePk,
    space,
    incentive,
    t,
    provider,
    recipients,
    recipientsLoading,
    (address) => {
      setRecipients((prev) =>
        prev.map((item) =>
          item.address.toLowerCase() === address.toLowerCase()
            ? { ...item, claimed: true }
            : item,
        ),
      );
    },
    user?.evm_address ?? null,
    new State(isClaiming),
    new State(claimAmountRaw),
    selectedToken ?? null,
    tokenDecimals ?? null,
  );

  useEffect(() => {
    const loadRecipients = async () => {
      if (!provider || !incentive?.contract_address) {
        setRecipients([]);
        return;
      }
      setRecipientsLoading(true);
      try {
        const service = new SpaceIncentiveService(provider);
        const addresses = await service.getIncentiveRecipients(
          incentive.contract_address,
        );
        const rows = await Promise.all(
          addresses.map(async (address) => ({
            address,
            claimed: await service.isIncentiveClaimed(
              incentive.contract_address,
              address,
            ),
          })),
        );
        setRecipients(rows);
      } catch (error) {
        console.error('Failed to fetch incentive recipients:', error);
        setRecipients([]);
      } finally {
        setRecipientsLoading(false);
      }
    };
    void loadRecipients();
  }, [incentive?.contract_address, provider]);

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
        console.error('Failed to fetch incentive amount:', error);
        claimAmountRaw[1](null);
      }
    };
    void loadClaimAmount();
  }, [incentive?.contract_address, provider, selectedToken]);

  return ctrl;
}
