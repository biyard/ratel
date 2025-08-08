'use client';
import React from 'react';
import ProfileSection from './profile-section';

import RecentActivities from './RecentActivities';
import Spaces from './Spaces';
import Saved from './Saved';
import { useUserInfo } from '@/lib/api/hooks/users';
import Link from 'next/link';
import { route } from '@/route';
import { Post, Draft, Settings } from '@/components/icons';
import { UserType } from '@/lib/api/models/user';
import { useTheme } from '@/app/_providers/ThemeProvider';
// import DevTools from './dev-tools';

export default function UserSidemenu() {
  const { theme } = useTheme();
  const { data: user, isLoading } = useUserInfo();
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
      <nav
        className={`py-5 px-3 w-full rounded-[10px] ${theme === 'light' ? 'bg-neutral-50 border border-neutral-200' : 'bg-component-bg'}`}
      >
        <Link
          href={route.myPosts()}
          className={`flex flex-row items-center gap-1 px-1 py-2 rounded-md font-bold text-sm ${theme === 'light' ? 'hover:bg-neutral-200' : 'hover:bg-gray-600'}`}
        >
          <Post className="w-[24px] h-[24px]" />
          <span
            className={`${theme === 'light' ? 'text-neutral-800' : 'text-white'}`}
          >
            My Posts
          </span>
        </Link>
        <Link
          href={route.drafts()}
          className={`flex flex-row items-center gap-1 px-1 py-2 rounded-md font-bold text-sm ${theme === 'light' ? 'hover:bg-neutral-200' : 'hover:bg-gray-600'}`}
        >
          <Draft className="w-[24px] h-[24px]" />
          <span
            className={`${theme === 'light' ? 'text-neutral-800' : 'text-white'}`}
          >
            Drafts
          </span>
        </Link>
        <Link
          href={route.settings()}
          className={`flex flex-row items-center gap-1 px-1 py-2 rounded-md font-bold text-sm ${theme === 'light' ? 'hover:bg-neutral-200' : 'hover:bg-gray-600'}`}
        >
          <Settings className="w-[24px] h-[24px]" />
          <span
            className={`${theme === 'light' ? 'text-neutral-800' : 'text-white'}`}
          >
            Settings
          </span>
        </Link>
      </nav>

      {/* <DevTools /> */}

      <RecentActivities />

      <Spaces />

      <Saved />
    </div>
  );
}
