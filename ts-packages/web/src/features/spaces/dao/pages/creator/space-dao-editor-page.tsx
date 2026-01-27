import { useTranslation } from 'react-i18next';
import { SpacePathProps } from '@/features/space-path-props';
import { Input } from '@/components/ui/input';
import { logger } from '@/lib/logger';
import { useSpaceDaoEditorController } from './space-dao-editor-controller';
import { RegisterDaoPopup } from '@/features/teams/dao/components/register-dao-popup';
import Card from '@/components/card';

export function SpaceDaoEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceDaoEditorPage: spacePk=${spacePk}`);
  const ctrl = useSpaceDaoEditorController(spacePk);
  const { t } = useTranslation('SpaceDaoEditor');

  if (!ctrl.space) {
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

          <div className="grid gap-5">
            <div>
              <label className="text-sm text-text-secondary mb-2 block">
                {t('sampling_count_label')}
              </label>
              <Input
                type="number"
                min={0}
                value={ctrl.samplingCount}
                onChange={(e) => ctrl.setSamplingCount(e.target.value)}
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
                value={ctrl.rewardAmount}
                onChange={(e) => ctrl.setRewardAmount(e.target.value)}
                placeholder={t('reward_amount_placeholder')}
              />
            </div>
          </div>

          <button
            onClick={ctrl.handleOpenRegistrationPopup}
            disabled={
              !ctrl.canRegisterDao || !ctrl.isTeamSpace || !ctrl.canSubmitInputs
            }
            className="mt-6 w-full px-6 py-3 bg-primary text-white rounded-md font-medium hover:bg-primary-dark transition-colors disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-primary"
          >
            {t('register_dao')}
          </button>
        </div>

        {ctrl.isPopupOpen && (
          <RegisterDaoPopup
            eligibleAdmins={ctrl.eligibleAdmins}
            onRegister={ctrl.handleRegisterDao}
            onCancel={ctrl.handleClosePopup}
            isRegistering={ctrl.isRegistering}
          />
        )}
      </Card>
    </div>
  );
}
