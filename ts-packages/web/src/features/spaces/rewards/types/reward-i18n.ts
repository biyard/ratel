import { useTranslation } from 'react-i18next';
import { RewardUserBehavior } from './reward-user-behavior';
import { RewardPeriod } from './reward-period';
import { ConditionType } from './reward-condition';

export const i18nRewardTypes = {
  en: {
    behavior_respond_poll: 'Poll Response',
    behavior_unknown: 'None',
    period_once: 'Once',
    period_hourly: 'Hourly',
    period_daily: 'Daily',
    period_weekly: 'Weekly',
    period_monthly: 'Monthly',
    period_yearly: 'Yearly',
    period_unlimited: 'Unlimited',
    condition_none: 'None',
    condition_max_claims: 'Max Claims',
    condition_max_points: 'Max Points',
    condition_max_user_claims: 'Max User Claims',
    condition_max_user_points: 'Max User Points',
  },
  ko: {
    behavior_respond_poll: '설문 응답',
    behavior_unknown: '없음',
    period_once: '1회',
    period_hourly: '시간당',
    period_daily: '일간',
    period_weekly: '주간',
    period_monthly: '월간',
    period_yearly: '연간',
    period_unlimited: '무제한',
    condition_none: '없음',
    condition_max_claims: '최대 청구 횟수',
    condition_max_points: '최대 포인트',
    condition_max_user_claims: '유저당 최대 청구 횟수',
    condition_max_user_points: '유저당 최대 포인트',
  },
};

export function useRewardBehaviorLabel(): (
  behavior: RewardUserBehavior,
) => string {
  const { t } = useTranslation('RewardTypes');
  return (behavior: RewardUserBehavior) => {
    switch (behavior) {
      case RewardUserBehavior.RespondPoll:
        return t('behavior_respond_poll');
      default:
        return t('behavior_unknown');
    }
  };
}

export function useRewardPeriodLabel(): (period: RewardPeriod) => string {
  const { t } = useTranslation('RewardTypes');
  return (period: RewardPeriod) => {
    switch (period) {
      case RewardPeriod.Once:
        return t('period_once');
      case RewardPeriod.Hourly:
        return t('period_hourly');
      case RewardPeriod.Daily:
        return t('period_daily');
      case RewardPeriod.Weekly:
        return t('period_weekly');
      case RewardPeriod.Monthly:
        return t('period_monthly');
      case RewardPeriod.Yearly:
        return t('period_yearly');
      case RewardPeriod.Unlimited:
        return t('period_unlimited');
      default:
        return period;
    }
  };
}

export function useRewardConditionLabel(): (
  conditionType: ConditionType,
) => string {
  const { t } = useTranslation('RewardTypes');
  return (conditionType: ConditionType) => {
    switch (conditionType) {
      case ConditionType.None:
        return t('condition_none');
      case ConditionType.MaxClaims:
        return t('condition_max_claims');
      case ConditionType.MaxPoints:
        return t('condition_max_points');
      case ConditionType.MaxUserClaims:
        return t('condition_max_user_claims');
      case ConditionType.MaxUserPoints:
        return t('condition_max_user_points');
      default:
        return conditionType;
    }
  };
}
