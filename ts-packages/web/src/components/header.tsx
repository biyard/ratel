'use client';
import React from 'react';

import Logo from '@/assets/icons/logo.svg';
import HomeIcon from '@/assets/icons/home.svg';
import UserGroupIcon from '@/assets/icons/user-group.svg';
import InternetIcon from '@/assets/icons/internet.svg';
import RoundBubbleIcon from '@/assets/icons/round-bubble.svg';
import BellIcon from '@/assets/icons/bell.svg';
import Hamburger from '@/assets/icons/hamburger.svg';
import CloseIcon from '@/assets/icons/remove.svg';
import Link from 'next/link';
import Profile from './profile';
import { LoginModal } from './popup/login-popup';
import { usePopup } from '@/lib/contexts/popup-service';
import { logger } from '@/lib/logger';
import { route } from '@/route';
import { config } from '@/config';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { UserType } from '@/lib/api/models/user';
import LoginIcon from '@/assets/icons/login.svg';
export interface HeaderProps {
  mobileExtends: boolean;
  setMobileExtends: (extend: boolean) => void;
}

function Header(props: HeaderProps) {
  const popup = usePopup();

  const { data } = useSuspenseUserInfo();
  const loggedIn = data && data.user_type !== UserType.Individual;

  logger.debug('Header data:', data);

  const navItems = [
    {
      name: 'Home',
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
    {
      name: 'Explore',
      icon: (
        <InternetIcon
          className="group-hover:[&>path]:stroke-white group-hover:[&>circle]:stroke-white transition-all"
          width="24"
          height="24"
        />
      ),
      visible: config.experiment,
      href: route.explore(),
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
      visible: true,
      href: route.myNetwork(),
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
      visible: config.experiment,
      href: route.messages(),
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
      visible: config.experiment,
      href: route.notifications(),
      authorized: true,
    },
  ];

  return (
    <header className="border-b border-neutral-800 px-2.5 py-2.5 flex items-center justify-center !bg-bg">
      <nav className="flex items-center justify-between mx-2.5 gap-12.5 w-full max-w-desktop">
        <div className="flex items-center gap-5">
          <Link
            href={route.home()}
            onClick={() => {
              props.setMobileExtends(false);
            }}
          >
            <Logo width="54" height="54" />
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
                Sign In
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
            <Hamburger className="transition-all" />
          )}
        </div>
      </nav>
    </header>
  );
}

export default Header;
