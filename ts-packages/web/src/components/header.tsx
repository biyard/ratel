'use client';
import React from 'react';

import Logo from '@/assets/icons/logo.svg';
import HomeIcon from '@/assets/icons/home.svg';
import UserGroupIcon from '@/assets/icons/user-group.svg';
import Hamburger from '@/assets/icons/hamburger.svg';
import CloseIcon from '@/assets/icons/remove.svg';
import Link from 'next/link';
import Profile from './profile';
import { LoginModal } from './popup/login-popup';
import { usePopup } from '@/lib/contexts/popup-service';
import { route } from '@/route';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { UserType } from '@/lib/api/models/user';
import LoginIcon from '@/assets/icons/login.svg';
import { useLocale, useTranslations } from 'next-intl';
import { Us } from './icons';
import { Kr } from '@/assets/icons/flags';
export interface HeaderProps {
  mobileExtends: boolean;
  setMobileExtends: (extend: boolean) => void;
}

function Header(props: HeaderProps) {
  const t = useTranslations('Nav');
  const popup = usePopup();
  const locale = useLocale() as 'en' | 'ko';

  const { data } = useSuspenseUserInfo();
  const loggedIn =
    data &&
    (data.user_type === UserType.Individual ||
      data.user_type === UserType.Team);

  const navItems = [
    {
      name: t('home'),
      icon: (
        <HomeIcon
          className="group-hover:[&>path]:stroke-white transition-all"
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
          className="group-hover:[&>path]:stroke-white transition-all"
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
    <header className="border-b border-neutral-800 px-2.5 py-2.5 flex items-center justify-center !bg-bg h-[var(--header-height)]">
      <nav className="flex items-center justify-between mx-2.5 gap-12.5 w-full max-w-desktop">
        <div className="flex items-center gap-5">
          <Link
            href={route.home()}
            onClick={() => {
              props.setMobileExtends(false);
            }}
          >
            <Logo className="mobile:size-12 size-13.5" />
          </Link>
        </div>

        <div className="flex items-center gap-2.5 max-tablet:hidden">
          {navItems.map((item, index) => (
            <Link
              key={`nav-item-${index}`}
              href={item.href}
              className="flex flex-col items-center justify-center group p-2.5"
              hidden={!item.visible || (item.authorized && !loggedIn)}
            >
              {item.icon}
              <span className="whitespace-nowrap text-neutral-500 group-hover:text-white text-[15px] font-medium transition-all">
                {' '}
                {item.name}{' '}
              </span>
            </Link>
          ))}

          {data &&
          (data.user_type === UserType.Individual ||
            data?.user_type === UserType.Team) ? (
            <Profile profileUrl={data.profile_url} name={data.nickname} />
          ) : (
            <button
              className="group cursor-pointer font-bold text-neutral-500 text-[15px] flex flex-col items-center justify-center group p-2.5"
              onClick={() => {
                popup
                  .open(<LoginModal />)
                  .withTitle('Join the Movement')
                  .withoutBackdropClose();
              }}
            >
              <LoginIcon className="size-6 group-hover:[&>path]:stroke-white" />
              <span className="whitespace-nowrap text-neutral-500 group-hover:text-white text-[15px] font-medium transition-all">
                {t('signIn')}
              </span>
            </button>
          )}

          {locale == 'en' ? (
            <Us className="cursor-pointer rounded-full w-8 h-8" />
          ) : (
            <Kr className="cursor-pointer rounded-full w-8 h-8" />
          )}
        </div>

        <div
          className="hidden max-tablet:block cursor-pointer"
          onClick={() => props.setMobileExtends(!props.mobileExtends)}
        >
          {props.mobileExtends ? (
            <CloseIcon className="transition-all" />
          ) : (
            <Hamburger className="transition-all" />
          )}
        </div>
      </nav>
    </header>
  );
}

export default Header;
