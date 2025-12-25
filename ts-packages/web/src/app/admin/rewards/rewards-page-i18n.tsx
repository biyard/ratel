import { useTranslation } from 'react-i18next';

export const i18nAdminRewards = {
  en: {
    title: 'Reward Management',
    loading: 'Loading rewards...',
    load_error: 'Failed to load rewards',
    save: 'Save',
    cancel: 'Cancel',
    edit: 'Edit',
    reward_action: 'Reward Action',
    point: 'Points',
    period: 'Period',
    condition: 'Condition',
    no_rewards: 'No rewards configured',
    update_success: 'Reward updated successfully',
    update_error: 'Failed to update reward',
    // Reward actions
    action_none: 'None',
    action_poll_respond: 'Poll Response',
    // Periods
    period_once: 'Once',
    period_hourly: 'Hourly',
    period_daily: 'Daily',
    period_weekly: 'Weekly',
    period_monthly: 'Monthly',
    period_yearly: 'Yearly',
    period_unlimited: 'Unlimited',
    // Conditions
    condition_none: 'None',
    condition_max_claims: 'Max Claims',
    condition_max_points: 'Max Points',
    condition_max_user_claims: 'Max User Claims',
    condition_max_user_points: 'Max User Points',
  },
  ko: {
    title: '리워드 관리',
    loading: '리워드 로딩 중...',
    load_error: '리워드 로딩 실패',
    save: '저장',
    cancel: '취소',
    edit: '수정',
    reward_action: '리워드 액션',
    point: '포인트',
    period: '기간',
    condition: '조건',
    no_rewards: '설정된 리워드가 없습니다',
    update_success: '리워드가 성공적으로 업데이트되었습니다',
    update_error: '리워드 업데이트 실패',
    // Reward actions
    action_none: '없음',
    action_poll_respond: '설문 응답',
    // Periods
    period_once: '1회',
    period_hourly: '시간당',
    period_daily: '일간',
    period_weekly: '주간',
    period_monthly: '월간',
    period_yearly: '연간',
    period_unlimited: '무제한',
    // Conditions
    condition_none: '없음',
    condition_max_claims: '최대 청구 횟수',
    condition_max_points: '최대 포인트',
    condition_max_user_claims: '유저당 최대 청구 횟수',
    condition_max_user_points: '유저당 최대 포인트',
  },
};

export interface AdminRewardsI18n {
  title: string;
  loading: string;
  loadError: string;
  save: string;
  cancel: string;
  edit: string;
  rewardAction: string;
  point: string;
  period: string;
  condition: string;
  noRewards: string;
  updateSuccess: string;
  updateError: string;
  actionNone: string;
  actionPollRespond: string;
  periodOnce: string;
  periodHourly: string;
  periodDaily: string;
  periodWeekly: string;
  periodMonthly: string;
  periodYearly: string;
  periodUnlimited: string;
  conditionNone: string;
  conditionMaxClaims: string;
  conditionMaxPoints: string;
  conditionMaxUserClaims: string;
  conditionMaxUserPoints: string;
}

export function useAdminRewardsI18n(): AdminRewardsI18n {
  const { t } = useTranslation('AdminRewards');

  return {
    title: t('title'),
    loading: t('loading'),
    loadError: t('load_error'),
    save: t('save'),
    cancel: t('cancel'),
    edit: t('edit'),
    rewardAction: t('reward_action'),
    point: t('point'),
    period: t('period'),
    condition: t('condition'),
    noRewards: t('no_rewards'),
    updateSuccess: t('update_success'),
    updateError: t('update_error'),
    actionNone: t('action_none'),
    actionPollRespond: t('action_poll_respond'),
    periodOnce: t('period_once'),
    periodHourly: t('period_hourly'),
    periodDaily: t('period_daily'),
    periodWeekly: t('period_weekly'),
    periodMonthly: t('period_monthly'),
    periodYearly: t('period_yearly'),
    periodUnlimited: t('period_unlimited'),
    conditionNone: t('condition_none'),
    conditionMaxClaims: t('condition_max_claims'),
    conditionMaxPoints: t('condition_max_points'),
    conditionMaxUserClaims: t('condition_max_user_claims'),
    conditionMaxUserPoints: t('condition_max_user_points'),
  };
}
