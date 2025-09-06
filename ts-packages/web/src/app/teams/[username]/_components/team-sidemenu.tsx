'use client';
import React, { useContext, useMemo } from 'react';
// import { Users, MessageSquare } from 'lucide-react';
// import { Team } from '@/lib/api/models/team';
import TeamProfile from './team-profile';
import Link from 'next/link';
import { route } from '@/route';
import {
  Home,
  UserGroup,
  Settings,
  EditContent,
  Folder,
} from '@/components/icons';
import { TeamContext } from '@/lib/contexts/team-context';
import { useTranslations } from 'next-intl';

export interface TeamSidemenuProps {
  username: string;
}

export default function TeamSidemenu({ username }: TeamSidemenuProps) {
  const t = useTranslations('Team');
  const { teams } = useContext(TeamContext);
  const team = useMemo(() => {
    return teams.find((t) => t.username === username);
  }, [teams, username]);

  if (!team) {
    return <></>;
  }

  return (
    <div className="w-64 flex flex-col max-mobile:!hidden gap-2.5">
      <TeamProfile team={team} />

      <nav className="py-5 px-3 w-full rounded-[10px] bg-component-bg">
        <Link
          href={route.teamByUsername(team.username)}
          className="sidemenu-link"
        >
          <Home />
          <span>{t('home')}</span>
        </Link>
        <Link href={route.teamDrafts(team.username)} className="sidemenu-link">
          <EditContent className="w-6 h-6 [&>path]:stroke-neutral-500" />
          <span>{t('drafts')}</span>
        </Link>
        <Link href={route.teamGroups(team.username)} className="sidemenu-link">
          <Folder className="w-6 h-6 [&>path]:stroke-neutral-500" />
          <span>{t('manage_group')}</span>
        </Link>
        <Link href={route.teamMembers(team.username)} className="sidemenu-link">
          <UserGroup className="w-6 h-6" />
          <span>{t('members')}</span>
        </Link>
        <Link
          href={route.teamSettings(team.username)}
          className="sidemenu-link"
        >
          <Settings className="w-6 h-6" />
          <span>{t('settings')}</span>
        </Link>
      </nav>

      {/* <nav className="mt-4 px-2">
        <div className="flex items-center gap-3 px-2 py-2 rounded-md hover:bg-gray-800">
          <div className="w-5 h-5 rounded-full border border-gray-500 flex items-center justify-center">
            <Users size={12} />
          </div>
          <span className="text-sm">Profile</span>
        </div>
        <div className="flex items-center gap-3 px-2 py-2 rounded-md hover:bg-gray-800">
          <div className="w-5 h-5 rounded-full border border-gray-500 flex items-center justify-center">
            <MessageSquare size={12} />
          </div>
          <span className="text-sm">Threads</span>
        </div>
        <div className="flex items-center gap-3 px-2 py-2 rounded-md hover:bg-gray-800">
          <div className="w-5 h-5 rounded-full border border-gray-500 flex items-center justify-center">
            <Users size={12} />
          </div>
          <span className="text-sm">Manage Group</span>
        </div>
        <div className="flex items-center gap-3 px-2 py-2 rounded-md hover:bg-gray-800">
          <div className="w-5 h-5 rounded-full border border-gray-500 flex items-center justify-center">
            <Users size={12} />
          </div>
          <span className="text-sm">Settings</span>
        </div>
      </nav> */}
    </div>
  );
}
