import { useTranslation } from 'react-i18next';
import { SpacePathProps } from '@/features/space-path-props';
import { Input } from '@/components/ui/input';
import { logger } from '@/lib/logger';
import { useSpaceIncentiveEditorController } from './space-incentive-editor-controller';
import Card from '@/components/card';
import { useSpaceIncentive } from '@/features/spaces/incentive/hooks/use-space-incentive';
import { SpaceIncentiveInfoCard } from '@/features/spaces/incentive/components/space-incentive-info-card';
import { useSpaceIncentiveTokens } from '@/features/spaces/incentive/hooks/use-space-incentive-tokens';
import { useRefreshSpaceIncentiveTokensMutation } from '@/features/spaces/incentive/hooks/use-refresh-space-incentive-tokens-mutation';
import { useEffect, useMemo, useState } from 'react';
import { config } from '@/config';
import RadioButton from '@/components/radio-button';

const isZeroBalance = (balance?: string | null) =>
  !balance || /^0+$/.test(balance);

export function SpaceIncentiveEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceIncentiveEditorPage: spacePk=${spacePk}`);
  const { t } = useTranslation('SpaceIncentiveEditor');
  const { data: incentive, isLoading } = useSpaceIncentive(spacePk);
  const {
    data: tokenList,
    isLoading: tokensLoading,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = useSpaceIncentiveTokens(spacePk, 5, Boolean(incentive?.contract_address));
  const refreshTokens = useRefreshSpaceIncentiveTokensMutation(spacePk);
  const [selectedToken, setSelectedToken] = useState<string | null>(null);
  const [didRefreshTokens, setDidRefreshTokens] = useState(false);
  const [tokenPageIndex, setTokenPageIndex] = useState(0);

  const allTokens = useMemo(
    () => tokenList?.pages.flatMap((page) => page.items) ?? [],
    [tokenList?.pages],
  );
  const filteredTokens = useMemo(() => {
    const usdt = config.usdt_address?.toLowerCase();
    if (!usdt) return allTokens;
    return allTokens.filter((item) => {
      const isUsdt = item.token_address.toLowerCase() === usdt;
      if (!isUsdt) return true;
      return !isZeroBalance(item.balance);
    });
  }, [allTokens]);

  const orderedTokens = useMemo(() => {
    const items = filteredTokens;
    const usdt = config.usdt_address?.toLowerCase();
    if (!usdt || items.length === 0) {
      return items;
    }
    return [...items].sort((a, b) => {
      const aIsUsdt = a.token_address.toLowerCase() === usdt;
      const bIsUsdt = b.token_address.toLowerCase() === usdt;
      if (aIsUsdt === bIsUsdt) return 0;
      return aIsUsdt ? -1 : 1;
    });
  }, [filteredTokens]);
  const hasAnyTokens = filteredTokens.length > 0;

  const tokenPages = tokenList?.pages ?? [];
  const tokenPage = tokenPages[tokenPageIndex] ?? tokenPages[0];
  const visibleTokens = useMemo(() => {
    const items =
      tokenPage?.items?.filter((item) => {
        const usdt = config.usdt_address?.toLowerCase();
        if (!usdt) return true;
        const isUsdt = item.token_address.toLowerCase() === usdt;
        if (!isUsdt) return true;
        return !isZeroBalance(item.balance);
      }) ?? [];
    const usdt = config.usdt_address?.toLowerCase();
    if (!usdt || items.length === 0) return items;
    return [...items].sort((a, b) => {
      const aIsUsdt = a.token_address.toLowerCase() === usdt;
      const bIsUsdt = b.token_address.toLowerCase() === usdt;
      if (aIsUsdt === bIsUsdt) return 0;
      return aIsUsdt ? -1 : 1;
    });
  }, [tokenPage, tokenPage?.items]);
  const hasPrevPage = tokenPageIndex > 0;
  const canGoNext =
    tokenPageIndex < tokenPages.length - 1 || Boolean(hasNextPage);

  useEffect(() => {
    if (selectedToken) return;
    const items = orderedTokens;
    if (items.length) {
      const usdt = config.usdt_address?.toLowerCase();
      const usdtItem = usdt
        ? items.find((item) => item.token_address.toLowerCase() === usdt)
        : null;
      setSelectedToken(
        (usdtItem?.token_address ?? items[0].token_address) || null,
      );
      return;
    }
    if (config.usdt_address) {
      setSelectedToken(config.usdt_address);
    }
  }, [selectedToken, orderedTokens]);

  useEffect(() => {
    if (!incentive?.contract_address || didRefreshTokens) return;
    refreshTokens.mutate();
    setDidRefreshTokens(true);
  }, [incentive?.contract_address, didRefreshTokens, refreshTokens]);

  useEffect(() => {
    if (tokenPageIndex > 0 && tokenPageIndex > tokenPages.length - 1) {
      setTokenPageIndex(Math.max(tokenPages.length - 1, 0));
    }
  }, [tokenPageIndex, tokenPages.length]);

  const selectedTokenItem =
    orderedTokens.find(
      (item) =>
        item.token_address.toLowerCase() === selectedToken?.toLowerCase(),
    ) ?? null;
  const preferredTokenItem = orderedTokens[0] ?? null;
  const preferredToken = preferredTokenItem?.token_address ?? selectedToken;
  const preferredTokenBalance =
    preferredTokenItem?.balance ??
    (preferredToken?.toLowerCase() === config.usdt_address?.toLowerCase()
      ? '0'
      : null);
  const preferredTokenDecimals =
    preferredTokenItem?.decimals ??
    (preferredToken?.toLowerCase() === config.usdt_address?.toLowerCase()
      ? 6
      : null);
  const fallbackIsUsdt =
    Boolean(selectedToken && config.usdt_address) &&
    selectedToken?.toLowerCase() === config.usdt_address?.toLowerCase();
  const selectedTokenBalance =
    selectedTokenItem?.balance ?? (fallbackIsUsdt ? '0' : null);
  const selectedTokenDecimals =
    selectedTokenItem?.decimals ?? (fallbackIsUsdt ? 6 : null);

  const ctrl = useSpaceIncentiveEditorController(
    spacePk,
    incentive,
    selectedToken,
    selectedTokenBalance,
    selectedTokenDecimals,
    preferredToken ?? null,
    preferredTokenBalance,
    preferredTokenDecimals,
  );

  if (!ctrl.space || isLoading) {
    return null;
  }

  return (
    <div className="flex flex-col w-full max-w-[1152px] gap-5">
      <Card>
        <div className="mb-4">
          <h1 className="text-3xl font-bold text-text-primary mb-2">
            {t('title')}
          </h1>
          <p className="text-text-secondary">{t('description')}</p>
        </div>

        <div className="w-full">
          {!ctrl.isTeamSpace && !incentive && (
            <div className="mb-4 p-3 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-md">
              <p className="text-sm text-blue-800 dark:text-blue-200">
                {t('personal_space_notice')}
              </p>
            </div>
          )}

          {incentive ? (
            <SpaceIncentiveInfoCard
              incentive={incentive}
              recipientCount={ctrl.chainRecipientCount.get()}
              incentiveMode={ctrl.chainIncentiveMode.get()}
              rankingBps={ctrl.chainRankingBps.get()}
              isUpdating={ctrl.isUpdating.get()}
              onUpdateIncentive={ctrl.handleUpdateIncentive}
              showEdit={Boolean(ctrl.space?.isDraft)}
              tokens={visibleTokens}
              tokenHasAny={hasAnyTokens}
              tokensLoading={tokensLoading}
              onRefreshTokens={() => refreshTokens.mutate()}
              isRefreshingTokens={refreshTokens.isPending}
              tokenHasPrev={hasPrevPage}
              tokenHasNext={canGoNext}
              isFetchingNextTokenPage={isFetchingNextPage}
              onPrevTokens={() =>
                setTokenPageIndex((prev) => Math.max(prev - 1, 0))
              }
              onNextTokens={async () => {
                if (tokenPageIndex < tokenPages.length - 1) {
                  setTokenPageIndex((prev) => prev + 1);
                  return;
                }
                if (hasNextPage) {
                  await fetchNextPage();
                  setTokenPageIndex((prev) => prev + 1);
                }
              }}
            />
          ) : (
            <>
              <>
                <div className="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-md p-4 mb-6">
                  <div className="flex gap-3">
                    <div className="w-6 h-6 rounded-full border border-blue-300 text-blue-600 dark:text-blue-400 flex items-center justify-center shrink-0">
                      i
                    </div>
                    <div>
                      <h4 className="font-semibold text-blue-900 dark:text-blue-100 mb-2">
                        {t('admin_requirements')}
                      </h4>
                      <p className="text-sm text-blue-800 dark:text-blue-200 whitespace-pre-line">
                        {ctrl.isTeamSpace
                          ? t('admin_requirements_description')
                          : t('admin_requirements_description_personal')}
                      </p>
                    </div>
                  </div>
                </div>

                <div className="flex items-center justify-between mb-6">
                  <div>
                    <p className="text-sm text-text-secondary mb-1">
                      {ctrl.isTeamSpace
                        ? t('eligible_admins_count', {
                            count: ctrl.teamMembers.filter((m) => m.evm_address)
                              .length,
                          })
                        : t('personal_admin_requirement')}
                    </p>
                    <p className="text-xs text-text-tertiary">
                      {ctrl.isTeamSpace
                        ? t('min_admins_required')
                        : t('personal_admin_requirement_hint')}
                    </p>
                  </div>
                </div>

                {!ctrl.canRegisterIncentive && (
                  <div className="mb-4 p-3 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-md">
                    <p className="text-sm text-yellow-800 dark:text-yellow-200">
                      {ctrl.isTeamSpace
                        ? t('insufficient_admins')
                        : t('insufficient_admins_personal')}
                    </p>
                  </div>
                )}
              </>
              <div className="grid gap-5">
                <div>
                  <label className="text-sm text-text-secondary mb-2 block">
                    {t('incentive_mode_label')}
                  </label>
                  <div className="flex flex-col gap-2">
                    <div
                      role="button"
                      tabIndex={0}
                      onClick={() => ctrl.incentiveMode.set(0)}
                      onKeyDown={(event) => {
                        if (event.key === 'Enter' || event.key === ' ') {
                          event.preventDefault();
                          ctrl.incentiveMode.set(0);
                        }
                      }}
                      className="flex items-center gap-2 text-sm text-text-secondary cursor-pointer"
                    >
                      <RadioButton
                        selected={ctrl.incentiveMode.get() === 0}
                        onClick={() => ctrl.incentiveMode.set(0)}
                      />
                      {t('incentive_mode_random')}
                    </div>
                    <div
                      role="button"
                      tabIndex={0}
                      onClick={() => ctrl.incentiveMode.set(1)}
                      onKeyDown={(event) => {
                        if (event.key === 'Enter' || event.key === ' ') {
                          event.preventDefault();
                          ctrl.incentiveMode.set(1);
                        }
                      }}
                      className="flex items-center gap-2 text-sm text-text-secondary cursor-pointer"
                    >
                      <RadioButton
                        selected={ctrl.incentiveMode.get() === 1}
                        onClick={() => ctrl.incentiveMode.set(1)}
                      />
                      {t('incentive_mode_ranking')}
                    </div>
                    <div
                      role="button"
                      tabIndex={0}
                      onClick={() => ctrl.incentiveMode.set(2)}
                      onKeyDown={(event) => {
                        if (event.key === 'Enter' || event.key === ' ') {
                          event.preventDefault();
                          ctrl.incentiveMode.set(2);
                        }
                      }}
                      className="flex items-center gap-2 text-sm text-text-secondary cursor-pointer"
                    >
                      <RadioButton
                        selected={ctrl.incentiveMode.get() === 2}
                        onClick={() => ctrl.incentiveMode.set(2)}
                      />
                      {t('incentive_mode_mixed')}
                    </div>
                  </div>
                </div>
                <div>
                  <label className="text-sm text-text-secondary mb-2 block">
                    {t('incentive_count_label')}
                  </label>
                  <Input
                    type="number"
                    min={0}
                    max={100}
                    value={ctrl.incentiveCount.get()}
                    onChange={(e) => {
                      const next = e.target.value;
                      const numeric = Number(next);
                      if (
                        next === '' ||
                        (Number.isFinite(numeric) &&
                          numeric >= 0 &&
                          numeric <= 100)
                      ) {
                        ctrl.incentiveCount.set(next);
                      }
                    }}
                    placeholder={t('incentive_count_placeholder')}
                  />
                </div>
                {ctrl.incentiveMode.get() === 2 && (
                  <div>
                    <label className="text-sm text-text-secondary mb-2 block">
                      {t('incentive_mode_ranking_ratio_label')}
                    </label>
                    <Input
                      type="number"
                      min={0}
                      max={100}
                      value={ctrl.rankingBps.get()}
                      onChange={(e) => {
                        const next = e.target.value;
                        const numeric = Number(next);
                        if (
                          next === '' ||
                          (Number.isFinite(numeric) &&
                            numeric >= 0 &&
                            numeric <= 100)
                        ) {
                          ctrl.rankingBps.set(next);
                        }
                      }}
                      placeholder={t(
                        'incentive_mode_ranking_ratio_placeholder',
                      )}
                    />
                  </div>
                )}
              </div>

              <button
                onClick={ctrl.handleOpenRegistrationPopup}
                disabled={!ctrl.canRegisterIncentive || !ctrl.canSubmitInputs}
                className="mt-6 w-full px-6 py-3 bg-primary text-white rounded-md font-medium hover:bg-primary-dark transition-colors disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-primary"
              >
                {t('register_incentive')}
              </button>
            </>
          )}
        </div>
      </Card>
    </div>
  );
}
