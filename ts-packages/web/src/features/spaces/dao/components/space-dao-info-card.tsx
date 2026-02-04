import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import Card from '@/components/card';
import { SpaceDaoResponse } from '@/features/spaces/dao/hooks/use-space-dao';
import { SpaceDaoTokenResponse } from '@/features/spaces/dao/hooks/use-space-dao-tokens';
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
  incentiveMode?: number | null;
  rankingBps?: number | null;
  onUpdateDao?: (
    incentiveCount: string,
    rankingRatio?: string,
  ) => Promise<void>;
  showEdit?: boolean;
  currentUserEvm?: string | null;
  claimableAmount?: string | null;
  isClaimable?: boolean;
  isClaiming?: boolean;
  showClaim?: boolean;
  // withdrawal props removed
  tokens?: SpaceDaoTokenResponse[];
  tokensLoading?: boolean;
  onRefreshTokens?: () => void;
  isRefreshingTokens?: boolean;
  tokenHasPrev?: boolean;
  tokenHasNext?: boolean;
  tokenHasAny?: boolean;
  isFetchingNextTokenPage?: boolean;
  onPrevTokens?: () => void;
  onNextTokens?: () => void;
};

export function SpaceDaoInfoCard({
  dao,
  isUpdating = false,
  recipientCount,
  incentiveMode,
  rankingBps,
  onUpdateDao,
  showEdit = true,
  tokens = [],
  tokensLoading = false,
  onRefreshTokens,
  isRefreshingTokens = false,
  tokenHasPrev = false,
  tokenHasNext = false,
  tokenHasAny = false,
  isFetchingNextTokenPage = false,
  onPrevTokens,
  onNextTokens,
}: SpaceDaoInfoCardProps) {
  const { t } = useTranslation('SpaceDaoEditor');
  const [copied, setCopied] = useState(false);
  const [isEditing, setIsEditing] = useState(false);
  const [incentiveCountValue, setIncentiveCountValue] = useState(
    String(recipientCount ?? ''),
  );
  const [rankingRatioValue, setRankingRatioValue] = useState(
    rankingBps != null ? String(Math.round(rankingBps / 100)) : '',
  );
  const explorerUrl = config.block_explorer_url
    ? `${config.block_explorer_url}/address/${dao.contract_address}`
    : null;

  const tokenRows = tokens;

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
    setIncentiveCountValue(String(recipientCount ?? ''));
    setRankingRatioValue(
      rankingBps != null ? String(Math.round(rankingBps / 100)) : '',
    );
    setIsEditing(true);
  };

  const handleCancelEdit = () => {
    setIsEditing(false);
  };

  const handleSaveEdit = async () => {
    if (!onUpdateDao) return;
    const ratio = incentiveMode === 2 ? rankingRatioValue.trim() : undefined;
    await onUpdateDao(incentiveCountValue, ratio);
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
              {t('dao_info_incentive_mode')}
            </p>
            <p className="text-base text-text-primary">
              {formatIncentiveMode(t, incentiveMode)}
            </p>
          </div>
          <div>
            <p className="text-text-secondary mb-1">
              {t('dao_info_incentive_count')}
            </p>
            {isEditing ? (
              <Input
                type="number"
                min={1}
                max={100}
                value={incentiveCountValue}
                onChange={(e) => {
                  const next = e.target.value;
                  const numeric = Number(next);
                  if (
                    next === '' ||
                    (Number.isFinite(numeric) && numeric >= 0 && numeric <= 100)
                  ) {
                    setIncentiveCountValue(next);
                  }
                }}
              />
            ) : (
              <p className="text-base text-text-primary">
                {recipientCount ?? '-'}
              </p>
            )}
          </div>
          {incentiveMode === 2 && (
            <div>
              <p className="text-text-secondary mb-1">
                {t('dao_info_incentive_ranking_ratio')}
              </p>
              {isEditing ? (
                <Input
                  type="number"
                  min={0}
                  max={100}
                  value={rankingRatioValue}
                  onChange={(e) => {
                    const next = e.target.value;
                    const numeric = Number(next);
                    if (
                      next === '' ||
                      (Number.isFinite(numeric) &&
                        numeric >= 0 &&
                        numeric <= 100)
                    ) {
                      setRankingRatioValue(next);
                    }
                  }}
                />
              ) : (
                <p className="text-base text-text-primary">
                  {formatRankingRatio(rankingBps)}
                </p>
              )}
            </div>
          )}
        </div>

        {(tokensLoading || tokenHasAny) && (
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
            <div className="flex items-center justify-end gap-2 text-sm">
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={onPrevTokens}
                disabled={!tokenHasPrev}
              >
                {t('dao_selected_prev')}
              </Button>
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={onNextTokens}
                disabled={!tokenHasNext || isFetchingNextTokenPage}
              >
                {t('dao_selected_next')}
              </Button>
            </div>
            <div className="rounded-md border border-border bg-background">
              {tokensLoading ? (
                <div className="px-3 py-2 text-sm text-text-secondary">
                  {t('dao_info_token_loading')}
                </div>
              ) : tokenRows.length === 0 ? (
                <div className="px-3 py-2 text-sm text-text-secondary">
                  {t('dao_info_token_empty')}
                </div>
              ) : (
                <table className="w-full table-fixed text-sm">
                  <thead className="text-text-secondary">
                    <tr className="border-b border-border">
                      <th className="px-3 py-2 text-left font-medium">
                        {t('dao_info_token_table_token')}
                      </th>
                      <th className="px-3 py-2 text-right font-medium">
                        {t('dao_info_token_table_balance')}
                      </th>
                    </tr>
                  </thead>
                  <tbody>
                    {tokenRows.map((item) => (
                      <tr
                        key={item.token_address}
                        className="border-b border-border last:border-0"
                      >
                        <td className="px-3 py-2 text-text-primary">
                          {item.symbol || item.token_address}
                        </td>
                        <td className="px-3 py-2 text-right text-text-primary">
                          {formatTokenBalance(item.balance, item.decimals)}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              )}
            </div>
          </div>
        )}

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

function formatIncentiveMode(
  t: ReturnType<typeof useTranslation>['t'],
  mode?: number | null,
) {
  if (mode === 1) return t('incentive_mode_ranking');
  if (mode === 2) return t('incentive_mode_mixed');
  if (mode === 0) return t('incentive_mode_random');
  return '-';
}

function formatRankingRatio(rankingBps?: number | null) {
  if (rankingBps == null) return '-';
  return `${(rankingBps / 100).toFixed(2)}%`;
}
