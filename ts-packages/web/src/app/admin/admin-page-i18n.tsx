import { useTranslation } from 'react-i18next';

export const i18nAdmin = {
  en: {
    title: 'Admin Console',
    memberships: 'Memberships',
    memberships_desc: 'Manage membership tiers, pricing, and credits',
    attribute_codes: 'Attribute Codes',
    attribute_codes_desc: 'Manage verification codes for user attributes',
  },
  ko: {
    title: '관리자 콘솔',
    memberships: '멤버십',
    memberships_desc: '멤버십 등급, 가격 및 크레딧 관리',
    attribute_codes: '속성 코드',
    attribute_codes_desc: '사용자 속성 검증 코드 관리',
  },
};

export interface AdminI18n {
  title: string;
  memberships: string;
  membershipsDesc: string;
  attributeCodes: string;
  attributeCodesDesc: string;
}

export function useAdminI18n(): AdminI18n {
  const { t } = useTranslation('Admin');

  return {
    title: t('title'),
    memberships: t('memberships'),
    membershipsDesc: t('memberships_desc'),
    attributeCodes: t('attribute_codes'),
    attributeCodesDesc: t('attribute_codes_desc'),
  };
}
