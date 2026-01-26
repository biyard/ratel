'use client';

import { Suspense } from 'react';
import { useTranslation } from 'react-i18next';
import { useDaoPageController } from '@/features/teams/dao/hooks/use-dao-page-controller';
import { DaoInfoCard } from '@/features/teams/dao/components/dao-info-card';
import { DaoRegistrationCard } from '@/features/teams/dao/components/dao-registration-card';
import { RegisterDaoPopup } from '@/features/teams/dao/components/register-dao-popup';

interface DaoPageProps {
  username: string;
}

function DaoPageContent({ username }: DaoPageProps) {
  const { t } = useTranslation('TeamDao');
  const ctrl = useDaoPageController(username);

  if (!ctrl.permissions?.isAdmin()) {
    return (
      <div className="flex flex-col w-full max-w-[1152px] items-center justify-center min-h-[400px]">
        <div className="text-center">
          <h2 className="text-2xl font-bold text-text-primary mb-2">
            {t('admin_only')}
          </h2>
          <p className="text-text-secondary">
            You must be a team admin to access this page.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col w-full max-w-[1152px] gap-5 p-6">
      <div className="mb-4">
        <h1 className="text-3xl font-bold text-text-primary mb-2">
          {t('dao_title')}
        </h1>
        <p className="text-text-secondary">{t('dao_description')}</p>
      </div>

      {ctrl.team.dao_address ? (
        <DaoInfoCard
          daoAddress={ctrl.team.dao_address}
          explorerUrl={ctrl.blockExplorerUrl}
        />
      ) : (
        <DaoRegistrationCard
          onRegister={ctrl.handleOpenRegistrationPopup}
          eligibleCount={ctrl.eligibleAdminsCount}
          minRequired={3}
          canRegister={ctrl.canRegisterDao}
        />
      )}

      {ctrl.isPopupOpen && (
        <RegisterDaoPopup
          eligibleAdmins={ctrl.eligibleAdmins}
          onRegister={ctrl.handleRegisterDao}
          onCancel={ctrl.handleClosePopup}
          isRegistering={ctrl.isRegistering}
        />
      )}
    </div>
  );
}

export default function DaoPage({ username }: DaoPageProps) {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <DaoPageContent username={username} />
    </Suspense>
  );
}
