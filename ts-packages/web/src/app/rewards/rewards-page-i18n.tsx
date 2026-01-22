import { RewardsI18n } from '@/features/rewards/types';
import { rewardsCommonI18n, useRewardsI18n } from '@/features/rewards/i18n';

export const i18nRewardsPage = {
  en: {
    ...rewardsCommonI18n.en,
    title: "This month's points",
    your_share: 'Your share',
    this_months_pool: "This month's pool",
    swap_available_message:
      'Point-to-Token Swap will be available starting next month',
    empty_description: 'You have no point transactions yet',
  },
  ko: {
    ...rewardsCommonI18n.ko,
    title: '이번 달 포인트',
    your_share: '내 지분',
    this_months_pool: '이번 달 풀',
    swap_available_message: '포인트-토큰 스왑은 다음 달부터 가능합니다',
    empty_description: '아직 포인트 거래 내역이 없습니다',
  },
};

export function useMyRewardsI18n(): RewardsI18n {
  return useRewardsI18n('MyRewards');
}
