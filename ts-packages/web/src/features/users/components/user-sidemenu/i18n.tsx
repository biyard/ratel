import { useTranslation } from 'react-i18next';

export const UserSidemenu = {
  en: {
    my_posts: 'My Posts',
    drafts: 'Drafts',
    credentials: 'Credentials',
    settings: 'Settings',
  },
  ko: {
    my_posts: '내 게시물',
    drafts: '임시글',
    credentials: '자격 증명',
    settings: '설정',
  },
};

export interface UserSidemenuI18n {
  my_posts: string;
  drafts: string;
  settings: string;
  credentials: string;
}

export function useUserSidemenuI18n(): UserSidemenuI18n {
  const { t } = useTranslation('UserSidemenu');

  return {
    my_posts: t('my_posts'),
    drafts: t('drafts'),
    settings: t('settings'),
    credentials: t('credentials'),
  };
}
