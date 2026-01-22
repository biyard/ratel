'use client';

import { useTranslation } from 'react-i18next';
import { InformationCircleIcon } from '@heroicons/react/24/outline';

interface DaoRegistrationCardProps {
  onRegister: () => void;
  eligibleCount: number;
  minRequired: number;
  canRegister: boolean;
}

export function DaoRegistrationCard({
  onRegister,
  eligibleCount,
  minRequired,
  canRegister,
}: DaoRegistrationCardProps) {
  const { t } = useTranslation('TeamDao');

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700">
      <h3 className="text-xl font-semibold text-text-primary mb-4">
        {t('register_dao')}
      </h3>

      <div className="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-md p-4 mb-6">
        <div className="flex gap-3">
          <InformationCircleIcon className="w-6 h-6 text-blue-600 dark:text-blue-400 shrink-0" />
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
            {t('eligible_admins_count', { count: eligibleCount })}
          </p>
          <p className="text-xs text-text-tertiary">
            {t('min_admins_required')}
          </p>
        </div>

        {eligibleCount >= minRequired ? (
          <div className="px-3 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded-full text-sm font-medium">
            ✓ Ready
          </div>
        ) : (
          <div className="px-3 py-1 bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 rounded-full text-sm font-medium">
            Need {minRequired - eligibleCount} more
          </div>
        )}
      </div>

      {!canRegister && eligibleCount < minRequired && (
        <div className="mb-4 p-3 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-md">
          <p className="text-sm text-yellow-800 dark:text-yellow-200">
            {t('insufficient_admins')}
          </p>
        </div>
      )}

      <button
        onClick={onRegister}
        disabled={!canRegister}
        className="w-full px-6 py-3 bg-primary text-white rounded-md font-medium hover:bg-primary-dark transition-colors disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-primary"
      >
        {t('register_dao')}
      </button>
    </div>
  );
}
