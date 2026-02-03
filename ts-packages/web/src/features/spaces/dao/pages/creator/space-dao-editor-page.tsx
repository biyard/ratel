import { useTranslation } from 'react-i18next';
import { SpacePathProps } from '@/features/space-path-props';
import { Input } from '@/components/ui/input';
import { logger } from '@/lib/logger';
import { useSpaceDaoEditorController } from './space-dao-editor-controller';
import { RegisterDaoPopup } from '@/features/teams/dao/components/register-dao-popup';
import Card from '@/components/card';
import { useSpaceDao } from '@/features/spaces/dao/hooks/use-space-dao';
import { SpaceDaoInfoCard } from '@/features/spaces/dao/components/space-dao-info-card';
import { useSpaceDaoTokens } from '@/features/spaces/dao/hooks/use-space-dao-tokens';
import { useRefreshSpaceDaoTokensMutation } from '@/features/spaces/dao/hooks/use-refresh-space-dao-tokens-mutation';
import { useEffect, useMemo, useState } from 'react';
import { config } from '@/config';
import { Checkbox } from '@/components/checkbox/checkbox';

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
          {!ctrl.isTeamSpace && (
            <div className="mb-4 p-3 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-md">
              <p className="text-sm text-yellow-800 dark:text-yellow-200">
                {t('team_only')}
              </p>
            </div>
          )}

          {dao ? (
            <SpaceDaoInfoCard
              dao={dao}
              recipientCount={ctrl.chainRecipientCount.get()}
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
                        {t('admin_requirements_description')}
                      </p>
                    </div>
                  </div>
                </div>

                <div className="flex items-center justify-between mb-6">
                  <div>
                    <p className="text-sm text-text-secondary mb-1">
                      {t('eligible_admins_count', {
                        count: ctrl.eligibleAdmins.length,
                      })}
                    </p>
                    <p className="text-xs text-text-tertiary">
                      {t('min_admins_required')}
                    </p>
                  </div>

                  {ctrl.eligibleAdmins.length >= 3 ? (
                    <div className="px-3 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded-full text-sm font-medium">
                      ✓ {t('ready_label')}
                    </div>
                  ) : (
                    <div className="px-3 py-1 bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 rounded-full text-sm font-medium">
                      {t('need_more_label', {
                        count: 3 - ctrl.eligibleAdmins.length,
                      })}
                    </div>
                  )}
                </div>

                {!ctrl.canRegisterDao && ctrl.eligibleAdmins.length < 3 && (
                  <div className="mb-4 p-3 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-md">
                    <p className="text-sm text-yellow-800 dark:text-yellow-200">
                      {t('insufficient_admins')}
                    </p>
                  </div>
                )}
              </>
              <div className="grid gap-5">
                <div>
                  <label className="text-sm text-text-secondary mb-2 block">
                    {t('reward_count_label')}
                  </label>
                  <Input
                    type="number"
                    min={0}
                    value={ctrl.rewardCount.get()}
                    onChange={(e) => ctrl.rewardCount.set(e.target.value)}
                    placeholder={t('reward_count_placeholder')}
                  />
                </div>
                <div className="grid gap-3">
                  <label className="text-sm text-text-secondary block">
                    {t('reward_requirements_label')}
                  </label>
                  <Checkbox
                    id="reward-require-pre"
                    value={ctrl.requirePreSurvey.get()}
                    onChange={(checked) => ctrl.requirePreSurvey.set(checked)}
                  >
                    {t('reward_require_pre')}
                  </Checkbox>
                  <Checkbox
                    id="reward-require-post"
                    value={ctrl.requirePostSurvey.get()}
                    onChange={(checked) => ctrl.requirePostSurvey.set(checked)}
                  >
                    {t('reward_require_post')}
                  </Checkbox>
                </div>
              </div>

              <button
                onClick={ctrl.handleOpenRegistrationPopup}
                disabled={
                  !ctrl.canRegisterDao ||
                  !ctrl.isTeamSpace ||
                  !ctrl.canSubmitInputs
                }
                className="mt-6 w-full px-6 py-3 bg-primary text-white rounded-md font-medium hover:bg-primary-dark transition-colors disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-primary"
              >
                {t('register_dao')}
              </button>
            </>
          )}
        </div>

        {ctrl.isPopupOpen.get() && (
          <RegisterDaoPopup
            eligibleAdmins={ctrl.eligibleAdmins}
            onRegister={ctrl.handleRegisterDao}
            onCancel={ctrl.handleClosePopup}
            isRegistering={ctrl.isRegistering.get()}
          />
        )}
      </Card>
    </div>
  );
}
