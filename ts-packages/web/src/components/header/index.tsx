import Logo from '@/assets/icons/logo.svg?react';
import HomeIcon from '@/assets/icons/home.svg?react';
import UserGroupIcon from '@/assets/icons/user-group.svg?react';
import InternetIcon from '@/assets/icons/internet.svg?react';
import Hamburger from '@/assets/icons/hamburger.svg?react';
import CloseIcon from '@/assets/icons/remove.svg?react';
import { NavLink, useLocation } from 'react-router';
import Profile from '../profile';
import { LoginModal } from '../popup/login-popup';
import { usePopup } from '@/lib/contexts/popup-service';
import { route } from '@/route';
import { UserType } from '@/lib/api/ratel/users.v3';
import LoginIcon from '@/assets/icons/login.svg?react';
import { useTranslation } from 'react-i18next';
import { Membership, Us, NotificationBell } from '../icons';
import { Kr } from '@/assets/icons/flags';
import { useUserInfo } from '@/hooks/use-user-info';
import { config, Env } from '@/config';
import { Moon, Sun } from 'lucide-react';
import { useTheme } from '@/hooks/use-theme';
import { useContext, useMemo } from 'react';
import { TeamContext } from '@/lib/contexts/team-context';

export interface HeaderProps {
  mobileExtends: boolean;
  setMobileExtends: (extend: boolean) => void;
}

export default function Header(props: HeaderProps) {
  const { t, i18n } = useTranslation('Nav');
  const popup = usePopup();
  const locale = i18n.language;
  const { data: user } = useUserInfo();
  const loggedIn = user !== null;
  const { theme, setTheme } = useTheme();
  const location = useLocation();
  const { teams } = useContext(TeamContext);

  // Determine current profile image based on page context
  const teamMatch = location.pathname.match(/^\/teams\/([^/]+)/);
  const currentTeamUsername = teamMatch?.[1];
  const currentProfile = useMemo(() => {
    if (currentTeamUsername) {
      // Find the team by username from the URL
      const team = teams.find((t) => t.username === currentTeamUsername);
      if (team) {
        return {
          url: team.profile_url,
          name: team.nickname,
        };
      }
    }
    return {
      url: user?.profile_url || '',
      name: user?.nickname || '',
    };
  }, [currentTeamUsername, teams, user]);

  const handleChangeLanguage = (newLocale: string) => {
    document.cookie = `locale=${newLocale}; path=/; max-age=31536000; samesite=lax`;
    i18n.changeLanguage(newLocale);
  };

  const handleChangeTheme = (newTheme: 'light' | 'dark') => {
    setTheme(newTheme);
    localStorage.setItem('theme', newTheme);
    document.cookie = `theme=${newTheme}; path=/; max-age=31536000; samesite=lax`;
  };

  const isAdmin = user?.user_type === UserType.Admin;

  const navItems = [
    {
      name: t('home'),
      icon: (
        <HomeIcon
          className="[&>path]:stroke-menu-text group-hover:[&>path]:stroke-menu-text/80 transition-all"
          width="24"
          height="24"
        />
      ),
      visible: true,
      href: route.home(),
      authorized: false,
    },
    {
      name: t('my_network'),
      icon: (
        <UserGroupIcon
          className="[&>path]:stroke-menu-text group-hover:[&>path]:stroke-menu-text/80 transition-all"
          width="24"
          height="24"
        />
      ),
      visible: true,
      href: route.myNetwork(),
      authorized: true,
    },
    {
      name: t('notification'),
      icon: (
        <NotificationBell
          className="[&>path]:stroke-menu-text group-hover:[&>path]:stroke-menu-text/80 transition-all"
          width="24"
          height="24"
        />
      ),
      visible: true,
      href: route.notifications(),
      authorized: true,
    },
    {
      name: t('membership'),
      icon: (
        <Membership
          className="[&>path]:stroke-menu-text group-hover:[&>path]:stroke-menu-text/80 transition-all"
          width="24"
          height="24"
        />
      ),
      visible: true,
      href: route.membership(),
      authorized: false,
    },

    {
      name: t('admin'),
      icon: (
        <InternetIcon
          className="[&>path]:stroke-menu-text [&>circle]:stroke-menu-text group-hover:[&>path]:stroke-menu-text/80 group-hover:[&>circle]:stroke-menu-text/80 transition-all"
          width="24"
          height="24"
        />
      ),
      visible: isAdmin,
      href: route.admin(),
      authorized: true,
    },
    {
      name: 'Test Report',
      icon: (
        <InternetIcon
          className="[&>path]:stroke-menu-text [&>circle]:stroke-menu-text group-hover:[&>path]:stroke-menu-text/80 group-hover:[&>circle]:stroke-menu-text/80 transition-all"
          width="24"
          height="24"
        />
      ),
      visible: config.env !== Env.Prod,
      href: '/test-report',
      authorized: false,
    },
    {
      name: 'Storybook',
      icon: (
        <InternetIcon
          className="[&>path]:stroke-menu-text [&>circle]:stroke-menu-text group-hover:[&>path]:stroke-menu-text/80 group-hover:[&>circle]:stroke-menu-text/80 transition-all"
          width="24"
          height="24"
        />
      ),
      visible: config.env !== Env.Prod,
      href: '/storybook',
      authorized: false,
    },
  ];

  const seq: Array<'light' | 'dark'> = ['light', 'dark'];
  const nextTheme = seq[(seq.indexOf(theme) + 1) % seq.length];

  return (
    <header className="border-b border-divider px-2.5 py-2.5 flex items-center justify-center !bg-bg h-[var(--header-height)] z-999">
      <nav className="flex justify-between items-center mx-2.5 w-full gap-12.5 max-w-desktop">
        <div className="flex gap-5 items-center">
          <NavLink
            to={route.home()}
            onClick={() => {
              props.setMobileExtends(false);
            }}
          >
            <Logo className="mobile:size-12 size-13.5" />
          </NavLink>
        </div>

        <div className="flex gap-2.5 justify-center items-center max-tablet:hidden">
          {navItems.map((item, index) => (
            <NavLink
              key={`nav-item-${index}`}
              aria-label={`nav-${item.name}`}
              to={item.href}
              className="flex flex-col justify-center items-center p-2.5 group"
              hidden={!item.visible || (item.authorized && !loggedIn)}
            >
              {item.icon}
              <span className="font-medium whitespace-nowrap transition-all text-menu-text text-[15px] group-hover:text-menu-text/80">
                {item.name}
              </span>
            </NavLink>
          ))}

          <button
            className="flex flex-col justify-center items-center p-2.5 font-bold cursor-pointer group text-menu-text text-[15px] group"
            onClick={() => {
              if (locale == 'en') {
                handleChangeLanguage('ko');
              } else {
                handleChangeLanguage('en');
              }
            }}
          >
            <div
              className="cursor-pointer w-fit h-fit"
              onClick={() => {
                handleChangeLanguage('ko');
              }}
            >
              <div className="flex flex-col justify-center items-center h-6 w-fit">
                {locale == 'en' ? (
                  <Us className="object-cover w-4 h-4 rounded-full cursor-pointer" />
                ) : (
                  <Kr className="object-cover w-4 h-4 rounded-full cursor-pointer" />
                )}
              </div>
              <span className="font-medium whitespace-nowrap transition-all text-menu-text text-[15px] group-hover:text-menu-text/80">
                {locale == 'en' ? 'EN' : 'KO'}
              </span>
            </div>
          </button>

          <button
            className="flex flex-col justify-center items-center mx-2 w-fit"
            aria-label={`Theme: ${theme}`}
            onClick={() => handleChangeTheme(nextTheme)}
          >
            <div className="flex flex-col justify-center items-center h-6 w-fit">
              {theme === 'light' ? (
                <Sun className="[&>path]:stroke-menu-text [&>circle]:stroke-menu-text" />
              ) : (
                <Moon className="[&>path]:stroke-menu-text" />
              )}
            </div>
            <span className="font-medium whitespace-nowrap transition-all text-menu-text text-[15px] group-hover:text-menu-text/80">
              {theme === 'light' ? 'Light' : 'Dark'}
            </span>
          </button>

          {user && loggedIn ? (
            <Profile profileUrl={user.profile_url} name={user.nickname} />
          ) : (
            <button
              className="flex flex-col justify-center items-center p-2.5 font-bold cursor-pointer group text-menu-text text-[15px] group"
              onClick={() => {
                popup
                  .open(<LoginModal />)
                  .withTitle('Join the Movement')
                  .withoutBackdropClose();
              }}
            >
              <LoginIcon className="size-6 group-hover:[&>path]:stroke-menu-text/80" />
              <span className="font-medium whitespace-nowrap transition-all text-menu-text text-[15px] group-hover:text-menu-text/80">
                {t('signIn')}
              </span>
            </button>
          )}
        </div>

        <div
          className="hidden cursor-pointer max-tablet:block"
          onClick={() => props.setMobileExtends(!props.mobileExtends)}
          data-testid="mobile-menu-toggle"
        >
          {props.mobileExtends ? (
            <CloseIcon className="transition-all" />
          ) : user && loggedIn ? (
            <div className="relative" data-testid="mobile-profile-icon">
              <img
                src={currentProfile.url}
                alt={currentProfile.name}
                className="w-10 h-10 rounded-full object-cover border-2 light:border-neutral-900 dark:border-neutral-100"
                onError={(e) => {
                  const img = e.target as HTMLImageElement;
                  img.style.display = 'none';
                  img.nextElementSibling?.classList.remove('hidden');
                }}
              />
              <div className="hidden w-10 h-10 rounded-full bg-neutral-500 border-2 light:border-neutral-900 dark:border-neutral-100" />
            </div>
          ) : (
            <Hamburger className="transition-all light:[&>path]:stroke-text-primary" />
          )}
        </div>
      </nav>
    </header>
  );
}
