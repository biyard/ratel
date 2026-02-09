import { useTranslation } from 'react-i18next';

export const i18nAdminRewards = {
  en: {
    title: 'Reward Management',
    loading: 'Loading rewards...',
    load_error: 'Failed to load rewards',
    save: 'Save',
    edit: 'Edit',
    cancel: 'Cancel',
    add_reward: 'Add Reward',
    edit_reward: 'Edit Reward',
    action_label: 'Reward Action',
    reward_behavior: 'Reward Behavior',
    point: 'Points',
    period: 'Period',
    condition: 'Condition',
    no_rewards: 'No rewards configured',
    save_success: 'Reward saved successfully',
    save_error: 'Failed to save reward',
    // Tabs
    tab_rules: 'Reward Rules',
    tab_transactions: 'Point Transactions',
    // Transaction table
    user_id: 'User ID',
    month: 'Month',
    transaction_type: 'Type',
    amount: 'Amount',
    description: 'Description',
    created_at: 'Date',
    no_transactions: 'No transactions found',
    load_more: 'Load More',
    loading_more: 'Loading...',
  },
  ko: {
    title: '리워드 관리',
    loading: '리워드 로딩 중...',
    load_error: '리워드 로딩 실패',
    save: '저장',
    edit: '수정',
    cancel: '취소',
    add_reward: '리워드 추가',
    edit_reward: '리워드 수정',
    action_label: '리워드 액션',
    reward_behavior: '리워드 행위',
    point: '포인트',
    period: '기간',
    condition: '조건',
    no_rewards: '설정된 리워드가 없습니다',
    save_success: '리워드가 성공적으로 저장되었습니다',
    save_error: '리워드 저장 실패',
    // Tabs
    tab_rules: '리워드 규칙',
    tab_transactions: '포인트 지급 내역',
    // Transaction table
    user_id: '유저 ID',
    month: '월',
    transaction_type: '유형',
    amount: '금액',
    description: '설명',
    created_at: '일시',
    no_transactions: '지급 내역이 없습니다',
    load_more: '더 보기',
    loading_more: '로딩 중...',
  },
};

export interface AdminRewardsI18n {
  title: string;
  loading: string;
  loadError: string;
  save: string;
  edit: string;
  cancel: string;
  addReward: string;
  editReward: string;
  actionLabel: string;
  rewardAction: string;
  point: string;
  period: string;
  condition: string;
  noRewards: string;
  saveSuccess: string;
  saveError: string;
  tabRules: string;
  tabTransactions: string;
  userId: string;
  month: string;
  transactionType: string;
  amount: string;
  description: string;
  createdAt: string;
  noTransactions: string;
  loadMore: string;
  loadingMore: string;
}

export function useAdminRewardsI18n(): AdminRewardsI18n {
  const { t } = useTranslation('AdminRewards');

  return {
    title: t('title'),
    loading: t('loading'),
    loadError: t('load_error'),
    save: t('save'),
    edit: t('edit'),
    cancel: t('cancel'),
    addReward: t('add_reward'),
    editReward: t('edit_reward'),
    actionLabel: t('action_label'),
    rewardAction: t('reward_behavior'),
    saveSuccess: t('save_success'),
    saveError: t('save_error'),
    point: t('point'),
    period: t('period'),
    condition: t('condition'),
    noRewards: t('no_rewards'),
    tabRules: t('tab_rules'),
    tabTransactions: t('tab_transactions'),
    userId: t('user_id'),
    month: t('month'),
    transactionType: t('transaction_type'),
    amount: t('amount'),
    description: t('description'),
    createdAt: t('created_at'),
    noTransactions: t('no_transactions'),
    loadMore: t('load_more'),
    loadingMore: t('loading_more'),
  };
}
