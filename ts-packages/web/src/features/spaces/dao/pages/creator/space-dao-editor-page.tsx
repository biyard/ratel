import { useTranslation } from 'react-i18next';
import { SpacePathProps } from '@/features/space-path-props';
import { Input } from '@/components/ui/input';
import { logger } from '@/lib/logger';
import { useSpaceDaoEditorController } from './space-dao-editor-controller';
import Card from '@/components/card';
import { useSpaceDao } from '@/features/spaces/dao/hooks/use-space-dao';
import { SpaceDaoInfoCard } from '@/features/spaces/dao/components/space-dao-info-card';
import { useSpaceDaoTokens } from '@/features/spaces/dao/hooks/use-space-dao-tokens';
import { useRefreshSpaceDaoTokensMutation } from '@/features/spaces/dao/hooks/use-refresh-space-dao-tokens-mutation';
import { useEffect, useMemo, useState } from 'react';
import { config } from '@/config';
import RadioButton from '@/components/radio-button';

export function SpaceDaoEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceDaoEditorPage: spacePk=${spacePk}`);
  const { t } = useTranslation('SpaceDaoEditor');
  const { data: dao, isLoading } = useSpaceDao(spacePk);
  const { data: tokenList, isLoading: tokensLoading } = useSpaceDaoTokens(
    spacePk,
    50,
    Boolean(dao?.contract_address),
  );
  const refreshTokens = useRefreshSpaceDaoTokensMutation(spacePk);
  const [selectedToken, setSelectedToken] = useState<string | null>(null);
  const [didRefreshTokens, setDidRefreshTokens] = useState(false);

  const orderedTokens = useMemo(() => {
    const items = tokenList?.items ?? [];
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
  }, [tokenList?.items]);

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
    if (!dao?.contract_address || didRefreshTokens) return;
    refreshTokens.mutate();
    setDidRefreshTokens(true);
  }, [dao?.contract_address, didRefreshTokens, refreshTokens]);

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

  const ctrl = useSpaceDaoEditorController(
    spacePk,
    dao,
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
          {!ctrl.isTeamSpace && !dao && (
            <div className="mb-4 p-3 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-md">
              <p className="text-sm text-blue-800 dark:text-blue-200">
                {t('personal_space_notice')}
              </p>
            </div>
          )}

          {dao ? (
            <SpaceDaoInfoCard
              dao={dao}
              recipientCount={ctrl.chainRecipientCount.get()}
              rewardMode={ctrl.chainRewardMode.get()}
              rankingBps={ctrl.chainRankingBps.get()}
              isUpdating={ctrl.isUpdating.get()}
              onUpdateDao={ctrl.handleUpdateDao}
              rewardRecipients={ctrl.visibleRewardRecipients}
              rewardRemainingCount={ctrl.rewardMeta?.remaining_count ?? null}
              rewardTotalCount={ctrl.rewardMeta?.total_count ?? null}
              rewardLoading={ctrl.rewardRecipientsLoading}
              showRewardRecipients={Boolean(ctrl.space?.isFinished)}
              showEdit={Boolean(ctrl.space?.isDraft)}
              currentUserEvm={ctrl.currentUserEvm}
              claimableAmount={ctrl.perRecipientDisplay}
              isClaimable={ctrl.canClaimReward}
              isClaiming={ctrl.isClaiming.get()}
              onClaimReward={async (rewardSk) => {
                await ctrl.handleClaimReward(rewardSk);
                refreshTokens.mutate();
              }}
              tokens={orderedTokens}
              selectedToken={selectedToken}
              onSelectToken={setSelectedToken}
              tokensLoading={tokensLoading}
              onRefreshTokens={() => refreshTokens.mutate()}
              isRefreshingTokens={refreshTokens.isPending}
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

                {!ctrl.canRegisterDao && (
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
                    {t('reward_mode_label')}
                  </label>
                  <div className="flex flex-col gap-2">
                    <div
                      role="button"
                      tabIndex={0}
                      onClick={() => ctrl.rewardMode.set(0)}
                      onKeyDown={(event) => {
                        if (event.key === 'Enter' || event.key === ' ') {
                          event.preventDefault();
                          ctrl.rewardMode.set(0);
                        }
                      }}
                      className="flex items-center gap-2 text-sm text-text-secondary cursor-pointer"
                    >
                      <RadioButton
                        selected={ctrl.rewardMode.get() === 0}
                        onClick={() => ctrl.rewardMode.set(0)}
                      />
                      {t('reward_mode_random')}
                    </div>
                    <div
                      role="button"
                      tabIndex={0}
                      onClick={() => ctrl.rewardMode.set(1)}
                      onKeyDown={(event) => {
                        if (event.key === 'Enter' || event.key === ' ') {
                          event.preventDefault();
                          ctrl.rewardMode.set(1);
                        }
                      }}
                      className="flex items-center gap-2 text-sm text-text-secondary cursor-pointer"
                    >
                      <RadioButton
                        selected={ctrl.rewardMode.get() === 1}
                        onClick={() => ctrl.rewardMode.set(1)}
                      />
                      {t('reward_mode_ranking')}
                    </div>
                    <div
                      role="button"
                      tabIndex={0}
                      onClick={() => ctrl.rewardMode.set(2)}
                      onKeyDown={(event) => {
                        if (event.key === 'Enter' || event.key === ' ') {
                          event.preventDefault();
                          ctrl.rewardMode.set(2);
                        }
                      }}
                      className="flex items-center gap-2 text-sm text-text-secondary cursor-pointer"
                    >
                      <RadioButton
                        selected={ctrl.rewardMode.get() === 2}
                        onClick={() => ctrl.rewardMode.set(2)}
                      />
                      {t('reward_mode_mixed')}
                    </div>
                  </div>
                </div>
                <div>
                  <label className="text-sm text-text-secondary mb-2 block">
                    {t('reward_count_label')}
                  </label>
                  <Input
                    type="number"
                    min={0}
                    max={100}
                    value={ctrl.rewardCount.get()}
                    onChange={(e) => {
                      const next = e.target.value;
                      const numeric = Number(next);
                      if (
                        next === '' ||
                        (Number.isFinite(numeric) &&
                          numeric >= 0 &&
                          numeric <= 100)
                      ) {
                        ctrl.rewardCount.set(next);
                      }
                    }}
                    placeholder={t('reward_count_placeholder')}
                  />
                </div>
                {ctrl.rewardMode.get() === 2 && (
                  <div>
                    <label className="text-sm text-text-secondary mb-2 block">
                      {t('reward_mode_ranking_ratio_label')}
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
                      placeholder={t('reward_mode_ranking_ratio_placeholder')}
                    />
                  </div>
                )}
              </div>

              <button
                onClick={ctrl.handleOpenRegistrationPopup}
                disabled={!ctrl.canRegisterDao || !ctrl.canSubmitInputs}
                className="mt-6 w-full px-6 py-3 bg-primary text-white rounded-md font-medium hover:bg-primary-dark transition-colors disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-primary"
              >
                {t('register_dao')}
              </button>
            </>
          )}
        </div>
      </Card>
    </div>
  );
}
