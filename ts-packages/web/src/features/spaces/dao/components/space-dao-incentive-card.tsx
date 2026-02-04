import { useTranslation } from 'react-i18next';
import { CheckIcon } from '@heroicons/react/24/outline';
import { Button } from '@/components/ui/button';
import { SpaceDaoIncentiveResponse } from '@/features/spaces/dao/hooks/use-space-dao-incentive';

type SpaceDaoIncentiveCardProps = {
  incentiveRecipient?: SpaceDaoIncentiveResponse | null;
  remainingCount?: number | null;
  totalCount?: number | null;
  incentiveLoading?: boolean;
  currentUserEvm?: string | null;
  claimableAmount?: string | null;
  isClaimable?: boolean;
  isClaiming?: boolean;
  onClaimIncentive?: (incentiveSk: string) => void;
};

export function SpaceDaoIncentiveCard({
  incentiveRecipient,
  remainingCount,
  totalCount,
  incentiveLoading = false,
  currentUserEvm,
  claimableAmount,
  isClaimable = false,
  isClaiming = false,
  onClaimIncentive,
}: SpaceDaoIncentiveCardProps) {
  const { t } = useTranslation('SpaceDaoEditor');

  const renderAddress = (address: string) => {
    if (address.length <= 10) return address;
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  };

  const walletAddress =
    incentiveRecipient?.evm_address ?? currentUserEvm ?? null;
  const canClaim =
    Boolean(incentiveRecipient) &&
    Boolean(currentUserEvm) &&
    walletAddress?.toLowerCase() === currentUserEvm?.toLowerCase();

  return (
    <div className="mt-6 space-y-4 w-full">
      <div className="flex items-center justify-between">
        <p className="text-sm text-text-secondary">
          {t('dao_incentive_title')}
        </p>
      </div>

      {incentiveLoading ? (
        <div className="text-sm text-text-secondary">
          {t('dao_selected_loading')}
        </div>
      ) : incentiveRecipient ? (
        <div className="rounded-xl border border-input-box-border bg-background px-4 py-4">
          <div className="grid grid-cols-1 gap-3 text-sm">
            <div className="flex items-center justify-between">
              <span className="text-text-secondary">
                {t('dao_incentive_total')}
              </span>
              <span className="text-text-primary">{totalCount ?? '-'}</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-text-secondary">
                {t('dao_incentive_remaining')}
              </span>
              <span className="text-text-primary">{remainingCount ?? '-'}</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-text-secondary">
                {t('dao_incentive_wallet')}
              </span>
              <span className="text-text-primary">
                {walletAddress ? renderAddress(walletAddress) : '-'}
              </span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-text-secondary">
                {t('dao_incentive_amount')}
              </span>
              <span className="text-text-primary">
                {claimableAmount ?? '-'}
              </span>
            </div>
            <div className="flex items-center justify-end pt-2">
              {incentiveRecipient.incentive_distributed ? (
                <span className="inline-flex items-center gap-1 text-green-600 text-xs">
                  <CheckIcon className="w-4 h-4" />
                  {t('dao_selected_claimed')}
                </span>
              ) : canClaim ? (
                <Button
                  type="button"
                  variant="rounded_primary"
                  size="sm"
                  onClick={() => onClaimIncentive?.(incentiveRecipient.sk)}
                  disabled={!isClaimable || !onClaimIncentive || isClaiming}
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
