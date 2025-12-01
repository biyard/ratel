import { useTranslation } from 'react-i18next';

export const rewardsI18n = {
  en: {
    reward_side_menu_title: 'Reward',
    reward_side_menu_boosting: 'Boosting',
    reward_side_menu_total_estimated_value: 'Total estimated Value',
  },
  ko: {
    reward_side_menu_title: '리워드',
    reward_side_menu_boosting: '부스팅',
    reward_side_menu_total_estimated_value: '총 예상 가치',
  },
};

export interface RewardsI18n {
  rewardSideMenuTitle: string;
  rewardSideMenuBoosting: string;
  rewardSideMenuTotalEstimatedValue: string;
}

export function useRewardsI18n(): RewardsI18n {
  const { t } = useTranslation('Rewards');
  return {
    rewardSideMenuTitle: t('reward_side_menu_title'),
    rewardSideMenuBoosting: t('reward_side_menu_boosting'),
    rewardSideMenuTotalEstimatedValue: t(
      'reward_side_menu_total_estimated_value',
    ),
  };
}
