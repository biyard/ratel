import { useTranslation } from 'react-i18next';

export const SpaceSettings = {
  en: {
    title: 'Space Settings',
    loading: 'Loading...',
    error: 'Failed to load space settings',
    participation_title: 'Participation',
    anonymous_participation_label: 'Allow Anonymous Participation',
    anonymous_participation_description:
      'When enabled, users can participate in this space without revealing their identity',
    success_update: 'Successfully updated anonymous participation setting',
    error_update: 'Failed to update anonymous participation',
  },
  ko: {
    title: '스페이스 설정',
    loading: '로딩 중...',
    error: '스페이스 설정을 불러오는데 실패했습니다',
    participation_title: '참여 설정',
    anonymous_participation_label: '익명 참여 허용',
    anonymous_participation_description:
      '활성화하면 사용자가 신원을 밝히지 않고 이 스페이스에 참여할 수 있습니다',
    success_update: '익명 참여 설정이 성공적으로 업데이트되었습니다',
    error_update: '익명 참여 설정 업데이트에 실패했습니다',
  },
};

export interface SettingsI18n {
  title: string;
  loading: string;
  error: string;
  participation_title: string;
  anonymous_participation_label: string;
  anonymous_participation_description: string;
  success_update: string;
  error_update: string;
}

export function useSettingsI18n(): SettingsI18n {
  const { t } = useTranslation('SpaceSettings');

  return {
    title: t('title'),
    loading: t('loading'),
    error: t('error'),
    participation_title: t('participation_title'),
    anonymous_participation_label: t('anonymous_participation_label'),
    anonymous_participation_description: t(
      'anonymous_participation_description',
    ),
    success_update: t('success_update'),
    error_update: t('error_update'),
  };
}
