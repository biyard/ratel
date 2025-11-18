import ProfileSection from '../../../../app/(social)/_components/profile-section';

import { route } from '@/route';
import { Post, Draft, Settings, Did, UserGroup } from '@/components/icons';
import { useLocation } from 'react-router';
import { useUserInfo } from '@/hooks/use-user-info';
import { NavLink } from 'react-router';
import { UserType } from '@/lib/api/ratel/users.v3';
import { useUserSidemenuI18n } from './i18n';
// import DevTools from './dev-tools';

export default function UserSidemenu() {
  const t = useUserSidemenuI18n();
  const { data: user, isLoading } = useUserInfo();
  const location = useLocation();
  const pathname = location.pathname;

  if (pathname.includes('/subscribe')) {
    return <div />;
  }

  if (
    isLoading ||
    !user ||
    (user.user_type !== UserType.Individual &&
      user.user_type !== UserType.Team &&
      user.user_type !== UserType.Admin)
  ) {
    return <div />;
  }

  return (
    <div className="flex flex-col gap-2.5 w-62.5 max-mobile:hidden shrink-0">
      <ProfileSection />

      {/* Navigation */}
      <nav className="py-5 px-3 w-full border rounded-[10px] bg-card-bg border-card-border text-text-primary">
        <NavLink
          to={route.myPosts()}
          className="sidemenu-link text-text-primary"
          data-testid="sidemenu-my-posts"
        >
          <Post className="w-6 h-6" />
          <span>{t.my_posts}</span>
        </NavLink>

        <NavLink
          to={route.drafts()}
          className="sidemenu-link text-text-primary"
          data-testid="sidemenu-drafts"
        >
          <Draft className="w-6 h-6" />
          <span>{t.drafts}</span>
        </NavLink>

        <NavLink
          to={route.mySpaces()}
          className="sidemenu-link text-text-primary"
          data-testid="sidemenu-my-spaces"
        >
          <UserGroup className="w-6 h-6" />
          <span>{t.my_spaces}</span>
        </NavLink>

        <NavLink
          to={route.credentials()}
          className="sidemenu-link text-text-primary"
          data-testid="sidemenu-credentials"
        >
          <Did className="w-6 h-6" />
          <span>{t.credentials}</span>
        </NavLink>

        <NavLink
          to={route.settings()}
          className="sidemenu-link text-text-primary"
          data-testid="sidemenu-settings"
        >
          <Settings className="w-6 h-6" />
          <span>{t.settings}</span>
        </NavLink>
      </nav>

      {/* <DevTools /> */}
    </div>
  );
}
