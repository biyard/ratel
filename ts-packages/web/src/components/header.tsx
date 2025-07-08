'use client';

import React from 'react';
import Link from 'next/link';

import Logo from '@/assets/icons/logo.svg';
import UserGroupIcon from '@/assets/icons/user-group.svg';
import InternetIcon from '@/assets/icons/internet.svg';
import RoundBubbleIcon from '@/assets/icons/round-bubble.svg';
import BellIcon from '@/assets/icons/bell.svg';
import Hamburger from '@/assets/icons/hamburger.svg';
import CloseIcon from '@/assets/icons/remove.svg';
import LoginIcon from '@/assets/icons/login.svg';

import Profile from './profile';
import { LoginModal } from './popup/login-popup';

import { usePopup } from '@/lib/contexts/popup-service';
import { logger } from '@/lib/logger';
import { route } from '@/route';
import { config } from '@/config';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { UserType } from '@/lib/api/models/user';

export interface HeaderProps {
  mobileExtends: boolean;
  setMobileExtends: (extend: boolean) => void;
}

function Header({ mobileExtends, setMobileExtends }: HeaderProps) {
  const popup = usePopup();
  const { data } = useSuspenseUserInfo();

  const loggedIn = data && data.user_type !== UserType.Individual;
  logger.debug('Header user data:', data);

  const navItems = [
    // {
    //   name: 'Home',
    //   icon: (
    //     <HomeIcon
    //       className="group-hover:[&>path]:stroke-white transition-all"
    //       width="24"
    //       height="24"
    //     />
    //   ),
    //   href: route.home(),
    //   visible: true,
    //   authorized: false,
    // },
    {
      name: 'Explore',
      icon: (
        <InternetIcon
          className="group-hover:[&>path]:stroke-white group-hover:[&>circle]:stroke-white transition-all"
          width="24"
          height="24"
        />
      ),
      href: route.explore(),
      visible: config.experiment,
      authorized: false,
    },
    {
      name: 'My Network',
      icon: (
        <UserGroupIcon
          className="group-hover:[&>path]:stroke-white transition-all"
          width="24"
          height="24"
        />
      ),
      href: route.myNetwork(),
      visible: true,
      authorized: true,
    },
    {
      name: 'Message',
      icon: (
        <RoundBubbleIcon
          className="group-hover:[&>path]:stroke-white transition-all"
          width="24"
          height="24"
        />
      ),
      href: route.messages(),
      visible: config.experiment,
      authorized: true,
    },
    {
      name: 'Notification',
      icon: (
        <BellIcon
          className="group-hover:[&>path]:stroke-white transition-all"
          width="24"
          height="24"
        />
      ),
      href: route.notifications(),
      visible: config.experiment,
      authorized: true,
    },
  ];

  const handleLoginClick = () => {
    popup
      .open(<LoginModal />)
      .withTitle('Join the Movement')
      .withoutBackdropClose();
  };

  return (
    <header className="border-b border-neutral-800 px-2.5 py-2.5 flex items-center justify-center bg-bg h-[var(--header-height)]">
      <nav className="flex items-center justify-between w-full max-w-desktop mx-2.5 gap-8">
        <Link
          href={route.home()}
          onClick={() => setMobileExtends(false)}
          className="flex items-center"
        >
          <Logo width="54" height="54" />
        </Link>

        <div className="hidden max-tablet:flex-1 tablet:flex items-center justify-center gap-6">
          {navItems.map((item, index) =>
            item.visible && (!item.authorized || loggedIn) ? (
              <Link
                key={index}
                href={item.href}
                className="flex flex-col items-center justify-center group p-2.5"
              >
                {item.icon}
                <span className="whitespace-nowrap text-neutral-500 group-hover:text-white text-[15px] font-medium transition-all">
                  {item.name}
                </span>
              </Link>
            ) : null,
          )}
        </div>

        <div className="flex items-center gap-4">
          {data &&
          (data.user_type === UserType.Individual ||
            data.user_type === UserType.Team) ? (
            <Profile profileUrl={data.profile_url} name={data.nickname} />
          ) : (
            <button
              className="group cursor-pointer flex flex-col items-center justify-center"
              onClick={handleLoginClick}
            >
              <LoginIcon className="size-6 group-hover:[&>path]:stroke-white" />
              <span className="text-xs text-neutral-500 group-hover:text-white">
                Sign In
              </span>
            </button>
          )}

          <div
            className="block tablet:hidden cursor-pointer"
            onClick={() => setMobileExtends(!mobileExtends)}
          >
            {mobileExtends ? (
              <CloseIcon className="transition-all" />
            ) : (
              <Hamburger className="transition-all" />
            )}
          </div>
        </div>
      </nav>
    </header>
  );
}

export default Header;
