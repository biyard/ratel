import { useTranslation } from 'react-i18next';
import { CheckIcon } from '@heroicons/react/24/outline';
import { Button } from '@/components/ui/button';
import { SpaceDaoSampleResponse } from '@/features/spaces/dao/hooks/use-space-dao-samples';

type SpaceDaoSampleTableProps = {
  samples?: SpaceDaoSampleResponse[];
  samplesBookmark?: string | null;
  canPrevSample?: boolean;
  canNextSample?: boolean;
  samplesLoading?: boolean;
  canDistributeReward?: boolean;
  onNextSample?: () => void;
  onPrevSample?: () => void;
  onDistributePage?: () => void;
  isDistributingPage?: boolean;
};

export function SpaceDaoSampleTable({
  samples,
  samplesBookmark,
  canPrevSample = false,
  canNextSample = false,
  samplesLoading = false,
  canDistributeReward = false,
  onNextSample,
  onPrevSample,
  onDistributePage,
  isDistributingPage = false,
}: SpaceDaoSampleTableProps) {
  const { t } = useTranslation('SpaceDaoEditor');

  const renderAddress = (address: string) => {
    if (address.length <= 10) return address;
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  };

  return (
    <div className="mt-6 space-y-3 w-full">
      <div className="flex items-center justify-between">
        <p className="text-sm text-text-secondary">{t('dao_samples_title')}</p>
        <div className="flex items-center gap-2">
          <Button
            type="button"
            variant="outline"
            size="sm"
            onClick={onPrevSample}
            disabled={!canPrevSample}
          >
            {t('dao_samples_prev')}
          </Button>
          <Button
            type="button"
            variant="outline"
            size="sm"
            onClick={onNextSample}
            disabled={!canNextSample || !samplesBookmark}
          >
            {t('dao_samples_next')}
          </Button>
        </div>
      </div>

      {samplesLoading ? (
        <div className="text-sm text-text-secondary">
          {t('dao_samples_loading')}
        </div>
      ) : samples && samples.length > 0 ? (
        <table className="overflow-hidden w-full text-sm rounded-xl border border-input-box-border">
          <thead className="bg-muted text-[var(--color-panel-table-header)]">
            <tr>
              <th className="py-3 px-4 text-left">{t('dao_samples_user')}</th>
              <th className="py-3 px-4 text-left">{t('dao_samples_evm')}</th>
              <th className="py-3 px-4 text-left">{t('dao_samples_status')}</th>
            </tr>
          </thead>
          <tbody>
            {samples.map((sample) => (
              <tr
                key={sample.sk}
                className="border-t border-input-box-border hover:bg-muted/50"
              >
                <td className="py-3 px-4">
                  <span className="font-medium">{sample.display_name}</span>
                </td>
                <td className="py-3 px-4 text-xs break-all">
                  {renderAddress(sample.evm_address)}
                </td>
                <td className="py-3 px-4 text-xs">
                  {sample.reward_distributed ? (
                    <span className="inline-flex items-center gap-1 text-green-600">
                      <CheckIcon className="w-4 h-4" />
                      {t('dao_samples_distributed')}
                    </span>
                  ) : (
                    <span className="text-text-secondary">
                      {t('dao_samples_pending')}
                    </span>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      ) : (
        <div className="text-sm text-text-secondary">
          {t('dao_samples_empty')}
        </div>
      )}

      {canDistributeReward &&
        samples &&
        samples.some((item) => !item.reward_distributed) && (
          <div className="flex justify-end">
            <Button
              type="button"
              variant="rounded_primary"
              size="sm"
              onClick={onDistributePage}
              disabled={!onDistributePage || isDistributingPage}
            >
              {isDistributingPage
                ? t('dao_samples_distributing')
                : t('dao_samples_distribute')}
            </Button>
          </div>
        )}
    </div>
  );
}
