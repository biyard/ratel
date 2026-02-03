'use client';
import { useContext } from 'react';
import { TeamContext } from '@/lib/contexts/team-context';
import { useAuth } from '@/lib/contexts/auth-context';
import { usePopup } from '@/lib/contexts/popup-service';
import { NavLink, useLocation } from 'react-router';
import { route } from '@/route';
import TeamCreationPopup from '@/app/(social)/_popups/team-creation-popup';
import { useTranslation } from 'react-i18next';
import { useUserInfo } from '@/hooks/use-user-info';
import { useUserSidemenuI18n } from '@/features/users/components/user-sidemenu/i18n';
import {} from '@/features/teams/hooks/use-team';
import {
  TeamGroupPermission,
  TeamGroupPermissions,
} from '@/features/auth/utils/team-group-permissions';
import { useFindTeam } from '@/features/teams/hooks/use-find-team';

interface MobileSideMenuProps {
  isOpen: boolean;
  onClose: () => void;
}

export default function MobileSideMenu({
  isOpen,
  onClose,
}: MobileSideMenuProps) {
  const { t } = useTranslation('Home');
  const { t: tTeam } = useTranslation('Team');
  const userSidemenuT = useUserSidemenuI18n();
  const location = useLocation();
  const { teams, setSelectedTeam } = useContext(TeamContext);
  const userInfo = useUserInfo();
  const { logout } = useAuth();
  const popup = usePopup();

  // Detect if we're on a team page
  const teamMatch = location.pathname.match(/^\/teams\/([^/]+)/);
  const currentTeamUsername = teamMatch?.[1];

  // Get team details and permissions if on a team page
  const { data: team } = useFindTeam(currentTeamUsername || '');
  const permissions = new TeamGroupPermissions(team?.permissions || BigInt(0));

  const isOnTeamPage = !!currentTeamUsername && !!team;

  if (!isOpen) return null;

  const handleTeamSelect = (index: number) => {
    setSelectedTeam(index);
    onClose();
  };

  const handleLogout = () => {
    logout();
    userInfo.refetch();
    onClose();
  };

  // Separate main user from teams
  const mainUser = teams[0];
  const actualTeams = teams.slice(1);

  // Render user-specific menu (when on user's own pages)
  const renderUserMenu = () => (
    <>
      <NavLink
        to={route.myPosts()}
        onClick={onClose}
        className="w-full px-3 py-2.5 hover:bg-hover rounded-md text-base text-text-primary-muted"
      >
        {userSidemenuT.my_posts}
      </NavLink>

      <NavLink
        to={route.drafts()}
        onClick={onClose}
        className="w-full px-3 py-2.5 hover:bg-hover rounded-md text-base text-text-primary-muted"
      >
        {userSidemenuT.drafts}
      </NavLink>

      <NavLink
        to={route.mySpaces()}
        onClick={onClose}
        className="w-full px-3 py-2.5 hover:bg-hover rounded-md text-base text-text-primary-muted"
        data-testid="my-spaces-link"
      >
        {userSidemenuT.my_spaces}
      </NavLink>

      <NavLink
        to={route.credentials()}
        onClick={onClose}
        className="w-full px-3 py-2.5 hover:bg-hover rounded-md text-base text-text-primary-muted"
      >
        {userSidemenuT.credentials}
      </NavLink>

      <NavLink
        to={route.settings()}
        onClick={onClose}
        className="w-full px-3 py-2.5 hover:bg-hover rounded-md text-base text-text-primary-muted"
      >
        {userSidemenuT.settings}
      </NavLink>
    </>
  );

  // Render team-specific menu (when on a team page)
  const renderTeamMenu = () => {
    if (!team) return null;

    const writePostPermission =
      permissions?.has(TeamGroupPermission.PostWrite) ||
      permissions?.isAdmin() ||
      false;

    const canManageGroup =
      permissions?.has(TeamGroupPermission.TeamEdit) ||
      permissions?.isAdmin() ||
      false;

    const canEditMembers =
      permissions?.has(TeamGroupPermission.GroupEdit) ||
      permissions?.isAdmin() ||
      false;

    const canEditTeam =
      permissions?.has(TeamGroupPermission.TeamEdit) ||
      permissions?.isAdmin() ||
      false;

    return (
      <>
        <NavLink
          to={route.teamByUsername(team.username)}
          onClick={onClose}
          className="w-full px-3 py-2.5 hover:bg-hover rounded-md text-base text-text-primary-muted"
        >
          {tTeam('home')}
        </NavLink>

        {writePostPermission && (
          <NavLink
            to={route.teamDrafts(team.username)}
            onClick={onClose}
            className="w-full px-3 py-2.5 hover:bg-hover rounded-md text-base text-text-primary-muted"
          >
            {tTeam('drafts')}
          </NavLink>
        )}

        {canManageGroup && (
          <NavLink
            to={route.teamGroups(team.username)}
            onClick={onClose}
            className="w-full px-3 py-2.5 hover:bg-hover rounded-md text-base text-text-primary-muted"
          >
            {tTeam('manage_group')}
          </NavLink>
        )}

        {canEditMembers && (
          <NavLink
            to={route.teamMembers(team.username)}
            onClick={onClose}
            className="w-full px-3 py-2.5 hover:bg-hover rounded-md text-base text-text-primary-muted"
          >
            {tTeam('members')}
          </NavLink>
        )}

        {canEditTeam && (
          <NavLink
            to={route.teamSettings(team.username)}
            onClick={onClose}
            className="w-full px-3 py-2.5 hover:bg-hover rounded-md text-base text-text-primary-muted"
          >
            {tTeam('settings')}
          </NavLink>
        )}
      </>
    );
  };

  return (
    <div className="fixed top-[var(--header-height)] left-0 w-screen h-[calc(100vh-var(--header-height))] z-50 bg-bg max-tablet:block hidden">
      <div className="flex flex-col h-full w-full px-4 py-6 gap-6 overflow-y-auto">
        {/* Context-specific menu */}
        <div className="flex flex-col gap-2">
          {isOnTeamPage ? renderTeamMenu() : renderUserMenu()}
        </div>

        {/* Divider */}
        <div className="h-px bg-neutral-700 light:bg-[#e5e5e5]" />

        {/* User Profile Section */}
        <div className="flex flex-col gap-3">
          <div className="text-xs text-text-primary-muted px-2">User</div>
          <NavLink
            to={route.home()}
            className="flex items-center gap-3 px-3 py-2.5 hover:bg-hover rounded-md"
            onClick={() => handleTeamSelect(0)}
          >
            {mainUser?.profile_url ? (
              <img
                src={mainUser.profile_url}
                alt={mainUser.nickname}
                className="w-8 h-8 rounded-full object-cover object-top"
              />
            ) : (
              <div className="w-8 h-8 bg-neutral-600 rounded-full" />
            )}
            <span className="text-base text-text-primary-muted">
              {mainUser?.nickname}
            </span>
          </NavLink>
        </div>

        {/* Teams Section - only show if there are actual teams */}
        {actualTeams.length > 0 && (
          <>
            <div className="h-px bg-neutral-700 light:bg-[#e5e5e5]" />
            <div className="flex flex-col gap-3">
              <div className="text-xs text-text-primary-muted px-2">
                {t('teams')}
              </div>
              <div className="flex flex-col gap-2">
                {actualTeams.map((teamItem, index) => (
                  <NavLink
                    key={`mobile-team-${teamItem.pk}`}
                    to={route.teamByUsername(teamItem.username)}
                    className="flex items-center gap-3 px-3 py-2.5 hover:bg-hover rounded-md"
                    onClick={() => handleTeamSelect(index + 1)}
                  >
                    {teamItem.profile_url ? (
                      <img
                        src={teamItem.profile_url}
                        alt={teamItem.nickname}
                        className="w-8 h-8 rounded-full object-cover object-top"
                      />
                    ) : (
                      <div className="w-8 h-8 bg-neutral-600 rounded-full" />
                    )}
                    <span className="text-base text-text-primary-muted">
                      {teamItem.nickname}
                    </span>
                  </NavLink>
                ))}
              </div>
            </div>
          </>
        )}

        {/* Divider */}
        <div className="h-px bg-neutral-700 light:bg-[#e5e5e5]" />

        {/* General Actions */}
        <div className="flex flex-col gap-2">
          <button
            onClick={() => {
              popup.open(<TeamCreationPopup />).withTitle('Create a new team');
              onClose();
            }}
            className="w-full px-3 py-2.5 hover:bg-hover rounded-md text-left text-base text-text-primary-muted"
          >
            {t('create_team')}
          </button>

          <button
            onClick={handleLogout}
            className="w-full px-3 py-2.5 hover:bg-hover rounded-md text-left text-base text-text-primary-muted"
          >
            {t('logout')}
          </button>
        </div>
      </div>
    </div>
  );
}
