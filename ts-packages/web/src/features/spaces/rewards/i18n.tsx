import { lang } from '@/i18n/config';
import { useTranslation } from 'react-i18next';

// change Type key: String, value: RewardsI18n
export const rewardsI18n: Record<lang, RewardsI18n> = {
  en: {
    sidemenu: {
      title: 'Reward',
      totalPoint: 'Total Point',
    },
  },
  ko: {
    sidemenu: {
      title: '리워드',
      totalPoint: '총 포인트',
    },
  },
};

export interface RewardsI18n {
  sidemenu: SideMenu;
}

interface SideMenu {
  title: string;
  totalPoint: string;
}

export function useRewardsI18n(): RewardsI18n {
  const { t } = useTranslation('Rewards');
  console.log('t', t('sidemenu.title'));
  return {
    sidemenu: {
      title: t('sidemenu.title'),
      totalPoint: t('sidemenu.totalPoint'),
    },
  };
}
