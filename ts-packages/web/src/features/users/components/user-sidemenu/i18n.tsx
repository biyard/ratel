import { useTranslation } from 'react-i18next';

export const UserSidemenu = {
  en: {
    my_posts: 'My Posts',
    drafts: 'Drafts',
    my_spaces: 'My Spaces',
    credentials: 'Credentials',
    membership: 'Membership',
    settings: 'Settings',
  },
  ko: {
    my_posts: '내 게시물',
    drafts: '임시글',
    my_spaces: '내 스페이스',
    credentials: '자격 증명',
    membership: '멤버십',
    settings: '설정',
  },
};

export interface UserSidemenuI18n {
  my_posts: string;
  drafts: string;
  my_spaces: string;
  credentials: string;
  membership: string;
  settings: string;
}

export function useUserSidemenuI18n(): UserSidemenuI18n {
  const { t } = useTranslation('UserSidemenu');

  return {
    my_posts: t('my_posts'),
    drafts: t('drafts'),
    my_spaces: t('my_spaces'),
    credentials: t('credentials'),
    membership: t('membership'),
    settings: t('settings'),
  };
}
