import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import Card from '@/components/card';
import { SpaceDaoResponse } from '@/features/spaces/dao/hooks/use-space-dao';
import { SpaceDaoSampleResponse } from '@/features/spaces/dao/hooks/use-space-dao-samples';
import { SpaceDaoSampleTable } from './space-dao-sample-table';
import { config } from '@/config';
import { CheckIcon, ClipboardIcon } from '@heroicons/react/24/outline';
import { Button } from '@/components/ui/button';
import { SpaceDaoDepositDialog } from './space-dao-deposit-dialog';
import { Input } from '@/components/ui/input';

type SpaceDaoInfoCardProps = {
  dao: SpaceDaoResponse;
  balance?: string | null;
  balanceLoading?: boolean;
  isDepositOpen?: boolean;
  depositAmount?: string;
  isDepositing?: boolean;
  onOpenDeposit?: () => void;
  onCloseDeposit?: () => void;
  onDepositAmountChange?: (value: string) => void;
  onConfirmDeposit?: () => void;
  isUpdating?: boolean;
  onUpdateDao?: (samplingCount: string, rewardAmount: string) => Promise<void>;
  samples?: SpaceDaoSampleResponse[];
  samplesBookmark?: string | null;
  canPrevSample?: boolean;
  canNextSample?: boolean;
  samplesLoading?: boolean;
  showSamples?: boolean;
  showEdit?: boolean;
  showDeposit?: boolean;
  canDistributeReward?: boolean;
  onNextSample?: () => void;
  onPrevSample?: () => void;
  onDistributePage?: () => void;
  isDistributingPage?: boolean;
};

export function SpaceDaoInfoCard({
  dao,
  balance,
  balanceLoading = false,
  isDepositOpen = false,
  depositAmount = '',
  isDepositing = false,
  onOpenDeposit,
  onCloseDeposit,
  onDepositAmountChange,
  onConfirmDeposit,
  isUpdating = false,
  onUpdateDao,
  samples,
  samplesBookmark,
  canPrevSample = false,
  canNextSample = false,
  samplesLoading = false,
  showSamples = true,
  showEdit = true,
  showDeposit = true,
  canDistributeReward = false,
  onNextSample,
  onPrevSample,
  onDistributePage,
  isDistributingPage = false,
}: SpaceDaoInfoCardProps) {
  const { t } = useTranslation('SpaceDaoEditor');
  const [copied, setCopied] = useState(false);
  const [isEditing, setIsEditing] = useState(false);
  const [samplingValue, setSamplingValue] = useState(
    String(dao.sampling_count ?? ''),
  );
  const [rewardValue, setRewardValue] = useState(
    String(dao.reward_amount ?? ''),
  );
  const explorerUrl = config.block_explorer_url
    ? `${config.block_explorer_url}/address/${dao.contract_address}`
    : null;

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(dao.contract_address);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (error) {
      console.error('Failed to copy:', error);
    }
  };

  const handleEdit = () => {
    setSamplingValue(String(dao.sampling_count ?? ''));
    setRewardValue(String(dao.reward_amount ?? ''));
    setIsEditing(true);
  };

  const handleCancelEdit = () => {
    setIsEditing(false);
  };

  const handleSaveEdit = async () => {
    if (!onUpdateDao) return;
    await onUpdateDao(samplingValue, rewardValue);
    setIsEditing(false);
  };

  return (
    <Card>
      <div className="space-y-4 w-full">
        <div className="flex items-start justify-between gap-4">
          <div>
            <h3 className="text-xl font-semibold text-text-primary mb-1">
              {t('dao_info_title')}
            </h3>
            <p className="text-sm text-text-secondary">
              {t('dao_info_description')}
            </p>
          </div>
          <div className="px-3 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded-full text-sm font-medium">
            {t('dao_info_status_active')}
          </div>
        </div>

        <div className="light:bg-slate-50 bg-neutral-500/40 rounded-md px-4 py-3">
          <div className="flex items-center justify-between gap-3">
            <code className="text-base font-mono text-text-primary break-all">
              {dao.contract_address}
            </code>
            <button
              onClick={handleCopy}
              className="shrink-0 p-2 hover:bg-slate-200 dark:hover:bg-slate-800 rounded transition-colors"
              title={t('dao_info_copy')}
            >
              {copied ? (
                <CheckIcon className="w-5 h-5 text-green-600" />
              ) : (
                <ClipboardIcon className="w-5 h-5 text-text-secondary" />
              )}
            </button>
          </div>
        </div>

        <div className="grid grid-cols-2 gap-4 text-sm">
          <div>
            <p className="text-text-secondary mb-1">
              {t('dao_info_sampling_count')}
            </p>
            {isEditing ? (
              <Input
                type="number"
                min={1}
                value={samplingValue}
                onChange={(e) => setSamplingValue(e.target.value)}
              />
            ) : (
              <p className="text-base text-text-primary">
                {dao.sampling_count}
              </p>
            )}
          </div>
          <div>
            <p className="text-text-secondary mb-1">
              {t('dao_info_reward_amount')}
            </p>
            {isEditing ? (
              <Input
                type="number"
                min={1}
                value={rewardValue}
                onChange={(e) => setRewardValue(e.target.value)}
              />
            ) : (
              <p className="text-base text-text-primary">{dao.reward_amount}</p>
            )}
          </div>
        </div>

        <div>
          <p className="text-text-secondary mb-1 text-sm">
            {t('dao_info_balance_label')}
          </p>
          <p className="text-base text-text-primary">
            {balanceLoading
              ? t('dao_info_balance_loading')
              : (balance ?? t('dao_info_balance_unavailable'))}
          </p>
        </div>

        <div className="flex flex-wrap items-center justify-end gap-2">
          {onUpdateDao && showEdit && (
            <>
              {isEditing ? (
                <>
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={handleCancelEdit}
                    disabled={isUpdating}
                  >
                    {t('dao_info_edit_cancel')}
                  </Button>
                  <Button
                    type="button"
                    variant="rounded_primary"
                    size="sm"
                    onClick={handleSaveEdit}
                    disabled={isUpdating}
                  >
                    {isUpdating
                      ? t('dao_info_edit_saving')
                      : t('dao_info_edit_save')}
                  </Button>
                </>
              ) : (
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={handleEdit}
                >
                  {t('dao_info_edit_button')}
                </Button>
              )}
            </>
          )}

          {showDeposit && (
            <Button
              type="button"
              variant="rounded_primary"
              size="sm"
              onClick={onOpenDeposit}
              disabled={!onOpenDeposit}
            >
              {t('dao_info_deposit_button')}
            </Button>
          )}

          {explorerUrl && (
            <a href={explorerUrl} target="_blank" rel="noopener noreferrer">
              <svg
                className="w-4 h-4"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
                />
              </svg>
            </a>
          )}
        </div>
      </div>

      {showSamples && (
        <SpaceDaoSampleTable
          samples={samples}
          samplesBookmark={samplesBookmark}
          samplesLoading={samplesLoading}
          canPrevSample={canPrevSample}
          canNextSample={canNextSample}
          onPrevSample={onPrevSample}
          onNextSample={onNextSample}
          canDistributeReward={canDistributeReward}
          onDistributePage={onDistributePage}
          isDistributingPage={isDistributingPage}
        />
      )}

      {showDeposit && onCloseDeposit && onDepositAmountChange && onConfirmDeposit && (
        <SpaceDaoDepositDialog
          open={isDepositOpen}
          depositAmount={depositAmount}
          isDepositing={isDepositing}
          onClose={onCloseDeposit}
          onDepositAmountChange={onDepositAmountChange}
          onConfirmDeposit={onConfirmDeposit}
        />
      )}
    </Card>
  );
}
