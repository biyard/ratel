'use client';
import React from 'react';
import ProfileSection from './profile-section';

import Link from 'next/link';
import { route } from '@/route';
import { Post, Draft, Settings } from '@/components/icons';
import { UserType } from '@/lib/api/models/user';
import { useTranslations } from 'next-intl';
import { useUserInfo } from '@/lib/api/hooks/users';
// import DevTools from './dev-tools';

export default function UserSidemenu() {
  const t = useTranslations('Home');
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
      <nav className="py-5 px-3 w-full rounded-[10px] bg-card-bg border border-card-border text-text-primary">
        <Link
          href={route.myPosts()}
          className="sidemenu-link text-text-primary"
        >
          <Post className="w-[24px] h-[24px]" />
          <span>{t('my_posts')}</span>
        </Link>
        <Link href={route.drafts()} className="sidemenu-link text-text-primary">
          <Draft className="w-[24px] h-[24px]" />
          <span>{t('drafts')}</span>
        </Link>
        <Link
          href={route.settings()}
          className="sidemenu-link text-text-primary"
        >
          <Settings className="w-[24px] h-[24px]" />
          <span>{t('settings')}</span>
        </Link>
      </nav>

      {/* <DevTools /> */}
    </div>
  );
}
