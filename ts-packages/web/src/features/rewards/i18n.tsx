import { useTranslation } from 'react-i18next';
import { RewardsI18n } from './types';

// Common i18n data for rewards components
export const rewardsCommonI18n = {
  en: {
    exchange_from: 'from',
    exchange_to: 'To',
    point: 'Point',
    token: 'Token',
    received: 'Received',
    spent: 'Spent',
    from: 'from',
    empty: 'No transactions',
    loading: 'Loading...',
    error: 'Error loading rewards',
    load_more: 'Load more',
  },
  ko: {
    exchange_from: 'from',
    exchange_to: 'To',
    point: 'Point',
    token: 'Token',
    received: '획득',
    spent: '사용',
    from: 'from',
    empty: '거래 내역 없음',
    loading: '로딩 중...',
    error: '리워드 로딩 오류',
    load_more: '더 보기',
  },
};

// Hook to get common rewards i18n with custom overrides
export function useRewardsI18n(
  namespace: string,
  overrides?: Partial<RewardsI18n>,
): RewardsI18n {
  const { t } = useTranslation(namespace);

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
    ...overrides,
  };
}
