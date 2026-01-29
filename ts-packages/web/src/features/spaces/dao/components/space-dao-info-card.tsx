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
  withdrawalAmount?: string;
  onWithdrawalAmountChange?: (value: string) => void;
  onProposeWithdrawal?: () => void;
  isWithdrawing?: boolean;
  proposals?: {
    id: number;
    proposer: string;
    amount: string;
    approvals: number;
    executed: boolean;
    approvedByMe: boolean;
  }[];
  proposalsLoading?: boolean;
  onApproveWithdrawal?: (id: number) => void;
  isApprovingWithdrawal?: boolean;
  availableShare?: string | null;
  availableShareLoading?: boolean;
  depositorCount?: number | null;
  canApproveWithdrawal?: boolean;
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
  withdrawalAmount = '',
  onWithdrawalAmountChange,
  onProposeWithdrawal,
  isWithdrawing = false,
  proposals = [],
  proposalsLoading = false,
  onApproveWithdrawal,
  isApprovingWithdrawal = false,
  availableShare,
  availableShareLoading = false,
  depositorCount,
  canApproveWithdrawal = false,
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

        <div className="grid gap-4 rounded-md border border-border/60 px-4 py-4">
          <div>
            <p className="text-sm font-medium text-text-primary">
              {t('dao_withdraw_title')}
            </p>
            <p className="text-sm text-text-secondary">
              {t('dao_withdraw_description')}
            </p>
          </div>
          <div className="grid gap-3 md:grid-cols-2">
            <div>
              <p className="text-xs text-text-secondary mb-1">
                {t('dao_withdraw_available')}
              </p>
              <p className="text-base text-text-primary">
                {availableShareLoading
                  ? t('dao_withdraw_loading')
                  : (availableShare ?? t('dao_info_balance_unavailable'))}
              </p>
            </div>
            <div>
              <p className="text-xs text-text-secondary mb-1">
                {t('dao_withdraw_depositor_count')}
              </p>
              <p className="text-base text-text-primary">
                {typeof depositorCount === 'number'
                  ? depositorCount
                  : t('dao_info_balance_unavailable')}
              </p>
            </div>
          </div>
          <div className="flex flex-col gap-2 md:flex-row md:items-end">
            <div className="flex-1">
              <p className="text-xs text-text-secondary mb-1">
                {t('dao_withdraw_amount_label')}
              </p>
              <Input
                type="number"
                min={0}
                value={withdrawalAmount}
                onChange={(e) => onWithdrawalAmountChange?.(e.target.value)}
                placeholder={t('dao_withdraw_amount_placeholder')}
                disabled={!onWithdrawalAmountChange}
              />
            </div>
            <Button
              type="button"
              variant="rounded_primary"
              size="sm"
              onClick={onProposeWithdrawal}
              disabled={!onProposeWithdrawal || isWithdrawing}
              className="md:self-end"
            >
              {isWithdrawing
                ? t('dao_withdraw_requesting')
                : t('dao_withdraw_request_button')}
            </Button>
          </div>

          <div className="space-y-2">
            <p className="text-sm font-medium text-text-primary">
              {t('dao_withdraw_proposals')}
            </p>
            {proposalsLoading ? (
              <p className="text-sm text-text-secondary">
                {t('dao_withdraw_loading')}
              </p>
            ) : proposals.length === 0 ? (
              <p className="text-sm text-text-secondary">
                {t('dao_withdraw_empty')}
              </p>
            ) : (
              <div className="space-y-2">
                {proposals.map((proposal) => (
                  <div
                    key={proposal.id}
                    className="flex flex-col gap-2 rounded-md border border-border/60 px-3 py-2 md:flex-row md:items-center md:justify-between"
                  >
                    <div className="text-sm text-text-primary">
                      <p>
                        {t('dao_withdraw_proposal_id', { id: proposal.id })}
                      </p>
                      <p className="text-xs text-text-secondary">
                        {t('dao_withdraw_proposal_amount', {
                          amount: proposal.amount,
                        })}
                      </p>
                      <p className="text-xs text-text-secondary">
                        {t('dao_withdraw_proposal_approvals', {
                          approvals: proposal.approvals,
                        })}
                      </p>
                      <p className="text-xs text-text-secondary">
                        {proposal.executed
                          ? t('dao_withdraw_proposal_executed')
                          : t('dao_withdraw_proposal_pending')}
                      </p>
                    </div>
                    {!proposal.executed &&
                      canApproveWithdrawal &&
                      !proposal.approvedByMe && (
                      <Button
                        type="button"
                        variant="outline"
                        size="sm"
                        onClick={() => onApproveWithdrawal?.(proposal.id)}
                        disabled={!onApproveWithdrawal || isApprovingWithdrawal}
                      >
                        {isApprovingWithdrawal
                          ? t('dao_withdraw_approving')
                          : t('dao_withdraw_approve_button')}
                      </Button>
                    )}
                  </div>
                ))}
              </div>
            )}
          </div>
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
