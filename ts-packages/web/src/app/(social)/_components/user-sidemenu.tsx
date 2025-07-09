'use client';

import React from 'react';
import ProfileSection from './profile-section';
import RecentActivities from './RecentActivities';
import Spaces from './Spaces';
import Saved from './Saved';
import { useUserInfo } from '@/lib/api/hooks/users';
import Link from 'next/link';
import { route } from '@/route';
import { Post, Settings, ChevronLeft, ChevronRight } from '@/components/icons';
import { UserType } from '@/lib/api/models/user';

export default function UserSidemenu({
  isOpen,
  toggleSidebar,
}: {
  isOpen: boolean;
  toggleSidebar: () => void;
}) {
  const { data: user, isLoading } = useUserInfo();
  if (
    isLoading ||
    !user ||
    (user.user_type !== UserType.Individual && user.user_type !== UserType.Team)
  ) {
    return <div />;
  }

  return (
    <div
      className={`transition-all duration-300 ${
        isOpen ? 'w-62.5' : 'w-[100px]'
      } flex flex-col gap-2.5 max-mobile:hidden shrink-0 relative`}
    >
      <button
        onClick={toggleSidebar}
        className="absolute -right-3 top-4 z-10 bg-component-bg border border-neutral-700 rounded-full p-1 hover:bg-neutral-700 transition"
      >
        {isOpen ? (
          <ChevronLeft className="w-4 h-4 text-white" />
        ) : (
          <ChevronRight className="w-4 h-4 text-white" />
        )}
      </button>
      {isOpen && <ProfileSection />}

      <nav
        className={`py-5 px-3 w-full rounded-[10px] bg-component-bg flex flex-col gap-2`}
      >
        <Link href={route.myPosts()} className="sidemenu-link">
          <Post className="w-[24px] h-[24px]" />
          {isOpen && <span>My Posts</span>}
        </Link>
        <Link href={route.drafts()} className="sidemenu-link">
          <Post className="w-[24px] h-[24px]" />
          {isOpen && <span>Drafts</span>}
        </Link>
        <Link href={route.settings()} className="sidemenu-link">
          <Settings className="w-[24px] h-[24px]" />
          {isOpen && <span>Settings</span>}
        </Link>
      </nav>

      {isOpen && (
        <>
          <RecentActivities />
          <Spaces />
          <Saved />
        </>
      )}
    </div>
  );
}
