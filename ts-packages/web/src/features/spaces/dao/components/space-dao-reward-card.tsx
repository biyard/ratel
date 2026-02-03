import { useTranslation } from 'react-i18next';
import { CheckIcon } from '@heroicons/react/24/outline';
import { Button } from '@/components/ui/button';
import { SpaceDaoRewardResponse } from '@/features/spaces/dao/hooks/use-space-dao-reward';

type SpaceDaoRewardCardProps = {
  rewardRecipient?: SpaceDaoRewardResponse | null;
  remainingCount?: number | null;
  totalCount?: number | null;
  rewardLoading?: boolean;
  currentUserEvm?: string | null;
  claimableAmount?: string | null;
  isClaimable?: boolean;
  isClaiming?: boolean;
  onClaimReward?: (rewardSk: string) => void;
};

export function SpaceDaoRewardCard({
  rewardRecipient,
  remainingCount,
  totalCount,
  rewardLoading = false,
  currentUserEvm,
  claimableAmount,
  isClaimable = false,
  isClaiming = false,
  onClaimReward,
}: SpaceDaoRewardCardProps) {
  const { t } = useTranslation('SpaceDaoEditor');

  const renderAddress = (address: string) => {
    if (address.length <= 10) return address;
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  };

  const walletAddress = rewardRecipient?.evm_address ?? currentUserEvm ?? null;
  const canClaim =
    Boolean(rewardRecipient) &&
    Boolean(currentUserEvm) &&
    walletAddress?.toLowerCase() === currentUserEvm?.toLowerCase();

  return (
    <div className="mt-6 space-y-4 w-full">
      <div className="flex items-center justify-between">
        <p className="text-sm text-text-secondary">{t('dao_reward_title')}</p>
      </div>

      {rewardLoading ? (
        <div className="text-sm text-text-secondary">
          {t('dao_selected_loading')}
        </div>
      ) : rewardRecipient ? (
        <div className="rounded-xl border border-input-box-border bg-background px-4 py-4">
          <div className="grid grid-cols-1 gap-3 text-sm">
            <div className="flex items-center justify-between">
              <span className="text-text-secondary">
                {t('dao_reward_total')}
              </span>
              <span className="text-text-primary">{totalCount ?? '-'}</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-text-secondary">
                {t('dao_reward_remaining')}
              </span>
              <span className="text-text-primary">{remainingCount ?? '-'}</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-text-secondary">
                {t('dao_reward_wallet')}
              </span>
              <span className="text-text-primary">
                {walletAddress ? renderAddress(walletAddress) : '-'}
              </span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-text-secondary">
                {t('dao_reward_amount')}
              </span>
              <span className="text-text-primary">
                {claimableAmount ?? '-'}
              </span>
            </div>
            <div className="flex items-center justify-end pt-2">
              {rewardRecipient.reward_distributed ? (
                <span className="inline-flex items-center gap-1 text-green-600 text-xs">
                  <CheckIcon className="w-4 h-4" />
                  {t('dao_selected_claimed')}
                </span>
              ) : canClaim ? (
                <Button
                  type="button"
                  variant="rounded_primary"
                  size="sm"
                  onClick={() => onClaimReward?.(rewardRecipient.sk)}
                  disabled={!isClaimable || !onClaimReward || isClaiming}
                >
                  {isClaiming
                    ? t('dao_selected_claiming')
                    : t('dao_selected_claim')}
                </Button>
              ) : (
                <span className="text-text-secondary text-xs">
                  {t('dao_selected_pending')}
                </span>
              )}
            </div>
          </div>
        </div>
      ) : (
        <div className="text-sm text-text-secondary">
          {t('dao_selected_empty')}
        </div>
      )}
    </div>
  );
}
