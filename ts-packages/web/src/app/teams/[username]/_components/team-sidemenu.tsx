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
import { useTranslation } from 'react-i18next';

import { useSuspenseFindTeam } from '@/features/teams/hooks/use-find-team';
import {
  TeamGroupPermission,
  TeamGroupPermissions,
} from '@/features/auth/utils/team-group-permissions';
import { Controller, Trophy } from '@/assets/icons/game';

export interface TeamSidemenuProps {
  username: string;
}

export default function TeamSidemenu({ username }: TeamSidemenuProps) {
  const { t } = useTranslation('Team');

  // Get team data using v3 API
  const { data: team } = useSuspenseFindTeam(username);

  // Get permissions from team detail response (no API calls!)
  const permissions = new TeamGroupPermissions(team.permissions);

  const writePostPermission =
    permissions?.has(TeamGroupPermission.PostWrite) ||
    permissions?.isAdmin() ||
    false;

  return (
    <div className="w-64 flex flex-col max-mobile:hidden! gap-2.5">
      {team && <TeamProfile team={team} />}

      <nav className="py-5 px-3 w-full rounded-[10px] bg-card-bg border border-card-border">
        <Link
          to={route.teamByUsername(team.username)}
          className="sidemenu-link text-text-primary [&>path]:stroke-[#737373]"
          data-pw="team-nav-home"
        >
          <Home className="w-6 h-6" />
          <span>{t('home')}</span>
        </Link>
        {writePostPermission ? (
          <Link
            to={route.teamDrafts(team.username)}
            className="sidemenu-link text-text-primary"
            data-testid="sidemenu-team-drafts"
            data-pw="team-nav-drafts"
          >
            <EditContent className="w-6 h-6 [&>path]:stroke-icon" />
            <span>{t('drafts')}</span>
          </Link>
        ) : (
          <></>
        )}
        {permissions?.has(TeamGroupPermission.TeamEdit) ||
        permissions?.isAdmin() ? (
          <Link
            to={route.teamGroups(team.username)}
            className="sidemenu-link text-text-primary "
            data-testid="sidemenu-team-groups"
            data-pw="team-nav-groups"
          >
            <Folder className="w-6 h-6 [&>path]:stroke-icon" />
            <span>{t('manage_group')}</span>
          </Link>
        ) : null}
        {permissions?.has(TeamGroupPermission.GroupEdit) ||
        permissions?.isAdmin() ? (
          <Link
            to={route.teamMembers(team.username)}
            className="sidemenu-link text-text-primary"
            data-testid="sidemenu-team-members"
            data-pw="team-nav-members"
          >
            <UserGroup className="w-6 h-6 [&>path]:stroke-icon" />
            <span>{t('members')}</span>
          </Link>
        ) : null}
        {permissions?.isAdmin() ? (
          <Link
            to={route.teamDao(team.username)}
            className="sidemenu-link text-text-primary"
            data-testid="sidemenu-team-dao"
            data-pw="team-nav-dao"
          >
            <Controller className="[&>path]:stroke-icon [&>circle]:stroke-icon w-6 h-6" />
            <span>{t('dao')}</span>
          </Link>
        ) : null}
        {permissions?.isAdmin() ? (
          <Link
            to={route.teamRewards(team.username)}
            className="sidemenu-link text-text-primary"
            data-testid="sidemenu-team-rewards"
            data-pw="team-nav-rewards"
          >
            <Trophy className="[&>path]:stroke-icon w-6 h-6" />
            <span>{t('rewards')}</span>
          </Link>
        ) : null}
        {permissions?.has(TeamGroupPermission.TeamEdit) ||
        permissions?.isAdmin() ? (
          <Link
            to={route.teamSettings(team.username)}
            className="sidemenu-link text-text-primary"
            data-testid="sidemenu-team-settings"
            data-pw="team-nav-settings"
          >
            <Settings className="w-6 h-6" />
            <span>{t('settings')}</span>
          </Link>
        ) : null}
      </nav>
    </div>
  );
}
