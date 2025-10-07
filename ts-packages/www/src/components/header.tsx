import Logo from '@/assets/icons/logo.svg?react';
import HomeIcon from '@/assets/icons/home.svg?react';
import UserGroupIcon from '@/assets/icons/user-group.svg?react';
import Hamburger from '@/assets/icons/hamburger.svg?react';
import CloseIcon from '@/assets/icons/remove.svg?react';
import { NavLink } from 'react-router';
import Profile from './profile';
import { LoginModal } from './popup/login-popup';
import { usePopup } from '@/lib/contexts/popup-service';
import { route } from '@/route';
import { UserType } from '@/lib/api/models/user';
import LoginIcon from '@/assets/icons/login.svg?react';
import { useTranslation } from 'react-i18next';
import { Us } from './icons';
import { Kr } from '@/assets/icons/flags';
import { useUserInfo } from '@/hooks/use-user-info';
export interface HeaderProps {
  mobileExtends: boolean;
  setMobileExtends: (extend: boolean) => void;
}

export default function Header(props: HeaderProps) {
  const { t, i18n } = useTranslation('Nav');
  const popup = usePopup();
  const locale = i18n.language;

  const { data } = useUserInfo();
  const loggedIn =
    data &&
    (data.user_type === UserType.Individual ||
      data.user_type === UserType.Team);

  const handleChangeLanguage = (newLocale: string) => {
    document.cookie = `locale=${newLocale}; path=/; max-age=31536000; samesite=lax`;
    i18n.changeLanguage(newLocale);
  };

  const navItems = [
    {
      name: t('home'),
      icon: (
        <HomeIcon
          className="group-hover:[&>path]:stroke-menu-text/80 transition-all"
          width="24"
          height="24"
        />
      ),
      visible: true,
      href: route.home(),
      authorized: false,
    },
    /* {
     *   name: t('explore'),
     *   icon: (
     *     <InternetIcon
     *       className="group-hover:[&>path]:stroke-white group-hover:[&>circle]:stroke-white transition-all"
     *       width="24"
     *       height="24"
     *     />
     *   ),
     *   visible: config.experiment,
     *   href: route.explore(),
     *   authorized: false,
     * }, */
    {
      name: t('my_network'),
      icon: (
        <UserGroupIcon
          className="group-hover:[&>path]:stroke-menu-text/80 transition-all"
          width="24"
          height="24"
        />
      ),
      visible: true,
      href: route.myNetwork(),
      authorized: true,
    },
    /* {
     *   name: t('message'),
     *   icon: (
     *     <RoundBubbleIcon
     *       className="group-hover:[&>path]:stroke-white transition-all"
     *       width="24"
     *       height="24"
     *     />
     *   ),
     *   visible: config.experiment,
     *   href: route.messages(),
     *   authorized: true,
     * },
     * {
     *   name: t('notification'),
     *   icon: (
     *     <BellIcon
     *       className="group-hover:[&>path]:stroke-white transition-all"
     *       width="24"
     *       height="24"
     *     />
     *   ),
     *   visible: true,
     *   href: route.notifications(),
     *   authorized: true,
     * }, */
  ];

  return (
    <header className="border-b border-divider px-2.5 py-2.5 flex items-center justify-center !bg-bg h-[var(--header-height)] z-999">
      <nav className="flex items-center justify-between mx-2.5 gap-12.5 w-full max-w-desktop">
        <div className="flex items-center gap-5">
          <NavLink
            to={route.home()}
            onClick={() => {
              props.setMobileExtends(false);
            }}
          >
            <Logo className="mobile:size-12 size-13.5" />
          </NavLink>
        </div>

        <div className="flex items-center justify-center gap-2.5 max-tablet:hidden">
          {navItems.map((item, index) => (
            <NavLink
              key={`nav-item-${index}`}
              to={item.href}
              className="flex flex-col items-center justify-center group p-2.5"
              hidden={!item.visible || (item.authorized && !loggedIn)}
            >
              {item.icon}
              <span className="whitespace-nowrap text-menu-text group-hover:text-menu-text/80 text-[15px] font-medium transition-all">
                {' '}
                {item.name}{' '}
              </span>
            </NavLink>
          ))}

          <button
            className="group cursor-pointer font-bold text-menu-text text-[15px] flex flex-col items-center justify-center group p-2.5"
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
              <div className="flex flex-col w-fit justify-center items-center h-6">
                {locale == 'en' ? (
                  <Us className="cursor-pointer rounded-full w-4 h-4 object-cover" />
                ) : (
                  <Kr className="cursor-pointer rounded-full w-4 h-4 object-cover" />
                )}
              </div>
              <span className="whitespace-nowrap text-menu-text group-hover:text-menu-text/80 text-[15px] font-medium transition-all">
                {' '}
                {locale == 'en' ? 'EN' : 'KO'}
              </span>
            </div>
          </button>

          {data &&
          (data.user_type === UserType.Individual ||
            data?.user_type === UserType.Team) ? (
            <Profile profileUrl={data.profile_url} name={data.nickname} />
          ) : (
            <button
              className="group cursor-pointer font-bold text-menu-text text-[15px] flex flex-col items-center justify-center group p-2.5"
              onClick={() => {
                popup
                  .open(<LoginModal />)
                  .withTitle('Join the Movement')
                  .withoutBackdropClose();
              }}
            >
              <LoginIcon className="size-6 group-hover:[&>path]:stroke-menu-text/80" />
              <span className="whitespace-nowrap text-menu-text group-hover:text-menu-text/80 text-[15px] font-medium transition-all">
                {t('signIn')}
              </span>
            </button>
          )}
        </div>

        <div
          className="hidden max-tablet:block cursor-pointer"
          onClick={() => props.setMobileExtends(!props.mobileExtends)}
        >
          {props.mobileExtends ? (
            <CloseIcon className="transition-all" />
          ) : (
            <Hamburger className="transition-all light:[&>path]:stroke-text-primary" />
          )}
        </div>
      </nav>
    </header>
  );
}
