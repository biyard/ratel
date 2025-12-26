import { useTranslation } from 'react-i18next';

export const i18nRewardsPage = {
  en: {
    title: "This month's points",
    your_share: 'Your share',
    this_months_pool: "This month's pool",
    exchange_from: 'from',
    exchange_to: 'To',
    point: 'Point',
    token: 'Token',
    swap_available_message:
      'Point-to-Token Swap will be available starting next month',
    received: 'Received',
    spent: 'Spent',
    from: 'from',
    empty: 'No transactions',
    empty_description: 'You have no point transactions yet',
    loading: 'Loading...',
    error: 'Error loading rewards',
    load_more: 'Load more',
  },
  ko: {
    title: '이번 달 포인트',
    your_share: '내 지분',
    this_months_pool: '이번 달 풀',
    exchange_from: 'from',
    exchange_to: 'To',
    point: 'Point',
    token: 'Token',
    swap_available_message: '포인트-토큰 스왑은 다음 달부터 가능합니다',
    received: '획득',
    spent: '사용',
    from: 'from',
    empty: '거래 내역 없음',
    empty_description: '아직 포인트 거래 내역이 없습니다',
    loading: '로딩 중...',
    error: '리워드 로딩 오류',
    load_more: '더 보기',
  },
};

export interface RewardsI18n {
  title: string;
  your_share: string;
  this_months_pool: string;
  exchange_from: string;
  exchange_to: string;
  point: string;
  token: string;
  swap_available_message: string;
  received: string;
  spent: string;
  from: string;
  empty: string;
  empty_description: string;
  loading: string;
  error: string;
  load_more: string;
}

export function useMyRewardsI18n(): RewardsI18n {
  const { t } = useTranslation('MyRewards');

  return {
    title: t('title'),
    your_share: t('your_share'),
    this_months_pool: t('this_months_pool'),
    exchange_from: t('exchange_from'),
    exchange_to: t('exchange_to'),
    point: t('point'),
    token: t('token'),
    swap_available_message: t('swap_available_message'),
    received: t('received'),
    spent: t('spent'),
    from: t('from'),
    empty: t('empty'),
    empty_description: t('empty_description'),
    loading: t('loading'),
    error: t('error'),
    load_more: t('load_more'),
  };
}
