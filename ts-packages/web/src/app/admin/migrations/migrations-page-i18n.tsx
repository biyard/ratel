import { useTranslation } from 'react-i18next';

export const i18nAdminMigrations = {
  en: {
    title: 'Migrations',
    description: 'Run data migrations for backend maintenance.',
    run_teams: 'Run Teams Migration',
    running: 'Running...',
    success: 'Migration completed successfully.',
    failed: 'Migration failed.',
  },
  ko: {
    title: '마이그레이션',
    description: '백엔드 데이터 마이그레이션을 실행합니다.',
    run_teams: '팀 마이그레이션 실행',
    running: '실행 중...',
    success: '마이그레이션이 완료되었습니다.',
    failed: '마이그레이션에 실패했습니다.',
  },
};

export interface AdminMigrationsI18n {
  title: string;
  description: string;
  runTeams: string;
  running: string;
  success: string;
  failed: string;
}

export function useAdminMigrationsI18n(): AdminMigrationsI18n {
  const { t } = useTranslation('AdminMigrations');

  return {
    title: t('title'),
    description: t('description'),
    runTeams: t('run_teams'),
    running: t('running'),
    success: t('success'),
    failed: t('failed'),
  };
}
