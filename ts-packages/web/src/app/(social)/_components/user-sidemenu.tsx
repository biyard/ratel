import ProfileSection from './profile-section';

import { route } from '@/route';
import { Post, Draft, Settings } from '@/components/icons';
import { UserType } from '@/lib/api/models/user';
import { useTranslation } from 'react-i18next';
import { useLocation } from 'react-router';
import { useUserInfo } from '@/hooks/use-user-info';
import { NavLink } from 'react-router';
// import DevTools from './dev-tools';

export default function UserSidemenu() {
  const { t } = useTranslation('Home');
  const { data: user, isLoading } = useUserInfo();
  const location = useLocation();
  const pathname = location.pathname;

  if (pathname.includes('/subscribe')) {
    return <div />;
  }

  if (
    isLoading ||
    !user ||
    (user.user_type !== UserType.Individual && user.user_type !== UserType.Team)
  ) {
    return <div />;
  }

  return (
    <div className="w-62.5 flex flex-col gap-2.5 max-mobile:hidden shrink-0">
      <ProfileSection />

      {/* Navigation */}
      <nav className="py-5 px-3 w-full rounded-[10px] bg-card-bg border border-card-border text-text-primary">
        <NavLink
          to={route.myPosts()}
          className="sidemenu-link text-text-primary"
        >
          <Post className="w-[24px] h-[24px]" />
          <span>{t('my_posts')}</span>
        </NavLink>
        <NavLink
          to={route.drafts()}
          className="sidemenu-link text-text-primary"
        >
          <Draft className="w-[24px] h-[24px]" />
          <span>{t('drafts')}</span>
        </NavLink>
        <NavLink
          to={route.settings()}
          className="sidemenu-link text-text-primary"
        >
          <Settings className="w-[24px] h-[24px]" />
          <span>{t('settings')}</span>
        </NavLink>
      </nav>

      {/* <DevTools /> */}
    </div>
  );
}
