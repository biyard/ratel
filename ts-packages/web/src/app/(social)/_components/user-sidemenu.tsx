'use client';
import React from 'react';
import ProfileSection from './profile-section';

import RecentActivities from './RecentActivities';
import Spaces from './Spaces';
import Saved from './Saved';
import { useUserInfo } from '@/lib/api/hooks/users';
import Link from 'next/link';
import { route } from '@/route';
import { Post, Settings } from '@/components/icons';
import { UserType } from '@/lib/api/models/user';
// import DevTools from './dev-tools';

export default function UserSidemenu() {
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
      <nav className="py-5 px-3 w-full rounded-[10px] bg-component-bg">
        <Link href={route.myPosts()} className="sidemenu-link">
          <Post className="w-[24px] h-[24px]" />
          <span>My Posts</span>
        </Link>
        <Link href={route.drafts()} className="sidemenu-link">
          <Post className="w-[24px] h-[24px]" />
          <span>Drafts</span>
        </Link>
        <Link href={route.settings()} className="sidemenu-link">
          <Settings className="w-[24px] h-[24px]" />
          <span>Settings</span>
        </Link>
      </nav>

      {/* <DevTools /> */}

      <RecentActivities />

      <Spaces />

      <Saved />
    </div>
  );
}
