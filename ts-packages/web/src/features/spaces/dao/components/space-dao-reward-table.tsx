import { useTranslation } from 'react-i18next';
import { CheckIcon } from '@heroicons/react/24/outline';
import { Button } from '@/components/ui/button';
import { SpaceDaoRewardResponse } from '@/features/spaces/dao/hooks/use-space-dao-reward';

type SpaceDaoRewardTableProps = {
  rewardRecipients?: SpaceDaoRewardResponse[];
  rewardBookmark?: string | null;
  canPrevReward?: boolean;
  canNextReward?: boolean;
  rewardLoading?: boolean;
  canDistributeReward?: boolean;
  onNextReward?: () => void;
  onPrevReward?: () => void;
  onDistributePage?: () => void;
  isDistributingPage?: boolean;
};

export function SpaceDaoRewardTable({
  rewardRecipients,
  rewardBookmark,
  canPrevReward = false,
  canNextReward = false,
  rewardLoading = false,
  canDistributeReward = false,
  onNextReward,
  onPrevReward,
  onDistributePage,
  isDistributingPage = false,
}: SpaceDaoRewardTableProps) {
  const { t } = useTranslation('SpaceDaoEditor');

  const renderAddress = (address: string) => {
    if (address.length <= 10) return address;
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  };

  return (
    <div className="mt-6 space-y-3 w-full">
      <div className="flex items-center justify-between">
        <p className="text-sm text-text-secondary">{t('dao_selected_title')}</p>
        <div className="flex items-center gap-2">
          <Button
            type="button"
            variant="outline"
            size="sm"
            onClick={onPrevReward}
            disabled={!canPrevReward}
          >
            {t('dao_selected_prev')}
          </Button>
          <Button
            type="button"
            variant="outline"
            size="sm"
            onClick={onNextReward}
            disabled={!canNextReward || !rewardBookmark}
          >
            {t('dao_selected_next')}
          </Button>
        </div>
      </div>

      {rewardLoading ? (
        <div className="text-sm text-text-secondary">
          {t('dao_selected_loading')}
        </div>
      ) : rewardRecipients && rewardRecipients.length > 0 ? (
        <table className="overflow-hidden w-full text-sm rounded-xl border border-input-box-border">
          <thead className="bg-muted text-[var(--color-panel-table-header)]">
            <tr>
              <th className="py-3 px-4 text-left">{t('dao_selected_user')}</th>
              <th className="py-3 px-4 text-left">{t('dao_selected_evm')}</th>
              <th className="py-3 px-4 text-left">
                {t('dao_selected_status')}
              </th>
            </tr>
          </thead>
          <tbody>
            {rewardRecipients.map((item) => (
              <tr
                key={item.sk}
                className="border-t border-input-box-border hover:bg-muted/50"
              >
                <td className="py-3 px-4">
                  <span className="font-medium">{item.display_name}</span>
                </td>
                <td className="py-3 px-4 text-xs break-all">
                  {renderAddress(item.evm_address)}
                </td>
                <td className="py-3 px-4 text-xs">
                  {item.reward_distributed ? (
                    <span className="inline-flex items-center gap-1 text-green-600">
                      <CheckIcon className="w-4 h-4" />
                      {t('dao_selected_distributed')}
                    </span>
                  ) : (
                    <span className="text-text-secondary">
                      {t('dao_selected_pending')}
                    </span>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      ) : (
        <div className="text-sm text-text-secondary">
          {t('dao_selected_empty')}
        </div>
      )}

      {canDistributeReward &&
        rewardRecipients &&
        rewardRecipients.some((item) => !item.reward_distributed) && (
          <div className="flex justify-end">
            <Button
              type="button"
              variant="rounded_primary"
              size="sm"
              onClick={onDistributePage}
              disabled={!onDistributePage || isDistributingPage}
            >
              {isDistributingPage
                ? t('dao_selected_distributing')
                : t('dao_selected_distribute')}
            </Button>
          </div>
        )}
    </div>
  );
}
