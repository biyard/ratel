import { useTranslation } from 'react-i18next';
import { SpacePathProps } from '@/features/space-path-props';
import { Input } from '@/components/ui/input';
import { logger } from '@/lib/logger';
import { useSpaceDaoEditorController } from './space-dao-editor-controller';
import { RegisterDaoPopup } from '@/features/teams/dao/components/register-dao-popup';
import Card from '@/components/card';
import { useSpaceDao } from '@/features/spaces/dao/hooks/use-space-dao';
import { SpaceDaoInfoCard } from '@/features/spaces/dao/components/space-dao-info-card';

export function SpaceDaoEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceDaoEditorPage: spacePk=${spacePk}`);
  const { t } = useTranslation('SpaceDaoEditor');
  const { data: dao, isLoading } = useSpaceDao(spacePk);
  const ctrl = useSpaceDaoEditorController(spacePk, dao);

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
              balance={ctrl.balance.get()}
              balanceLoading={ctrl.balanceLoading.get()}
              isDepositOpen={ctrl.isDepositOpen.get()}
              depositAmount={ctrl.depositAmount.get()}
              isDepositing={ctrl.isDepositing.get()}
              onOpenDeposit={ctrl.handleOpenDeposit}
              onCloseDeposit={ctrl.handleCloseDeposit}
              onDepositAmountChange={ctrl.handleDepositAmountChange}
              onConfirmDeposit={ctrl.handleConfirmDeposit}
              isUpdating={ctrl.isUpdating.get()}
              onUpdateDao={ctrl.handleUpdateDao}
              samples={ctrl.visibleSamples}
              samplesBookmark={ctrl.samples?.bookmark ?? null}
              samplesLoading={ctrl.samplesLoading}
              canPrevSample={ctrl.canPrevSample}
              canNextSample={ctrl.canNextSample}
              onPrevSample={ctrl.handlePrevSample}
              onNextSample={ctrl.handleNextSample}
              showSamples={Boolean(ctrl.space?.isFinished)}
              showEdit={Boolean(ctrl.space?.isDraft)}
              showDeposit={true}
              canDistributeReward={ctrl.canDistributeReward}
              onDistributePage={ctrl.handleDistribute}
              isDistributingPage={ctrl.isDistributingPage.get()}
              withdrawalAmount={ctrl.withdrawAmount.get()}
              onWithdrawalAmountChange={ctrl.handleWithdrawAmountChange}
              onProposeWithdrawal={ctrl.handleProposeWithdrawal}
              isWithdrawing={ctrl.isWithdrawing.get()}
              proposals={ctrl.proposals.get()}
              proposalsLoading={ctrl.proposalsLoading.get()}
              onApproveWithdrawal={ctrl.handleApproveWithdrawal}
              isApprovingWithdrawal={ctrl.isApprovingWithdrawal.get()}
              availableShare={ctrl.availableShare.get()}
              availableShareLoading={ctrl.availableShareLoading.get()}
              depositorCount={ctrl.depositorCount.get()}
              canApproveWithdrawal={Number(ctrl.availableShare.get() ?? 0) > 0}
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
                    {t('sampling_count_label')}
                  </label>
                  <Input
                    type="number"
                    min={0}
                    value={ctrl.samplingCount.get()}
                    onChange={(e) => ctrl.samplingCount.set(e.target.value)}
                    placeholder={t('sampling_count_placeholder')}
                  />
                </div>

                <div>
                  <label className="text-sm text-text-secondary mb-2 block">
                    {t('reward_amount_label')}
                  </label>
                  <Input
                    type="number"
                    min={0}
                    value={ctrl.rewardAmount.get()}
                    onChange={(e) => ctrl.rewardAmount.set(e.target.value)}
                    placeholder={t('reward_amount_placeholder')}
                  />
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
