import { useTranslation } from 'react-i18next';

export const i18nAdmin = {
  en: {
    title: 'Admin Console',
    memberships: 'Memberships',
    memberships_desc: 'Manage membership tiers, pricing, and credits',
    attribute_codes: 'Attribute Codes',
    attribute_codes_desc: 'Manage verification codes for user attributes',
    users: 'User Management',
    users_desc: 'Manage user accounts, roles, and permissions',
    rewards: 'Rewards',
    rewards_desc: 'Manage reward points and conditions',
    migrations: 'Migrations',
    migrations_desc: 'Run data migrations',
    payments: 'Payment History',
    payments_desc: 'User payment history, refund management',
  },
  ko: {
    title: '관리자 콘솔',
    memberships: '멤버십',
    memberships_desc: '멤버십 등급, 가격 및 크레딧 관리',
    attribute_codes: '속성 코드',
    attribute_codes_desc: '사용자 속성 검증 코드 관리',
    users: '사용자 관리',
    users_desc: '사용자 계정, 역할 및 권한 관리',
    rewards: '리워드',
    rewards_desc: '리워드 포인트 및 조건 관리',
    migrations: '마이그레이션',
    migrations_desc: '데이터 마이그레이션',
    payments: '결제 내역',
    payments_desc: '사용자 결제 내역, 환불 관리',
  },
};

export interface AdminI18n {
  title: string;
  memberships: string;
  membershipsDesc: string;
  attributeCodes: string;
  attributeCodesDesc: string;
  users: string;
  usersDesc: string;
  rewards: string;
  rewardsDesc: string;
  migrations: string;
  migrationsDesc: string;
  payments: string;
  paymentsDesc: string;
}

export function useAdminI18n(): AdminI18n {
  const { t } = useTranslation('Admin');

  return {
    title: t('title'),
    memberships: t('memberships'),
    membershipsDesc: t('memberships_desc'),
    attributeCodes: t('attribute_codes'),
    attributeCodesDesc: t('attribute_codes_desc'),
    users: t('users'),
    usersDesc: t('users_desc'),
    rewards: t('rewards'),
    rewardsDesc: t('rewards_desc'),
    migrations: t('migrations'),
    migrationsDesc: t('migrations_desc'),
    payments: t('payments'),
    paymentsDesc: t('payments_desc'),
  };
}
