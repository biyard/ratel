import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import Card from '@/components/card';
import { SpaceDaoResponse } from '@/features/spaces/dao/hooks/use-space-dao';
import { SpaceDaoRewardResponse } from '@/features/spaces/dao/hooks/use-space-dao-reward';
import { SpaceDaoTokenResponse } from '@/features/spaces/dao/hooks/use-space-dao-tokens';
import { SpaceDaoRewardTable } from './space-dao-reward-table';
import { config } from '@/config';
import {
  ArrowPathIcon,
  CheckIcon,
  ClipboardIcon,
} from '@heroicons/react/24/outline';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { ethers } from 'ethers';

type SpaceDaoInfoCardProps = {
  dao: SpaceDaoResponse;
  isUpdating?: boolean;
  recipientCount?: string | number | null;
  onUpdateDao?: (rewardCount: string) => Promise<void>;
  rewardRecipients?: SpaceDaoRewardResponse[];
  rewardBookmark?: string | null;
  canPrevReward?: boolean;
  canNextReward?: boolean;
  rewardLoading?: boolean;
  showRewardRecipients?: boolean;
  showEdit?: boolean;
  canDistributeReward?: boolean;
  onNextReward?: () => void;
  onPrevReward?: () => void;
  onDistributePage?: () => void;
  isDistributingPage?: boolean;
  // withdrawal props removed
  tokens?: SpaceDaoTokenResponse[];
  selectedToken?: string | null;
  onSelectToken?: (tokenAddress: string) => void;
  tokensLoading?: boolean;
  onRefreshTokens?: () => void;
  isRefreshingTokens?: boolean;
};

export function SpaceDaoInfoCard({
  dao,
  isUpdating = false,
  recipientCount,
  onUpdateDao,
  rewardRecipients,
  rewardBookmark,
  canPrevReward = false,
  canNextReward = false,
  rewardLoading = false,
  showRewardRecipients = true,
  showEdit = true,
  canDistributeReward = false,
  onNextReward,
  onPrevReward,
  onDistributePage,
  isDistributingPage = false,
  tokens = [],
  selectedToken,
  onSelectToken,
  tokensLoading = false,
  onRefreshTokens,
  isRefreshingTokens = false,
}: SpaceDaoInfoCardProps) {
  const { t } = useTranslation('SpaceDaoEditor');
  const [copied, setCopied] = useState(false);
  const [isEditing, setIsEditing] = useState(false);
  const [rewardCountValue, setRewardCountValue] = useState(
    String(recipientCount ?? ''),
  );
  const explorerUrl = config.block_explorer_url
    ? `${config.block_explorer_url}/address/${dao.contract_address}`
    : null;

  const tokensWithDefault = (() => {
    if (!config.usdt_address) return tokens;
    const hasUsdt = tokens.some(
      (item) =>
        item.token_address.toLowerCase() === config.usdt_address.toLowerCase(),
    );
    if (hasUsdt) return tokens;
    return [
      {
        token_address: config.usdt_address,
        symbol: 'USDT',
        decimals: 6,
        balance: '0',
        updated_at: Date.now(),
        sk: `TOKEN#${config.usdt_address}`,
        pk: `SPACE_DAO#${dao.contract_address}`,
      },
      ...tokens,
    ];
  })();

  const selectedTokenItem =
    tokensWithDefault.find((item) => item.token_address === selectedToken) ??
    null;
  const formattedTokenBalance = selectedTokenItem
    ? formatTokenBalance(selectedTokenItem.balance, selectedTokenItem.decimals)
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
    setRewardCountValue(String(recipientCount ?? ''));
    setIsEditing(true);
  };

  const handleCancelEdit = () => {
    setIsEditing(false);
  };

  const handleSaveEdit = async () => {
    if (!onUpdateDao) return;
    await onUpdateDao(rewardCountValue);
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

        <div className="grid grid-cols-1 gap-4 text-sm">
          <div>
            <p className="text-text-secondary mb-1">
              {t('dao_info_reward_count')}
            </p>
            {isEditing ? (
              <Input
                type="number"
                min={1}
                value={rewardCountValue}
                onChange={(e) => setRewardCountValue(e.target.value)}
              />
            ) : (
              <p className="text-base text-text-primary">
                {recipientCount ?? '-'}
              </p>
            )}
          </div>
        </div>

        <div className="space-y-2">
          <div className="flex items-center gap-2.5">
            <p className="text-text-secondary text-sm">
              {t('dao_info_token_label')}
            </p>
            {onRefreshTokens && (
              <div
                role="button"
                tabIndex={0}
                onClick={() => {
                  if (!isRefreshingTokens) onRefreshTokens();
                }}
                className={`${
                  isRefreshingTokens
                    ? 'opacity-60 cursor-not-allowed'
                    : 'cursor-pointer hover:bg-muted/40'
                }`}
                aria-disabled={isRefreshingTokens}
              >
                <ArrowPathIcon
                  className={`h-4 w-4 ${
                    isRefreshingTokens ? 'animate-spin' : ''
                  }`}
                />
              </div>
            )}
          </div>
          <select
            className="w-full rounded-md border border-border bg-background px-3 py-2 text-sm text-text-primary"
            value={selectedToken ?? ''}
            onChange={(e) => onSelectToken?.(e.target.value)}
            disabled={tokensLoading || tokensWithDefault.length === 0}
          >
            {tokensWithDefault.length === 0 ? (
              <option value="">{t('dao_info_token_empty')}</option>
            ) : (
              tokensWithDefault.map((item) => (
                <option key={item.token_address} value={item.token_address}>
                  {item.symbol || item.token_address}
                </option>
              ))
            )}
          </select>
          <p className="text-base text-text-primary">
            {tokensLoading
              ? t('dao_info_token_loading')
              : (formattedTokenBalance ?? t('dao_info_balance_unavailable'))}
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

      {showRewardRecipients && (
        <SpaceDaoRewardTable
          rewardRecipients={rewardRecipients}
          rewardBookmark={rewardBookmark}
          rewardLoading={rewardLoading}
          canPrevReward={canPrevReward}
          canNextReward={canNextReward}
          onPrevReward={onPrevReward}
          onNextReward={onNextReward}
          canDistributeReward={canDistributeReward}
          onDistributePage={onDistributePage}
          isDistributingPage={isDistributingPage}
        />
      )}
    </Card>
  );
}

function formatTokenBalance(balance: string, decimals: number) {
  try {
    if (!balance) return '0';
    return ethers.formatUnits(balance, decimals ?? 0);
  } catch {
    return balance;
  }
}
