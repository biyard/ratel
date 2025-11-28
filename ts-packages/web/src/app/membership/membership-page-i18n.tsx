import { useTranslation } from 'react-i18next';

export const MembershipPage = {
  en: {
    title: 'Membership',
    current_plan: 'Current Plan',
    tier: 'Tier',
    total_credits: 'Total Credits',
    remaining_credits: 'Remaining Credits',
    expiration: 'Expires',
    next_membership: 'Next Membership',
    scheduled_downgrade: 'Scheduled Downgrade',
    purchase_history: 'Purchase History',
    transaction_type: 'Type',
    amount: 'Amount',
    payment_id: 'Payment ID',
    date: 'Date',
    no_purchases: 'No purchase history',
    unlimited: 'Unlimited',
  },
  ko: {
    title: '멤버십',
    current_plan: '현재 플랜',
    tier: '등급',
    total_credits: '총 크레딧',
    remaining_credits: '남은 크레딧',
    expiration: '만료일',
    next_membership: '다음 멤버십',
    scheduled_downgrade: '예정된 다운그레이드',
    purchase_history: '구매 내역',
    transaction_type: '유형',
    amount: '금액',
    payment_id: '결제 ID',
    date: '날짜',
    no_purchases: '구매 내역이 없습니다',
    unlimited: '무제한',
  },
};

export interface MembershipPageI18n {
  title: string;
  current_plan: string;
  tier: string;
  total_credits: string;
  remaining_credits: string;
  expiration: string;
  next_membership: string;
  scheduled_downgrade: string;
  purchase_history: string;
  transaction_type: string;
  amount: string;
  payment_id: string;
  date: string;
  no_purchases: string;
  unlimited: string;
}

export function useMembershipPageI18n(): MembershipPageI18n {
  const { t } = useTranslation('MembershipPage');

  return {
    title: t('title'),
    current_plan: t('current_plan'),
    tier: t('tier'),
    total_credits: t('total_credits'),
    remaining_credits: t('remaining_credits'),
    expiration: t('expiration'),
    next_membership: t('next_membership'),
    scheduled_downgrade: t('scheduled_downgrade'),
    purchase_history: t('purchase_history'),
    transaction_type: t('transaction_type'),
    amount: t('amount'),
    payment_id: t('payment_id'),
    date: t('date'),
    no_purchases: t('no_purchases'),
    unlimited: t('unlimited'),
  };
}
