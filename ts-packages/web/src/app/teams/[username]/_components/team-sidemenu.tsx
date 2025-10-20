'use client';

import { useContext, useMemo } from 'react';
// import { Users, MessageSquare } from 'lucide-react';
// import type { Team } from '@/lib/api/models/team';
import TeamProfile from './team-profile';
import { Link } from 'react-router';
import { route } from '@/route';
import {
  Home,
  UserGroup,
  Settings,
  EditContent,
  Folder,
} from '@/components/icons';
import { TeamContext } from '@/lib/contexts/team-context';
import { useTranslation } from 'react-i18next';

import {
  useTeamDetailByUsername,
  useTeamPermissionsFromDetail,
} from '@/features/teams/hooks/use-team';
import { TeamGroupPermission } from '@/features/auth/utils/team-group-permissions';

export interface TeamSidemenuProps {
  username: string;
}

export default function TeamSidemenu({ username }: TeamSidemenuProps) {
  const { t } = useTranslation('Team');
  const { teams } = useContext(TeamContext);
  const team = useMemo(() => {
    return teams.find((t) => t.username === username);
  }, [teams, username]);

  // Get team data using v3 API
  const teamDetailQuery = useTeamDetailByUsername(username);

  // Get permissions from team detail response (no API calls!)
  const permissions = useTeamPermissionsFromDetail(teamDetailQuery.data);

  const writePostPermission =
    permissions?.has(TeamGroupPermission.PostWrite) ||
    permissions?.isAdmin() ||
    false;

  // Use v3 team data if available, otherwise fall back to context team
  const displayTeam = teamDetailQuery.data || team;

  if (teamDetailQuery.isLoading) {
    return <div className="flex justify-center p-4">Loading...</div>;
  }

  if (!displayTeam) {
    return <></>;
  }

  // If no team data is available, show loading or error state
  if (teamDetailQuery.isLoading) {
    return <div>Loading team...</div>;
  }

  if (teamDetailQuery.isError || !displayTeam) {
    return <div>Team not found</div>;
  }

  return (
    <div className="w-64 flex flex-col max-mobile:!hidden gap-2.5">
      {team && <TeamProfile team={team} />}

      <nav className="py-5 px-3 w-full rounded-[10px] bg-card-bg border border-card-border">
        <Link
          to={route.teamByUsername(displayTeam.username)}
          className="sidemenu-link text-text-primary [&>path]:stroke-[#737373]"
          data-pw="team-nav-home"
        >
          <Home className="w-6 h-6" />
          <span>{t('home')}</span>
        </Link>
        {writePostPermission ? (
          <Link
            to={route.teamDrafts(displayTeam.username)}
            className="sidemenu-link text-text-primary"
            data-pw="team-nav-drafts"
          >
            <EditContent className="w-6 h-6 [&>path]:stroke-[#737373]" />
            <span>{t('drafts')}</span>
          </Link>
        ) : (
          <></>
        )}
        {permissions?.has(TeamGroupPermission.TeamEdit) ||
        permissions?.isAdmin() ? (
          <Link
            to={route.teamGroups(displayTeam.username)}
            className="sidemenu-link text-text-primary "
            data-pw="team-nav-groups"
          >
            <Folder className="w-6 h-6 [&>path]:stroke-[#737373]" />
            <span>{t('manage_group')}</span>
          </Link>
        ) : null}
        {permissions?.has(TeamGroupPermission.GroupEdit) ||
        permissions?.isAdmin() ? (
          <Link
            to={route.teamMembers(displayTeam.username)}
            className="sidemenu-link text-text-primary"
            data-pw="team-nav-members"
          >
            <UserGroup className="w-6 h-6 [&>path]:stroke-[#737373]" />
            <span>{t('members')}</span>
          </Link>
        ) : null}
        {permissions?.has(TeamGroupPermission.TeamEdit) ||
        permissions?.isAdmin() ? (
          <Link
            to={route.teamSettings(displayTeam.username)}
            className="sidemenu-link text-text-primary"
            data-pw="team-nav-settings"
          >
            <Settings className="w-6 h-6" />
            <span>{t('settings')}</span>
          </Link>
        ) : null}
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
