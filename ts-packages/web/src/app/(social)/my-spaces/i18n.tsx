import { useTranslation } from 'react-i18next';

export const MySpaces = {
  en: {
    status: {
      pending: 'Pending',
      participating: 'Participating',
      blocked: 'Expired',
    },
  },
  ko: {
    status: {
      pending: '대기중',
      participating: '참여중',
      blocked: '참여기간 만료',
    },
  },
};

export interface MySpacesI18n {
  status: {
    pending: string;
    participating: string;
    blocked: string;
  };
}

export function useMySpacesI18n(): MySpacesI18n {
  const { t } = useTranslation('MySpaces');

  return {
    status: {
      pending: t('status.pending'),
      participating: t('status.participating'),
      blocked: t('status.blocked'),
    },
  };
}
